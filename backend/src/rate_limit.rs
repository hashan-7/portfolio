use std::{
    collections::HashMap,
    env, fmt,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

use axum::{
    extract::{ConnectInfo, Request},
    http::{HeaderValue, header},
    response::{IntoResponse, Response},
};

use crate::error::ApiError;

const MAX_TRACKED_CLIENTS: usize = 4_096;
const CLIENT_ENTRY_TTL: Duration = Duration::from_secs(10 * 60);
const CLEANUP_INTERVAL: Duration = Duration::from_secs(60);
const UNKNOWN_CLIENT_IP: IpAddr = IpAddr::V6(Ipv6Addr::UNSPECIFIED);

const CHAT_RULE: RateLimitRule = RateLimitRule {
    per_client: Quota {
        max_requests: 20,
        window: Duration::from_secs(60),
    },
    global: Quota {
        max_requests: 300,
        window: Duration::from_secs(60),
    },
};

const ADMIN_LOGIN_RULE: RateLimitRule = RateLimitRule {
    per_client: Quota {
        max_requests: 5,
        window: Duration::from_secs(5 * 60),
    },
    global: Quota {
        max_requests: 100,
        window: Duration::from_secs(5 * 60),
    },
};

const ADMIN_RULE: RateLimitRule = RateLimitRule {
    per_client: Quota {
        max_requests: 60,
        window: Duration::from_secs(60),
    },
    global: Quota {
        max_requests: 600,
        window: Duration::from_secs(60),
    },
};

static RATE_LIMIT_CONFIG: OnceLock<RateLimitConfig> = OnceLock::new();
static CHAT_LIMITER: OnceLock<RateLimiter> = OnceLock::new();
static ADMIN_LOGIN_LIMITER: OnceLock<RateLimiter> = OnceLock::new();
static ADMIN_LIMITER: OnceLock<RateLimiter> = OnceLock::new();

#[derive(Clone, Copy, Debug)]
pub struct RateLimitConfig {
    pub trust_proxy_headers: bool,
    trusted_proxy_header: TrustedProxyHeader,
}

impl RateLimitConfig {
    pub fn from_env() -> Result<Self, RateLimitConfigError> {
        let trust_proxy_headers = env::var("TRUST_PROXY_HEADERS").ok().is_some_and(|value| {
            matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true")
        });
        let trusted_proxy_header = env::var("TRUSTED_PROXY_HEADER")
            .unwrap_or_else(|_| "x-forwarded-for".to_string())
            .parse()?;

        Ok(Self {
            trust_proxy_headers,
            trusted_proxy_header,
        })
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            trust_proxy_headers: false,
            trusted_proxy_header: TrustedProxyHeader::ForwardedFor,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TrustedProxyHeader {
    ForwardedFor,
    Cloudflare,
    RealIp,
}

impl std::str::FromStr for TrustedProxyHeader {
    type Err = RateLimitConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "x-forwarded-for" => Ok(Self::ForwardedFor),
            "cf-connecting-ip" => Ok(Self::Cloudflare),
            "x-real-ip" => Ok(Self::RealIp),
            _ => Err(RateLimitConfigError),
        }
    }
}

#[derive(Debug)]
pub struct RateLimitConfigError;

impl fmt::Display for RateLimitConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "TRUSTED_PROXY_HEADER must be x-forwarded-for, cf-connecting-ip, or x-real-ip",
        )
    }
}

impl std::error::Error for RateLimitConfigError {}

#[derive(Clone, Copy)]
struct Quota {
    max_requests: u32,
    window: Duration,
}

#[derive(Clone, Copy)]
struct RateLimitRule {
    per_client: Quota,
    global: Quota,
}

#[derive(Clone, Copy)]
struct WindowBucket {
    started_at: Instant,
    last_seen: Instant,
    requests: u32,
}

impl WindowBucket {
    fn new(now: Instant) -> Self {
        Self {
            started_at: now,
            last_seen: now,
            requests: 0,
        }
    }

