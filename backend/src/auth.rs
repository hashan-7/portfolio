use std::{
    env, fmt,
    sync::{
        OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use argon2::{
    Argon2, Params,
    password_hash::{PasswordHash, PasswordVerifier},
};
use axum::{extract::Request, http::header, response::Response};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

const DEFAULT_SESSION_SECONDS: u64 = 60 * 60;
const MIN_SESSION_SECONDS: u64 = 5 * 60;
const MAX_SESSION_SECONDS: u64 = 60 * 60;
const TOKEN_CLOCK_GRACE_SECONDS: u64 = 30;
const MIN_PASSWORD_BYTES: usize = 12;
const MAX_CREDENTIAL_BYTES: usize = 1_024;
const MIN_SECRET_BYTES: usize = 32;
const MAX_SECRET_BYTES: usize = 4_096;
const MIN_ARGON2_MEMORY_KIB: u32 = 19 * 1024;
const MAX_ARGON2_MEMORY_KIB: u32 = 64 * 1024;
const MIN_ARGON2_ITERATIONS: u32 = 2;
const MAX_ARGON2_ITERATIONS: u32 = 4;
const MAX_ARGON2_PARALLELISM: u32 = 4;
const MIN_ARGON2_OUTPUT_BYTES: usize = 16;
const MAX_ARGON2_OUTPUT_BYTES: usize = 64;
const MAX_ARGON2_WORK_KIB: u64 = 256 * 1024;
const DEFAULT_ISSUER: &str = "portfolio-backend";
const DEFAULT_AUDIENCE: &str = "portfolio-admin";
const DEFAULT_SESSION_VERSION: &str = "1";
const ADMIN_TOKEN_TYPE: &str = "admin";

static AUTH_CONFIG: OnceLock<AuthConfig> = OnceLock::new();
static TOKEN_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub struct AuthConfig {
    admin_email: String,
    admin_password: AdminPassword,
    session_secret: String,
    issuer: String,
    audience: String,
    session_version: String,
    session_seconds: u64,
}

enum AdminPassword {
    Argon2id(String),
    Plaintext(String),
}

impl AuthConfig {
    pub fn from_env() -> Result<Self, AuthConfigError> {
        let admin_email = required_env("ADMIN_EMAIL")?;
        let admin_password = match env::var("ADMIN_PASSWORD_HASH") {
            Ok(value) if !value.trim().is_empty() => AdminPassword::from_argon2id(value)?,
            _ => AdminPassword::from_plaintext(required_env("ADMIN_PASSWORD")?, &admin_email)?,
        };
        let session_secret = required_env("ADMIN_SESSION_SECRET")?;
        let issuer =
            env::var("ADMIN_SESSION_ISSUER").unwrap_or_else(|_| DEFAULT_ISSUER.to_string());
        let audience =
            env::var("ADMIN_SESSION_AUDIENCE").unwrap_or_else(|_| DEFAULT_AUDIENCE.to_string());
        let session_version = env::var("ADMIN_SESSION_VERSION")
            .unwrap_or_else(|_| DEFAULT_SESSION_VERSION.to_string());
        let session_seconds = env::var("ADMIN_SESSION_SECONDS")
            .ok()
            .map(|value| {
                value.parse::<u64>().map_err(|_| {
                    AuthConfigError::new(
                        "ADMIN_SESSION_SECONDS must be an integer between 300 and 3600.",
                    )
                })
            })
            .transpose()?
            .unwrap_or(DEFAULT_SESSION_SECONDS);

        Self::new_with_password(
            admin_email,
            admin_password,
            session_secret,
            issuer,
            audience,
            session_version,
            session_seconds,
        )
    }

    #[cfg(test)]
    fn new(
        admin_email: String,
        admin_password: String,
        session_secret: String,
        issuer: String,
        audience: String,
        session_version: String,
        session_seconds: u64,
    ) -> Result<Self, AuthConfigError> {
        let admin_password = AdminPassword::from_plaintext(admin_password, &admin_email)?;
        Self::new_with_password(
            admin_email,
            admin_password,
            session_secret,
            issuer,
            audience,
            session_version,
            session_seconds,
        )
    }

    fn new_with_password(
        admin_email: String,
        admin_password: AdminPassword,
        session_secret: String,
        issuer: String,
        audience: String,
        session_version: String,
        session_seconds: u64,
    ) -> Result<Self, AuthConfigError> {
        let admin_email = admin_email.trim().to_string();

        if admin_email.is_empty()
            || admin_email.len() > 320
            || contains_ascii_control(&admin_email)
            || !admin_email.contains('@')
        {
            return Err(AuthConfigError::new(
                "ADMIN_EMAIL must be a valid non-empty email address of at most 320 bytes.",
            ));
        }

        if session_secret.len() < MIN_SECRET_BYTES
            || session_secret.len() > MAX_SECRET_BYTES
            || contains_ascii_control(&session_secret)
            || is_common_secret(&session_secret)
        {
            return Err(AuthConfigError::new(
                "ADMIN_SESSION_SECRET must be a unique 32-4096 byte value with no control characters.",
            ));
        }

        validate_claim_config("ADMIN_SESSION_ISSUER", &issuer)?;
        validate_claim_config("ADMIN_SESSION_AUDIENCE", &audience)?;
        validate_claim_config("ADMIN_SESSION_VERSION", &session_version)?;

        if !(MIN_SESSION_SECONDS..=MAX_SESSION_SECONDS).contains(&session_seconds) {
            return Err(AuthConfigError::new(
                "ADMIN_SESSION_SECONDS must be between 300 and 3600.",
            ));
        }

        Ok(Self {
            admin_email,
            admin_password,
            session_secret,
            issuer,
            audience,
            session_version,
            session_seconds,
        })
    }

    fn credentials_match(&self, email: &str, password: &str) -> bool {
        let normalized_email = email.trim();
        constant_time_eq(normalized_email.as_bytes(), self.admin_email.as_bytes())
            & self.admin_password.verify(password)
    }
}

impl AdminPassword {
    fn from_plaintext(value: String, email: &str) -> Result<Self, AuthConfigError> {
        if value.len() < MIN_PASSWORD_BYTES
            || value.len() > MAX_CREDENTIAL_BYTES
            || contains_ascii_control(&value)
            || weak_password(&value, email)
        {
            return Err(AuthConfigError::new(
                "ADMIN_PASSWORD must be 12-1024 bytes, contain no control characters, and not be a common or email-derived password.",
            ));
        }

        Ok(Self::Plaintext(value))
    }

    fn from_argon2id(value: String) -> Result<Self, AuthConfigError> {
        if value.len() > MAX_CREDENTIAL_BYTES || contains_ascii_control(&value) {
            return Err(AuthConfigError::new(
                "ADMIN_PASSWORD_HASH must be a valid Argon2id PHC string of at most 1024 bytes.",
            ));
        }

        let parsed = PasswordHash::new(&value).map_err(|_| {
            AuthConfigError::new(
                "ADMIN_PASSWORD_HASH must be a valid Argon2id PHC string of at most 1024 bytes.",
            )
        })?;
        if parsed.algorithm.as_str() != "argon2id" {
            return Err(AuthConfigError::new(
                "ADMIN_PASSWORD_HASH must use the Argon2id algorithm.",
            ));
        }

        let params = Params::try_from(&parsed).map_err(|_| {
            AuthConfigError::new(
                "ADMIN_PASSWORD_HASH must contain safe, valid Argon2id parameters.",
            )
        })?;
        let output_len = params.output_len().unwrap_or(Params::DEFAULT_OUTPUT_LEN);
        let work_kib = u64::from(params.m_cost()) * u64::from(params.t_cost());
        if parsed.version != Some(19)
            || parsed.salt.is_none()
            || parsed.hash.is_none()
            || !(MIN_ARGON2_MEMORY_KIB..=MAX_ARGON2_MEMORY_KIB).contains(&params.m_cost())
            || !(MIN_ARGON2_ITERATIONS..=MAX_ARGON2_ITERATIONS).contains(&params.t_cost())
            || !(1..=MAX_ARGON2_PARALLELISM).contains(&params.p_cost())
            || !(MIN_ARGON2_OUTPUT_BYTES..=MAX_ARGON2_OUTPUT_BYTES).contains(&output_len)
            || work_kib > MAX_ARGON2_WORK_KIB
        {
            return Err(AuthConfigError::new(
                "ADMIN_PASSWORD_HASH must use Argon2id v19 with bounded production-safe parameters.",
            ));
        }

        Ok(Self::Argon2id(value))
    }

    fn verify(&self, candidate: &str) -> bool {
        if candidate.len() > MAX_CREDENTIAL_BYTES {
            return false;
        }

        match self {
            Self::Argon2id(encoded) => PasswordHash::new(encoded).ok().is_some_and(|hash| {
                Argon2::default()
                    .verify_password(candidate.as_bytes(), &hash)
                    .is_ok()
            }),
            Self::Plaintext(expected) => {
                constant_time_eq(candidate.as_bytes(), expected.as_bytes())
            }
        }
    }
}

#[derive(Debug)]
pub struct AuthConfigError {
    message: String,
}

impl AuthConfigError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for AuthConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AuthConfigError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub nbf: usize,
    pub jti: String,
    #[serde(rename = "typ")]
    pub token_type: String,
    pub ver: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: usize,
    pub expires_in_seconds: usize,
}

#[derive(Debug, Serialize)]
pub struct AuthError {
    pub error: String,
}

impl AuthError {
    pub fn is_invalid(&self) -> bool {
        self.error == "Invalid or expired credentials."
    }

    fn invalid() -> Self {
        Self {
            error: "Invalid or expired credentials.".to_string(),
        }
    }

    fn unavailable() -> Self {
        Self {
            error: "Authentication service is unavailable.".to_string(),
        }
    }
}

pub fn initialize_from_env() -> Result<&'static AuthConfig, AuthConfigError> {
    if let Some(config) = AUTH_CONFIG.get() {
        return Ok(config);
    }

    let config = AuthConfig::from_env()?;
    AUTH_CONFIG
        .set(config)
        .map_err(|_| AuthConfigError::new("Authentication configuration initialization failed."))?;

    AUTH_CONFIG
        .get()
        .ok_or_else(|| AuthConfigError::new("Authentication configuration initialization failed."))
}

