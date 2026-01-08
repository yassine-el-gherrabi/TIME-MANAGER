use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Rate limiter configuration
const MAX_REQUESTS_PER_WINDOW: usize = 100;
const WINDOW_DURATION_SECS: u64 = 60;

/// Rate limiter state
#[derive(Clone)]
pub struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn check_rate_limit(&self, key: &str) -> Result<bool, RateLimitError> {
        let mut requests = self
            .requests
            .lock()
            .map_err(|_| RateLimitError::LockPoisoned)?;
        let now = Instant::now();
        let window_start = now - Duration::from_secs(WINDOW_DURATION_SECS);

        // Get or create request log for this key
        let request_log = requests.entry(key.to_string()).or_default();

        // Remove expired requests
        request_log.retain(|&timestamp| timestamp > window_start);

        // Check if limit exceeded
        if request_log.len() >= MAX_REQUESTS_PER_WINDOW {
            return Ok(false);
        }

        // Add new request
        request_log.push(now);
        Ok(true)
    }

    pub fn cleanup_old_entries(&self) -> Result<(), RateLimitError> {
        let mut requests = self
            .requests
            .lock()
            .map_err(|_| RateLimitError::LockPoisoned)?;
        let now = Instant::now();
        let window_start = now - Duration::from_secs(WINDOW_DURATION_SECS * 2);

        requests.retain(|_, request_log| {
            request_log.retain(|&timestamp| timestamp > window_start);
            !request_log.is_empty()
        });
        Ok(())
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiting middleware
///
/// Limits requests per IP address to prevent abuse
/// Returns 429 Too Many Requests if limit exceeded
pub async fn rate_limit_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, RateLimitError> {
    // Extract IP address from request
    let ip = extract_ip_from_request(&req).ok_or(RateLimitError::NoIpAddress)?;

    // Get rate limiter from extensions (should be added by app state)
    let rate_limiter = req
        .extensions()
        .get::<RateLimiter>()
        .ok_or(RateLimitError::NoRateLimiter)?
        .clone();

    // Check rate limit
    if !rate_limiter.check_rate_limit(&ip)? {
        return Err(RateLimitError::LimitExceeded);
    }

    Ok(next.run(req).await)
}

/// Extract IP address from request
fn extract_ip_from_request(req: &Request<Body>) -> Option<String> {
    // Try X-Forwarded-For header first (proxy/load balancer)
    if let Some(forwarded) = req.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Take first IP from comma-separated list
            if let Some(ip) = forwarded_str.split(',').next() {
                return Some(ip.trim().to_string());
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = req.headers().get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            return Some(ip_str.to_string());
        }
    }

    // Fallback to connection remote addr (not available in middleware context)
    // In production, this would come from connection info
    None
}

/// Rate limiting errors
#[derive(Debug)]
pub enum RateLimitError {
    LimitExceeded,
    NoIpAddress,
    NoRateLimiter,
    LockPoisoned,
}

impl IntoResponse for RateLimitError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            RateLimitError::LimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                format!(
                    "Rate limit exceeded. Maximum {} requests per {} seconds",
                    MAX_REQUESTS_PER_WINDOW, WINDOW_DURATION_SECS
                ),
            ),
            RateLimitError::NoIpAddress => (
                StatusCode::BAD_REQUEST,
                "Could not determine client IP address".to_string(),
            ),
            RateLimitError::NoRateLimiter => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Rate limiter not configured".to_string(),
            ),
            RateLimitError::LockPoisoned => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal rate limiter error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": "Rate Limit",
            "message": message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_requests_within_limit() {
        let limiter = RateLimiter::new();
        let key = "test_ip";

        for _ in 0..MAX_REQUESTS_PER_WINDOW {
            assert!(limiter.check_rate_limit(key).unwrap());
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess_requests() {
        let limiter = RateLimiter::new();
        let key = "test_ip";

        // Use up the limit
        for _ in 0..MAX_REQUESTS_PER_WINDOW {
            assert!(limiter.check_rate_limit(key).unwrap());
        }

        // Next request should be blocked
        assert!(!limiter.check_rate_limit(key).unwrap());
    }

    #[test]
    fn test_rate_limiter_different_keys() {
        let limiter = RateLimiter::new();

        // Different keys should have independent limits
        assert!(limiter.check_rate_limit("ip1").unwrap());
        assert!(limiter.check_rate_limit("ip2").unwrap());
    }

    #[test]
    fn test_cleanup_old_entries() {
        let limiter = RateLimiter::new();
        let key = "test_ip";

        limiter.check_rate_limit(key).unwrap();
        limiter.cleanup_old_entries().unwrap();

        // Should still have entries (not old enough to clean)
        let requests = limiter.requests.lock().unwrap();
        assert!(requests.contains_key(key));
    }

    #[test]
    fn test_error_responses() {
        let error = RateLimitError::LimitExceeded;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);

        let error = RateLimitError::NoIpAddress;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let error = RateLimitError::NoRateLimiter;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let error = RateLimitError::LockPoisoned;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