    fn refresh(&mut self, now: Instant, window: Duration) {
        if now.saturating_duration_since(self.started_at) >= window {
            self.started_at = now;
            self.requests = 0;
        }

        self.last_seen = now;
    }

    fn retry_after_seconds(&self, now: Instant, window: Duration) -> u64 {
        let remaining = window.saturating_sub(now.saturating_duration_since(self.started_at));
        remaining
            .as_secs()
            .saturating_add(u64::from(remaining.subsec_nanos() > 0))
            .max(1)
    }
}

struct LimiterState {
    clients: HashMap<IpAddr, WindowBucket>,
    global: WindowBucket,
    last_cleanup: Instant,
}

struct RateLimiter {
    state: Mutex<LimiterState>,
    max_clients: usize,
    entry_ttl: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RateLimitDecision {
    allowed: bool,
    retry_after_seconds: u64,
}

pub fn initialize_from_env() -> Result<&'static RateLimitConfig, RateLimitConfigError> {
    if let Some(config) = RATE_LIMIT_CONFIG.get() {
        return Ok(config);
    }

    let config = RateLimitConfig::from_env()?;
    let _ = RATE_LIMIT_CONFIG.set(config);
    RATE_LIMIT_CONFIG.get().ok_or(RateLimitConfigError)
}

pub async fn chat_rate_limit(req: Request, next: axum::middleware::Next) -> Response {
    apply_rate_limit(
        req,
        next,
        CHAT_LIMITER.get_or_init(default_rate_limiter),
        CHAT_RULE,
    )
    .await
}

pub async fn admin_login_rate_limit(req: Request, next: axum::middleware::Next) -> Response {
    apply_rate_limit(
        req,
        next,
        ADMIN_LOGIN_LIMITER.get_or_init(default_rate_limiter),
        ADMIN_LOGIN_RULE,
    )
    .await
}

pub async fn admin_rate_limit(req: Request, next: axum::middleware::Next) -> Response {
    apply_rate_limit(
        req,
        next,
        ADMIN_LIMITER.get_or_init(default_rate_limiter),
        ADMIN_RULE,
    )
    .await
}

async fn apply_rate_limit(
    req: Request,
    next: axum::middleware::Next,
    limiter: &'static RateLimiter,
    rule: RateLimitRule,
) -> Response {
    let client_ip = client_ip(&req);
    let decision = limiter.check(client_ip, rule);

    if decision.allowed {
        return next.run(req).await;
    }

    rate_limited_response(decision.retry_after_seconds)
}

fn rate_limited_response(retry_after_seconds: u64) -> Response {
    let mut response = ApiError::too_many_requests().into_response();

    if let Ok(value) = HeaderValue::from_str(&retry_after_seconds.to_string()) {
        response.headers_mut().insert(header::RETRY_AFTER, value);
    }
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));

    response
}

impl RateLimiter {
    fn new(max_clients: usize, entry_ttl: Duration) -> Self {
        let now = Instant::now();

        Self {
            state: Mutex::new(LimiterState {
                clients: HashMap::new(),
                global: WindowBucket::new(now),
                last_cleanup: now,
            }),
            max_clients: max_clients.max(1),
            entry_ttl,
        }
    }

    fn check(&self, client_ip: IpAddr, rule: RateLimitRule) -> RateLimitDecision {
        self.check_at(client_ip, rule, Instant::now())
    }