pub fn authenticate_admin(email: &str, password: &str) -> Result<AuthResponse, AuthError> {
    let config = active_config()?;

    if !config.credentials_match(email, password) {
        return Err(AuthError::invalid());
    }

    generate_token_with_config(config)
}

pub fn verify_token(token: &str) -> Result<Claims, AuthError> {
    verify_token_with_config(token, active_config()?)
}

pub async fn auth_middleware(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, ApiError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| !value.is_empty())
        .ok_or_else(ApiError::unauthorized)?;

    verify_token(token).map_err(|_| ApiError::unauthorized())?;

    Ok(next.run(req).await)
}

fn active_config() -> Result<&'static AuthConfig, AuthError> {
    AUTH_CONFIG.get().ok_or_else(AuthError::unavailable)
}

fn generate_token_with_config(config: &AuthConfig) -> Result<AuthResponse, AuthError> {
    let issued_at = unix_timestamp().ok_or_else(AuthError::unavailable)?;
    let expiration = issued_at
        .checked_add(config.session_seconds)
        .ok_or_else(AuthError::unavailable)?;
    let issued_at = usize::try_from(issued_at).map_err(|_| AuthError::unavailable())?;
    let expiration = usize::try_from(expiration).map_err(|_| AuthError::unavailable())?;

    let claims = Claims {
        sub: config.admin_email.clone(),
        iss: config.issuer.clone(),
        aud: config.audience.clone(),
        exp: expiration,
        iat: issued_at,
        nbf: issued_at,
        jti: next_token_id(),
        token_type: ADMIN_TOKEN_TYPE.to_string(),
        ver: config.session_version.clone(),
    };
    let header = Header {
        alg: Algorithm::HS256,
        typ: Some("JWT".to_string()),
        ..Header::default()
    };
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.session_secret.as_bytes()),
    )
    .map_err(|_| AuthError::unavailable())?;

    Ok(AuthResponse {
        token,
        expires_at: expiration,
        expires_in_seconds: config.session_seconds as usize,
    })
}

