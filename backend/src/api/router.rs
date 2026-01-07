use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

use super::handlers::health::health_check;

/// Creates the main application router
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::Service;

    #[tokio::test]
    async fn test_health_route() {
        let mut app = create_router();

        let response = app
            .call(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
