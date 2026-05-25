use axum::{
    Json,
    extract::Request,
    http::{StatusCode, header},
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::env;

const ADMIN_SESSION_SECONDS: i64 = 60 * 60;
const TOKEN_CLOCK_GRACE_SECONDS: usize = 60;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: usize,
    pub expires_in_seconds: usize,
}

#[derive(Serialize)]
pub struct AuthError {
    pub error: String,
}

fn session_secret() -> Result<String, AuthError> {
    env::var("ADMIN_SESSION_SECRET")
        .ok()
        .filter(|secret| secret.len() >= 32)
        .ok_or_else(|| AuthError {
            error: "ADMIN_SESSION_SECRET is missing or too short.".to_string(),
        })
}

pub fn generate_token(email: &str) -> Result<AuthResponse, AuthError> {
    let secret = session_secret()?;

    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(ADMIN_SESSION_SECONDS))
        .ok_or_else(|| AuthError {
            error: "Failed to create token expiration.".to_string(),
        })?
        .timestamp() as usize;

    let claims = Claims {
        sub: email.to_owned(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError {
        error: "Failed to generate token.".to_string(),
    })?;

    Ok(AuthResponse {
        token,
        expires_at: expiration,
        expires_in_seconds: ADMIN_SESSION_SECONDS as usize,
    })
}

pub fn verify_token(token: &str) -> Result<Claims, AuthError> {
    let secret = session_secret()?;

    let claims = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AuthError {
        error: "Invalid or expired token.".to_string(),
    })?;

    let now = Utc::now().timestamp() as usize;
    let maximum_allowed_exp = now
        .saturating_add(ADMIN_SESSION_SECONDS as usize)
        .saturating_add(TOKEN_CLOCK_GRACE_SECONDS);

    if claims.exp > maximum_allowed_exp {
        return Err(AuthError {
            error: "Invalid or expired token.".to_string(),
        });
    }

    Ok(claims)
}

pub async fn auth_middleware(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, (StatusCode, Json<AuthError>)> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(AuthError {
                    error: "Missing authorization header.".to_string(),
                }),
            )
        })?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(AuthError {
                error: "Invalid authorization format.".to_string(),
            }),
        )
    })?;

    verify_token(token).map_err(|error| {
        let status = if error.error.contains("ADMIN_SESSION_SECRET") {
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            StatusCode::UNAUTHORIZED
        };

        (status, Json(error))
    })?;

    Ok(next.run(req).await)
}
