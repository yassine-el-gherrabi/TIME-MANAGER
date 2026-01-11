//! HTTP Metrics Middleware
//!
//! Captures HTTP request metrics for Prometheus:
//! - Request count by method, path, and status
//! - Request duration histogram

use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

use crate::services::metrics_service::record_http_request;

/// Metrics middleware that records HTTP request metrics
///
/// This middleware captures:
/// - HTTP method (GET, POST, PUT, DELETE, etc.)
/// - Normalized path (UUIDs replaced with :id)
/// - Response status code class (2xx, 3xx, 4xx, 5xx)
/// - Request duration in seconds
pub async fn metrics_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Execute the request
    let response = next.run(req).await;

    // Record metrics
    let duration = start.elapsed().as_secs_f64();
    let status = response.status().as_u16();

    // Record the HTTP request metric
    record_http_request(&method, &path, status, duration);

    // Log HTTP request for Loki (only for non-health routes to reduce noise)
    if path != "/health" && path != "/metrics" {
        tracing::info!(
            method = %method,
            path = %path,
            status = %status,
            duration_ms = %format!("{:.2}", duration * 1000.0),
            "HTTP request"
        );
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    async fn test_handler() -> &'static str {
        "Hello"
    }

    #[tokio::test]
    async fn test_metrics_middleware_records_request() {
        // This test validates the middleware structure
        // Actual metrics recording is tested via integration tests
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn(metrics_middleware));

        let request = Request::builder()
            .uri("/test")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_metrics_middleware_handles_404() {
        let app = Router::new()
            .route("/test", get(test_handler))
            .layer(axum::middleware::from_fn(metrics_middleware));

        let request = Request::builder()
            .uri("/not-found")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
