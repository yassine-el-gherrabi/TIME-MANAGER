use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::trace::TraceLayer;

use super::handlers::auth;
use super::handlers::health::health_check;
use super::handlers::password;
use super::handlers::users;
use crate::config::AppState;

/// Creates the main application router with all endpoints
pub fn create_router(state: AppState) -> Router {
    // Auth routes - public endpoints
    let auth_routes = Router::new()
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/logout", post(auth::logout))
        .route("/logout-all", post(auth::logout_all))
        .route("/me", get(auth::me))
        .route("/change-password", put(auth::change_password))
        .route("/accept-invite", post(auth::accept_invite))
        .route("/verify-invite", post(auth::verify_invite))
        .route("/sessions", get(auth::get_sessions))
        .route("/sessions/:id", delete(auth::revoke_session));

    // Password management routes
    let password_routes = Router::new()
        .route("/request-reset", post(password::request_reset))
        .route("/reset", post(password::reset_password));

    // User management routes (Admin only)
    let user_routes = Router::new()
        .route("/", get(users::list_users).post(users::create_user))
        .route(
            "/:id",
            get(users::get_user)
                .put(users::update_user)
                .delete(users::delete_user),
        )
        .route("/:id/resend-invite", post(users::resend_invite));

    // Main router
    Router::new()
        .route("/health", get(health_check))
        .nest("/v1/auth", auth_routes)
        .nest("/v1/auth/password", password_routes)
        .nest("/v1/users", user_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::Service;

    // TODO: Fix this test - needs proper AppState mock with database pool
    #[tokio::test]
    #[ignore = "Requires AppState mock with database pool"]
    async fn test_health_route() {
        // This test needs a proper AppState with db_pool to work
        // For now, router integration is tested via E2E tests
        let _app: Router = todo!("Create mock AppState");

        let response = _app
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