fn verify_token_with_config(token: &str, config: &AuthConfig) -> Result<Claims, AuthError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = TOKEN_CLOCK_GRACE_SECONDS;
    validation.validate_exp = true;
    validation.validate_nbf = true;
    validation.set_required_spec_claims(&["exp", "nbf", "iss", "aud", "sub"]);
    validation.set_issuer(&[&config.issuer]);
    validation.set_audience(&[&config.audience]);
    validation.sub = Some(config.admin_email.clone());

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.session_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| AuthError::invalid())?;

    if data.header.alg != Algorithm::HS256 || data.header.typ.as_deref() != Some("JWT") {
        return Err(AuthError::invalid());
    }

    let claims = data.claims;
    let now = unix_timestamp().ok_or_else(AuthError::unavailable)?;
    let issued_at = u64::try_from(claims.iat).map_err(|_| AuthError::invalid())?;
    let not_before = u64::try_from(claims.nbf).map_err(|_| AuthError::invalid())?;
    let expiration = u64::try_from(claims.exp).map_err(|_| AuthError::invalid())?;
    let latest_allowed_issue = now.saturating_add(TOKEN_CLOCK_GRACE_SECONDS);
    let latest_allowed_expiration = issued_at
        .saturating_add(config.session_seconds)
        .saturating_add(TOKEN_CLOCK_GRACE_SECONDS);

    if claims.token_type != ADMIN_TOKEN_TYPE
        || claims.ver != config.session_version
        || claims.jti.is_empty()
        || claims.jti.len() > 128
        || issued_at > latest_allowed_issue
        || not_before > issued_at.saturating_add(TOKEN_CLOCK_GRACE_SECONDS)
        || expiration <= issued_at
        || expiration > latest_allowed_expiration
    {
        return Err(AuthError::invalid());
    }

    Ok(claims)
}

