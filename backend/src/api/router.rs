use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;

use crate::config::AppState;
use super::handlers::health::health_check;
use super::handlers::auth;
use super::handlers::password;

/// Creates the main application router with all endpoints
pub fn create_router(state: AppState) -> Router {
    // Auth routes - public endpoints
    let auth_routes = Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
        .route("/logout-all", post(auth::logout_all))
        .route("/me", get(auth::me));

    // Password management routes
    let password_routes = Router::new()
        .route("/request-reset", post(password::request_reset))
        .route("/reset", post(password::reset_password));

    // Main router
    Router::new()
        .route("/health", get(health_check))
        .nest("/v1/auth", auth_routes)
        .nest("/v1/auth/password", password_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
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
