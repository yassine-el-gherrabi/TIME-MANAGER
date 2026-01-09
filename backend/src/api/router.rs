use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderName, HeaderValue, Method};
use axum::routing::{delete, get, post, put};
use axum::Router;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower_http::trace::TraceLayer;

use super::handlers::absence_types;
use super::handlers::absences;
use super::handlers::auth;
use super::handlers::balances;
use super::handlers::clocks;
use super::handlers::health::health_check;
use super::handlers::holidays;
use super::handlers::kpis;
use super::handlers::password;
use super::handlers::schedules;
use super::handlers::teams;
use super::handlers::users;
use crate::config::AppState;

/// Creates the main application router with all endpoints
pub fn create_router(state: AppState) -> Router {
    // Build CORS layer from config
    let allowed_origins: Vec<HeaderValue> = state
        .config
        .cors_allowed_origins
        .iter()
        .filter_map(|origin| origin.parse::<HeaderValue>().ok())
        .collect();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            CONTENT_TYPE,
            AUTHORIZATION,
            ACCEPT,
            HeaderName::from_static("x-csrf-token"),
        ])
        .allow_credentials(true);

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
        .route("/:id/resend-invite", post(users::resend_invite))
        .route("/:id/schedule", put(schedules::assign_schedule));

    // Clock in/out routes
    let clock_routes = Router::new()
        .route("/in", post(clocks::clock_in))
        .route("/out", post(clocks::clock_out))
        .route("/status", get(clocks::get_status))
        .route("/history", get(clocks::get_history))
        .route("/pending", get(clocks::list_pending))
        .route("/:id/approve", post(clocks::approve_entry))
        .route("/:id/reject", post(clocks::reject_entry));

    // Team management routes
    let team_routes = Router::new()
        .route("/", get(teams::list_teams).post(teams::create_team))
        .route("/my", get(teams::get_my_teams))
        .route(
            "/:id",
            get(teams::get_team)
                .put(teams::update_team)
                .delete(teams::delete_team),
        )
        .route("/:id/members", post(teams::add_member))
        .route("/:team_id/members/:user_id", delete(teams::remove_member));

    // Work schedule routes
    let schedule_routes = Router::new()
        .route("/", get(schedules::list_schedules).post(schedules::create_schedule))
        .route("/me", get(schedules::get_my_schedule))
        .route(
            "/:id",
            get(schedules::get_schedule)
                .put(schedules::update_schedule)
                .delete(schedules::delete_schedule),
        )
        .route("/:id/days", post(schedules::add_day))
        .route(
            "/days/:day_id",
            put(schedules::update_day).delete(schedules::remove_day),
        );

    // KPI routes
    let kpi_routes = Router::new()
        .route("/me", get(kpis::get_my_kpis))
        .route("/users/:id", get(kpis::get_user_kpis))
        .route("/teams/:id", get(kpis::get_team_kpis))
        .route("/organization", get(kpis::get_org_kpis))
        .route("/presence", get(kpis::get_presence))
        .route("/charts", get(kpis::get_charts));

    // Absence type routes
    let absence_type_routes = Router::new()
        .route(
            "/",
            get(absence_types::list_absence_types).post(absence_types::create_absence_type),
        )
        .route("/seed", post(absence_types::seed_absence_types))
        .route(
            "/:id",
            get(absence_types::get_absence_type)
                .put(absence_types::update_absence_type)
                .delete(absence_types::delete_absence_type),
        );

    // Absence routes
    let absence_routes = Router::new()
        .route(
            "/",
            get(absences::list_absences).post(absences::create_absence),
        )
        .route("/pending", get(absences::list_pending_absences))
        .route("/:id", get(absences::get_absence))
        .route("/:id/approve", post(absences::approve_absence))
        .route("/:id/reject", post(absences::reject_absence))
        .route("/:id/cancel", post(absences::cancel_absence));

    // Leave balance routes
    let balance_routes = Router::new()
        .route("/", get(balances::list_balances))
        .route("/me", get(balances::get_my_balances))
        .route("/:id/adjust", put(balances::adjust_balance));

    // Holiday routes
    let holiday_routes = Router::new()
        .route(
            "/",
            get(holidays::list_holidays).post(holidays::create_holiday),
        )
        .route("/seed", post(holidays::seed_holidays))
        .route(
            "/:id",
            get(holidays::get_holiday)
                .put(holidays::update_holiday)
                .delete(holidays::delete_holiday),
        );

    // Main router
    Router::new()
        .route("/health", get(health_check))
        .nest("/v1/auth", auth_routes)
        .nest("/v1/auth/password", password_routes)
        .nest("/v1/users", user_routes)
        .nest("/v1/clocks", clock_routes)
        .nest("/v1/teams", team_routes)
        .nest("/v1/schedules", schedule_routes)
        .nest("/v1/kpis", kpi_routes)
        .nest("/v1/absence-types", absence_type_routes)
        .nest("/v1/absences", absence_routes)
        .nest("/v1/balances", balance_routes)
        .nest("/v1/holidays", holiday_routes)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use diesel::r2d2::{ConnectionManager, Pool};
    use diesel::PgConnection;
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use std::sync::Arc;
    use testcontainers::{clients::Cli, Container};
    use testcontainers_modules::postgres::Postgres;
    use tower::ServiceExt;

    use crate::config::app::{AppConfig, AppState};
    use crate::config::email::EmailConfig;
    use crate::config::hibp::HibpConfig;
    use crate::services::{EmailService, HibpService};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    /// Creates a PostgreSQL container for integration testing
    fn setup_postgres_container(docker: &Cli) -> Container<Postgres> {
        docker.run(Postgres::default())
    }

    /// Gets the database URL from a running container
    fn get_container_db_url(container: &Container<Postgres>) -> String {
        let port = container.get_host_port_ipv4(5432);
        format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port)
    }

    /// Creates a test AppConfig with disabled services
    fn create_test_config(database_url: &str) -> AppConfig {
        AppConfig {
            app_host: "127.0.0.1".to_string(),
            app_port: 8080,
            database_url: database_url.to_string(),
            rust_log: "info".to_string(),
            jwt_secret: "test-secret-key-for-integration-tests-minimum-32-chars".to_string(),
            jwt_access_token_expiry_seconds: 900,
            jwt_refresh_token_expiry_seconds: 604800,
            cors_allowed_origins: vec!["http://localhost:3000".to_string()],
            metrics_enabled: false,
            email: EmailConfig {
                smtp_host: "localhost".to_string(),
                smtp_port: 1025,
                smtp_username: None,
                smtp_password: None,
                from_email: "test@example.com".to_string(),
                from_name: "Test".to_string(),
                frontend_url: "http://localhost:5173".to_string(),
                enabled: false,
            },
            hibp: HibpConfig::disabled(),
        }
    }

    /// Creates a test AppState with real database connection
    fn create_test_state(database_url: &str) -> AppState {
        let config = create_test_config(database_url);

        let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
        let db_pool = Pool::builder()
            .max_size(5)
            .build(manager)
            .expect("Failed to create test pool");

        // Run migrations
        let mut conn = db_pool.get().expect("Failed to get connection");
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Failed to run migrations");

        let email_service = EmailService::new(config.email.clone())
            .expect("Failed to create email service");
        let hibp_service = HibpService::new(config.hibp.clone());

        AppState {
            config,
            db_pool,
            email_service: Arc::new(email_service),
            hibp_service: Arc::new(hibp_service),
        }
    }

    #[tokio::test]
    #[ignore = "Requires Docker - run with: cargo test -- --ignored"]
    async fn test_health_route_with_testcontainers() {
        let docker = Cli::default();
        let container = setup_postgres_container(&docker);
        let db_url = get_container_db_url(&container);

        let state = create_test_state(&db_url);
        let app = create_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    #[ignore = "Requires Docker - run with: cargo test -- --ignored"]
    async fn test_login_route_validation() {
        let docker = Cli::default();
        let container = setup_postgres_container(&docker);
        let db_url = get_container_db_url(&container);

        let state = create_test_state(&db_url);
        let app = create_router(state);

        // Test with invalid JSON
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/auth/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"invalid": "data"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should return 400 Bad Request for missing required fields
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[ignore = "Requires Docker - run with: cargo test -- --ignored"]
    async fn test_protected_route_without_auth() {
        let docker = Cli::default();
        let container = setup_postgres_container(&docker);
        let db_url = get_container_db_url(&container);

        let state = create_test_state(&db_url);
        let app = create_router(state);

        // Try to access /me without auth
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/auth/me")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Should return 401 Unauthorized
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