    fn check_at(&self, client_ip: IpAddr, rule: RateLimitRule, now: Instant) -> RateLimitDecision {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if now.saturating_duration_since(state.last_cleanup) >= CLEANUP_INTERVAL {
            state.clients.retain(|_, bucket| {
                now.saturating_duration_since(bucket.last_seen) < self.entry_ttl
            });
            state.last_cleanup = now;
        }

        state.global.refresh(now, rule.global.window);

        if state.global.requests >= rule.global.max_requests {
            return RateLimitDecision {
                allowed: false,
                retry_after_seconds: state.global.retry_after_seconds(now, rule.global.window),
            };
        }

        if !state.clients.contains_key(&client_ip)
            && state.clients.len() >= self.max_clients
            && let Some(oldest_ip) = state
                .clients
                .iter()
                .min_by_key(|(_, bucket)| bucket.last_seen)
                .map(|(ip, _)| *ip)
        {
            state.clients.remove(&oldest_ip);
        }

        let client_bucket = state
            .clients
            .entry(client_ip)
            .or_insert_with(|| WindowBucket::new(now));
        client_bucket.refresh(now, rule.per_client.window);

        if client_bucket.requests >= rule.per_client.max_requests {
            return RateLimitDecision {
                allowed: false,
                retry_after_seconds: client_bucket.retry_after_seconds(now, rule.per_client.window),
            };
        }

        client_bucket.requests = client_bucket.requests.saturating_add(1);
        state.global.requests = state.global.requests.saturating_add(1);

        RateLimitDecision {
            allowed: true,
            retry_after_seconds: 0,
        }
    }
}

fn default_rate_limiter() -> RateLimiter {
    RateLimiter::new(MAX_TRACKED_CLIENTS, CLIENT_ENTRY_TTL)
}

fn client_ip(req: &Request) -> IpAddr {
    let config = RATE_LIMIT_CONFIG.get_or_init(RateLimitConfig::default);
    let trusted_header = config
        .trust_proxy_headers
        .then_some(config.trusted_proxy_header);
    client_ip_with_proxy_trust(req, trusted_header).unwrap_or(UNKNOWN_CLIENT_IP)
}

fn client_ip_with_proxy_trust(
    req: &Request,
    trusted_header: Option<TrustedProxyHeader>,
) -> Option<IpAddr> {
    let proxy_ip = match trusted_header {
        Some(TrustedProxyHeader::ForwardedFor) => header_value(req, "x-forwarded-for")
            .and_then(|value| value.rsplit(',').next())
            .map(str::trim)
            .and_then(parse_ip),
        Some(TrustedProxyHeader::Cloudflare) => parse_ip_header(req, "cf-connecting-ip"),
        Some(TrustedProxyHeader::RealIp) => parse_ip_header(req, "x-real-ip"),
        None => None,
    };

    if proxy_ip.is_some() {
        return proxy_ip;
    }

    req.extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(address)| normalize_ip(address.ip()))
}

fn parse_ip_header(req: &Request, name: &'static str) -> Option<IpAddr> {
    header_value(req, name).and_then(parse_ip)
}