fn required_env(name: &str) -> Result<String, AuthConfigError> {
    env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| AuthConfigError::new(format!("{name} is required.")))
}

fn validate_claim_config(name: &str, value: &str) -> Result<(), AuthConfigError> {
    if value.is_empty() || value.len() > 128 || contains_ascii_control(value) {
        return Err(AuthConfigError::new(format!(
            "{name} must be 1-128 bytes and contain no control characters."
        )));
    }

    Ok(())
}

fn contains_ascii_control(value: &str) -> bool {
    value.bytes().any(|byte| byte.is_ascii_control())
}

fn weak_password(password: &str, email: &str) -> bool {
    let normalized = password.to_ascii_lowercase();
    let local_part = email.split('@').next().unwrap_or("").to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "password1234"
            | "password12345"
            | "administrator"
            | "adminpassword"
            | "change-this-admin-password"
            | "changeme1234"
            | "letmein12345"
    ) || (!local_part.is_empty() && normalized.contains(&local_part))
}

fn is_common_secret(secret: &str) -> bool {
    matches!(
        secret.to_ascii_lowercase().as_str(),
        "change-this-to-a-long-random-secret"
            | "change-this-to-a-unique-random-secret-of-32-or-more-characters"
            | "minimum_32_characters_long_secret_value"
            | "replace-with-a-long-random-secret"
            | "your-super-secret-key-change-this"
    )
}

fn constant_time_eq(candidate: &[u8], expected: &[u8]) -> bool {
    if candidate.len() > MAX_CREDENTIAL_BYTES || expected.len() > MAX_CREDENTIAL_BYTES {
        return false;
    }

    let mut difference = candidate.len() ^ expected.len();

    for index in 0..MAX_CREDENTIAL_BYTES {
        let candidate_byte = candidate.get(index).copied().unwrap_or(0);
        let expected_byte = expected.get(index).copied().unwrap_or(0);
        difference |= usize::from(candidate_byte ^ expected_byte);
    }

    std::hint::black_box(difference) == 0
}

fn unix_timestamp() -> Option<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}

