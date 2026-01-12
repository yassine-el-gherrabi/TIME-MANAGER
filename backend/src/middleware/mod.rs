// Middleware for request/response processing
// This module contains Axum middleware for authentication, CSRF, rate limiting, and metrics

pub mod auth_middleware;
pub mod csrf_middleware;
pub mod metrics_middleware;
pub mod rate_limit_middleware;

// Re-export commonly used types
pub use auth_middleware::{auth_middleware, optional_auth_middleware, AuthMiddlewareError};
pub use csrf_middleware::{csrf_middleware, optional_csrf_middleware, CsrfMiddlewareError};
pub use metrics_middleware::metrics_middleware;
pub use rate_limit_middleware::{rate_limit_middleware, RateLimitError, RateLimiter};
