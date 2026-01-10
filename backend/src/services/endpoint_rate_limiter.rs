//! Endpoint-specific rate limiter for sensitive operations.
//! Provides configurable per-endpoint rate limiting with in-memory storage.

use crate::error::AppError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Rate limit configuration for an endpoint
#[derive(Clone, Debug)]
pub struct RateLimitConfig {
    /// Maximum requests allowed in the window
    pub max_requests: usize,
    /// Time window duration in seconds
    pub window_seconds: u64,
}

impl RateLimitConfig {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

/// Entry for tracking requests
struct RequestEntry {
    timestamps: Vec<Instant>,
}

/// Endpoint-specific rate limiter
///
/// Supports different rate limits for different endpoints.
/// Uses IP address + endpoint as the key for tracking.
#[derive(Clone)]
pub struct EndpointRateLimiter {
    /// Map of endpoint -> config
    configs: Arc<HashMap<String, RateLimitConfig>>,
    /// Map of (endpoint + ip) -> request timestamps
    requests: Arc<Mutex<HashMap<String, RequestEntry>>>,
}

impl EndpointRateLimiter {
    /// Create a new rate limiter with default configurations for sensitive endpoints
    pub fn new() -> Self {
        let mut configs = HashMap::new();

        // Password reset request: 3 requests per 5 minutes
        configs.insert(
            "password_reset_request".to_string(),
            RateLimitConfig::new(3, 300),
        );

        // Password reset: 5 requests per 5 minutes
        configs.insert(
            "password_reset".to_string(),
            RateLimitConfig::new(5, 300),
        );

        // Accept invite: 5 requests per 5 minutes
        configs.insert(
            "accept_invite".to_string(),
            RateLimitConfig::new(5, 300),
        );

        // Verify invite: 10 requests per 5 minutes (less sensitive)
        configs.insert(
            "verify_invite".to_string(),
            RateLimitConfig::new(10, 300),
        );

        Self {
            configs: Arc::new(configs),
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Check if a request is allowed for the given endpoint and IP
    ///
    /// Returns Ok(remaining_requests) if allowed, Err with retry_after if rate limited
    pub fn check_rate_limit(
        &self,
        endpoint: &str,
        ip_address: &str,
    ) -> Result<usize, AppError> {
        let config = match self.configs.get(endpoint) {
            Some(c) => c,
            None => {
                // No config for this endpoint, allow all requests
                return Ok(usize::MAX);
            }
        };

        let key = format!("{}:{}", endpoint, ip_address);
        let now = Instant::now();
        let window_start = now - Duration::from_secs(config.window_seconds);

        let mut requests = self
            .requests
            .lock()
            .map_err(|_| AppError::InternalError)?;

        let entry = requests.entry(key).or_insert_with(|| RequestEntry {
            timestamps: Vec::new(),
        });

        // Remove expired timestamps
        entry.timestamps.retain(|&ts| ts > window_start);

        // Check if limit exceeded
        if entry.timestamps.len() >= config.max_requests {
            let oldest = entry.timestamps.first().copied();
            let retry_after = oldest
                .map(|ts| {
                    let elapsed = now.duration_since(ts);
                    Duration::from_secs(config.window_seconds).saturating_sub(elapsed)
                })
                .unwrap_or(Duration::from_secs(config.window_seconds));

            return Err(AppError::TooManyRequests(format!(
                "Rate limit exceeded. Please try again in {} seconds.",
                retry_after.as_secs() + 1
            )));
        }

        // Record this request
        entry.timestamps.push(now);

        // Return remaining requests
        Ok(config.max_requests - entry.timestamps.len())
    }

    /// Get the configuration for an endpoint
    pub fn get_config(&self, endpoint: &str) -> Option<&RateLimitConfig> {
        self.configs.get(endpoint)
    }

    /// Cleanup old entries to prevent memory growth
    pub fn cleanup(&self) -> Result<usize, AppError> {
        let mut requests = self
            .requests
            .lock()
            .map_err(|_| AppError::InternalError)?;

        let now = Instant::now();
        let max_window = Duration::from_secs(600); // 10 minutes max

        let before_count = requests.len();

        requests.retain(|_, entry| {
            entry.timestamps.retain(|&ts| now.duration_since(ts) < max_window);
            !entry.timestamps.is_empty()
        });

        Ok(before_count - requests.len())
    }
}

impl Default for EndpointRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_within_limit() {
        let limiter = EndpointRateLimiter::new();
        let endpoint = "password_reset_request";
        let ip = "192.168.1.1";

        // Should allow 3 requests
        for i in 0..3 {
            let result = limiter.check_rate_limit(endpoint, ip);
            assert!(result.is_ok(), "Request {} should be allowed", i + 1);
        }
    }

    #[test]
    fn test_rate_limiter_blocks_excess() {
        let limiter = EndpointRateLimiter::new();
        let endpoint = "password_reset_request";
        let ip = "192.168.1.1";

        // Use up the limit (3 requests for password_reset_request)
        for _ in 0..3 {
            limiter.check_rate_limit(endpoint, ip).unwrap();
        }

        // 4th request should be blocked
        let result = limiter.check_rate_limit(endpoint, ip);
        assert!(result.is_err());

        match result {
            Err(AppError::TooManyRequests(_)) => (),
            _ => panic!("Expected TooManyRequests error"),
        }
    }

    #[test]
    fn test_rate_limiter_different_ips() {
        let limiter = EndpointRateLimiter::new();
        let endpoint = "password_reset_request";

        // Different IPs should have independent limits
        assert!(limiter.check_rate_limit(endpoint, "ip1").is_ok());
        assert!(limiter.check_rate_limit(endpoint, "ip2").is_ok());
        assert!(limiter.check_rate_limit(endpoint, "ip3").is_ok());
    }

    #[test]
    fn test_rate_limiter_different_endpoints() {
        let limiter = EndpointRateLimiter::new();
        let ip = "192.168.1.1";

        // Different endpoints should have independent limits
        assert!(limiter.check_rate_limit("password_reset_request", ip).is_ok());
        assert!(limiter.check_rate_limit("password_reset", ip).is_ok());
        assert!(limiter.check_rate_limit("accept_invite", ip).is_ok());
    }

    #[test]
    fn test_unknown_endpoint_allows_all() {
        let limiter = EndpointRateLimiter::new();
        let ip = "192.168.1.1";

        // Unknown endpoint should allow unlimited requests
        for _ in 0..100 {
            assert!(limiter.check_rate_limit("unknown_endpoint", ip).is_ok());
        }
    }

    #[test]
    fn test_cleanup() {
        let limiter = EndpointRateLimiter::new();

        // Add some requests
        limiter.check_rate_limit("password_reset_request", "ip1").unwrap();
        limiter.check_rate_limit("password_reset_request", "ip2").unwrap();

        // Cleanup should work without errors
        let result = limiter.cleanup();
        assert!(result.is_ok());
    }

    #[test]
    fn test_remaining_requests() {
        let limiter = EndpointRateLimiter::new();
        let endpoint = "password_reset_request"; // max 3
        let ip = "192.168.1.1";

        assert_eq!(limiter.check_rate_limit(endpoint, ip).unwrap(), 2); // 3-1=2 remaining
        assert_eq!(limiter.check_rate_limit(endpoint, ip).unwrap(), 1); // 3-2=1 remaining
        assert_eq!(limiter.check_rate_limit(endpoint, ip).unwrap(), 0); // 3-3=0 remaining
    }
}