fn header_value<'a>(req: &'a Request, name: &'static str) -> Option<&'a str> {
    req.headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn parse_ip(value: &str) -> Option<IpAddr> {
    value.parse::<IpAddr>().ok().map(normalize_ip)
}

fn normalize_ip(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(ipv6) => ipv6
            .to_ipv4_mapped()
            .map(IpAddr::V4)
            .unwrap_or(IpAddr::V6(ipv6)),
        IpAddr::V4(_) => ip,
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use http_body_util::BodyExt;

    use super::*;

    fn request_with_peer(peer: &str) -> Request {
        let mut request = Request::builder()
            .body(Body::empty())
            .expect("request must build");
        let address = peer.parse::<SocketAddr>().expect("peer must be valid");
        request.extensions_mut().insert(ConnectInfo(address));
        request
    }

    fn test_rule(per_client: u32, global: u32) -> RateLimitRule {
        RateLimitRule {
            per_client: Quota {
                max_requests: per_client,
                window: Duration::from_secs(60),
            },
            global: Quota {
                max_requests: global,
                window: Duration::from_secs(60),
            },
        }
    }

    #[test]
    fn ignores_spoofed_proxy_headers_by_default() {
        let mut request = request_with_peer("192.0.2.10:4200");
        request
            .headers_mut()
            .insert("cf-connecting-ip", HeaderValue::from_static("198.51.100.2"));
        request.headers_mut().insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.3, 203.0.113.4"),
        );

        assert_eq!(
            client_ip_with_proxy_trust(&request, None),
            Some("192.0.2.10".parse().expect("IP must parse"))
        );
    }

    #[test]
    fn uses_only_rightmost_forwarded_ip_when_proxy_is_trusted() {
        let mut request = request_with_peer("192.0.2.10:4200");
        request.headers_mut().insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.3, 203.0.113.4"),
        );

        assert_eq!(
            client_ip_with_proxy_trust(&request, Some(TrustedProxyHeader::ForwardedFor)),
            Some("203.0.113.4".parse().expect("IP must parse"))
        );
    }

    #[test]
    fn malformed_forwarded_ip_falls_back_to_socket_peer() {
        let mut request = request_with_peer("192.0.2.10:4200");
        request.headers_mut().insert(
            "x-forwarded-for",
            HeaderValue::from_static("198.51.100.3, not-an-ip"),
        );

        assert_eq!(
            client_ip_with_proxy_trust(&request, Some(TrustedProxyHeader::ForwardedFor)),
            Some("192.0.2.10".parse().expect("IP must parse"))
        );
    }

    #[test]
    fn normalizes_ipv4_mapped_ipv6_addresses() {
        assert_eq!(
            parse_ip("::ffff:192.0.2.1"),
            Some("192.0.2.1".parse().expect("IP must parse"))
        );
    }

    #[test]
    fn enforces_client_and_global_quotas() {
        let limiter = RateLimiter::new(10, Duration::from_secs(600));
        let start = Instant::now();
        let first_ip = "192.0.2.1".parse().expect("IP must parse");
        let second_ip = "192.0.2.2".parse().expect("IP must parse");
        let rule = test_rule(2, 3);

        assert!(limiter.check_at(first_ip, rule, start).allowed);
        assert!(limiter.check_at(first_ip, rule, start).allowed);
        assert!(!limiter.check_at(first_ip, rule, start).allowed);
        assert!(limiter.check_at(second_ip, rule, start).allowed);
        assert!(!limiter.check_at(second_ip, rule, start).allowed);
    }

    #[test]
    fn client_map_is_bounded_and_idle_entries_expire() {
        let limiter = RateLimiter::new(2, Duration::from_secs(120));
        let start = Instant::now();
        let rule = test_rule(10, 100);

        for value in 1..=3 {
            let ip = IpAddr::from([192, 0, 2, value]);
            assert!(limiter.check_at(ip, rule, start).allowed);
        }

        {
            let state = limiter.state.lock().expect("limiter state must lock");
            assert_eq!(state.clients.len(), 2);
        }

        let later = start + Duration::from_secs(121);
        let new_ip = IpAddr::from([192, 0, 2, 100]);
        assert!(limiter.check_at(new_ip, rule, later).allowed);

        let state = limiter.state.lock().expect("limiter state must lock");
        assert_eq!(state.clients.len(), 1);
        assert!(state.clients.contains_key(&new_ip));
    }

    #[test]
    fn retry_after_is_rounded_up_and_never_zero() {
        let limiter = RateLimiter::new(2, Duration::from_secs(120));
        let start = Instant::now();
        let ip = IpAddr::from([192, 0, 2, 1]);
        let rule = test_rule(1, 100);

        assert!(limiter.check_at(ip, rule, start).allowed);
        let decision = limiter.check_at(ip, rule, start + Duration::from_millis(500));

        assert!(!decision.allowed);
        assert_eq!(decision.retry_after_seconds, 60);
    }

    #[tokio::test]
    async fn rate_limit_response_has_retry_and_json_code() {
        let response = rate_limited_response(17);

        assert_eq!(response.status(), axum::http::StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(response.headers().get(header::RETRY_AFTER).unwrap(), "17");
        assert_eq!(
            response.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-store"
        );
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["code"], "rate_limited");
    }
}