fn next_token_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    let sequence = TOKEN_SEQUENCE.fetch_add(1, Ordering::Relaxed);

    format!("{:x}-{:x}-{:x}", std::process::id(), timestamp, sequence)
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::{PasswordHasher, SaltString};

    fn test_config() -> AuthConfig {
        AuthConfig::new(
            "admin@example.com".to_string(),
            "a-strong-test-password".to_string(),
            "a-test-secret-that-is-at-least-thirty-two-bytes-long".to_string(),
            "test-issuer".to_string(),
            "test-audience".to_string(),
            "42".to_string(),
            600,
        )
        .expect("test authentication configuration must be valid")
    }

    #[test]
    fn credentials_require_both_exact_values() {
        let config = test_config();

        assert!(config.credentials_match(" admin@example.com ", "a-strong-test-password"));
        assert!(!config.credentials_match("other@example.com", "a-strong-test-password"));
        assert!(!config.credentials_match("admin@example.com", "wrong-password"));
    }

    #[test]
    fn rejects_weak_configuration() {
        let result = AuthConfig::new(
            "admin@example.com".to_string(),
            "password1234".to_string(),
            "a-test-secret-that-is-at-least-thirty-two-bytes-long".to_string(),
            "issuer".to_string(),
            "audience".to_string(),
            "1".to_string(),
            600,
        );

        assert!(result.is_err());
    }

    #[test]
    fn verifies_argon2id_password_hashes() {
        let salt = SaltString::encode_b64(b"fixed-test-salt!").expect("salt must be valid");
        let encoded = Argon2::default()
            .hash_password(b"correct horse battery staple", &salt)
            .expect("password should hash")
            .to_string();
        let password = AdminPassword::from_argon2id(encoded).expect("hash should be accepted");

        assert!(password.verify("correct horse battery staple"));
        assert!(!password.verify("wrong password"));
    }

    #[test]
    fn rejects_argon2id_hashes_with_excessive_resource_costs() {
        let salt = SaltString::encode_b64(b"fixed-test-salt!").expect("salt must be valid");
        let encoded = Argon2::default()
            .hash_password(b"correct horse battery staple", &salt)
            .expect("password should hash")
            .to_string()
            .replacen("m=19456", "m=65537", 1);

        assert!(AdminPassword::from_argon2id(encoded).is_err());
    }

    #[test]
    fn generated_token_has_bound_admin_claims() {
        let config = test_config();
        let response = generate_token_with_config(&config).expect("token must be generated");
        let claims = verify_token_with_config(&response.token, &config)
            .expect("generated token must verify");

        assert_eq!(claims.sub, "admin@example.com");
        assert_eq!(claims.iss, "test-issuer");
        assert_eq!(claims.aud, "test-audience");
        assert_eq!(claims.ver, "42");
        assert_eq!(claims.token_type, ADMIN_TOKEN_TYPE);
        assert!(!claims.jti.is_empty());
    }

    #[test]
    fn token_is_rejected_after_session_version_change() {
        let config = test_config();
        let response = generate_token_with_config(&config).expect("token must be generated");
        let mut changed_config = test_config();
        changed_config.session_version = "43".to_string();

        assert!(verify_token_with_config(&response.token, &changed_config).is_err());
    }

    #[test]
    fn token_is_rejected_for_another_admin_subject() {
        let config = test_config();
        let response = generate_token_with_config(&config).expect("token must be generated");
        let mut changed_config = test_config();
        changed_config.admin_email = "replacement@example.com".to_string();

        assert!(verify_token_with_config(&response.token, &changed_config).is_err());
    }

    #[test]
    fn token_is_rejected_for_another_issuer_or_audience() {
        let config = test_config();
        let response = generate_token_with_config(&config).expect("token must be generated");
        let mut changed_config = test_config();
        changed_config.issuer = "another-issuer".to_string();
        assert!(verify_token_with_config(&response.token, &changed_config).is_err());

        let mut changed_config = test_config();
        changed_config.audience = "another-audience".to_string();
        assert!(verify_token_with_config(&response.token, &changed_config).is_err());
    }
}
