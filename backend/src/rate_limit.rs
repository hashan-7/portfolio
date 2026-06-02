use std::{
    collections::{HashMap, VecDeque},
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

use axum::{
    Json,
    extract::Request,
    http::{StatusCode, header},
    response::Response,
};
use serde_json::{Value, json};

struct RateLimitRule {
    max_requests: usize,
    window: Duration,
}

#[derive(Default)]
struct RateLimiter {
    requests: Mutex<HashMap<String, VecDeque<Instant>>>,
}

static CHAT_LIMITER: OnceLock<RateLimiter> = OnceLock::new();
static ADMIN_LOGIN_LIMITER: OnceLock<RateLimiter> = OnceLock::new();
static ADMIN_LIMITER: OnceLock<RateLimiter> = OnceLock::new();

pub async fn chat_rate_limit(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    apply_rate_limit(
        req,
        next,
        CHAT_LIMITER.get_or_init(RateLimiter::default),
        RateLimitRule {
            max_requests: 20,
            window: Duration::from_secs(60),
        },
    )
    .await
}

pub async fn admin_login_rate_limit(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    apply_rate_limit(
        req,
        next,
        ADMIN_LOGIN_LIMITER.get_or_init(RateLimiter::default),
        RateLimitRule {
            max_requests: 5,
            window: Duration::from_secs(300),
        },
    )
    .await
}

pub async fn admin_rate_limit(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    apply_rate_limit(
        req,
        next,
        ADMIN_LIMITER.get_or_init(RateLimiter::default),
        RateLimitRule {
            max_requests: 60,
            window: Duration::from_secs(60),
        },
    )
    .await
}

async fn apply_rate_limit(
    req: Request,
    next: axum::middleware::Next,
    limiter: &'static RateLimiter,
    rule: RateLimitRule,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let key = client_key(&req);

    if limiter.allow_request(&key, &rule) {
        return Ok(next.run(req).await);
    }

    Err((
        StatusCode::TOO_MANY_REQUESTS,
        Json(json!({
            "error": "Too many requests. Please wait and try again."
        })),
    ))
}

impl RateLimiter {
    fn allow_request(&self, key: &str, rule: &RateLimitRule) -> bool {
        let now = Instant::now();
        let mut requests = self.requests.lock().unwrap_or_else(|error| error.into_inner());
        let user_requests = requests.entry(key.to_string()).or_default();

        while let Some(first_request) = user_requests.front() {
            if now.duration_since(*first_request) > rule.window {
                user_requests.pop_front();
            } else {
                break;
            }
        }

        if user_requests.len() >= rule.max_requests {
            return false;
        }

        user_requests.push_back(now);
        true
    }
}

fn client_key(req: &Request) -> String {
    if let Some(ip) = header_value(req, "cf-connecting-ip") {
        return ip;
    }

    if let Some(forwarded_for) = header_value(req, "x-forwarded-for") {
        if let Some(first_ip) = forwarded_for.split(',').next() {
            let first_ip = first_ip.trim();

            if !first_ip.is_empty() {
                return first_ip.to_string();
            }
        }
    }

    if let Some(ip) = header_value(req, "x-real-ip") {
        return ip;
    }

    if let Some(user_agent) = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        return format!("unknown:{user_agent}");
    }

    "unknown".to_string()
}

fn header_value(req: &Request, name: &'static str) -> Option<String> {
    req.headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}