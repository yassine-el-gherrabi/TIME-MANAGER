use anyhow::Context;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use timemanager_backend::{
    api::router::create_router,
    config::app::{AppConfig, AppState},
    config::database::create_pool,
    repositories::{
        InviteTokenRepository, LoginAttemptRepository, PasswordResetRepository,
        RefreshTokenRepository, UserSessionRepository,
    },
    services::{EmailService, EndpointRateLimiter, HibpService, MetricsService},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Embed migrations at compile time
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Cleanup interval: 24 hours
const CLEANUP_INTERVAL_SECS: u64 = 86400;

/// Background cleanup job for expired/old data
async fn run_cleanup_jobs(pool: Pool<ConnectionManager<PgConnection>>, rate_limiter: Arc<EndpointRateLimiter>) {
    let mut interval = tokio::time::interval(Duration::from_secs(CLEANUP_INTERVAL_SECS));

    // Skip the first immediate tick
    interval.tick().await;

    loop {
        interval.tick().await;
        tracing::info!("Starting scheduled cleanup jobs...");

        // Cleanup expired sessions
        let session_repo = UserSessionRepository::new(pool.clone());
        match session_repo.delete_expired().await {
            Ok(count) => tracing::info!("Cleaned up {} expired sessions", count),
            Err(e) => tracing::error!("Failed to cleanup expired sessions: {}", e),
        }

        // Cleanup old login attempts (> 30 days)
        let login_attempt_repo = LoginAttemptRepository::new(pool.clone());
        match login_attempt_repo.delete_older_than(30).await {
            Ok(count) => tracing::info!("Cleaned up {} old login attempts", count),
            Err(e) => tracing::error!("Failed to cleanup login attempts: {}", e),
        }

        // Cleanup expired refresh tokens
        let refresh_token_repo = RefreshTokenRepository::new(pool.clone());
        match refresh_token_repo.delete_expired().await {
            Ok(count) => tracing::info!("Cleaned up {} expired refresh tokens", count),
            Err(e) => tracing::error!("Failed to cleanup refresh tokens: {}", e),
        }

        // Cleanup expired password reset tokens
        let password_reset_repo = PasswordResetRepository::new(pool.clone());
        match password_reset_repo.delete_expired().await {
            Ok(count) => tracing::info!("Cleaned up {} expired password reset tokens", count),
            Err(e) => tracing::error!("Failed to cleanup password reset tokens: {}", e),
        }

        // Cleanup expired invite tokens
        let invite_token_repo = InviteTokenRepository::new(pool.clone());
        match invite_token_repo.delete_expired().await {
            Ok(count) => tracing::info!("Cleaned up {} expired invite tokens", count),
            Err(e) => tracing::error!("Failed to cleanup invite tokens: {}", e),
        }

        // Cleanup in-memory rate limiter
        match rate_limiter.cleanup() {
            Ok(count) => tracing::info!("Cleaned up {} rate limiter entries", count),
            Err(e) => tracing::error!("Failed to cleanup rate limiter: {}", e),
        }

        tracing::info!("Scheduled cleanup jobs completed");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "timemanager_backend=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env()?;

    tracing::info!("Starting Time Manager Backend");
    tracing::info!("Environment: {}", config.rust_log);
    tracing::info!(
        "Server will listen on {}:{}",
        config.app_host,
        config.app_port
    );

    // Create database connection pool
    let db_pool = create_pool(&config.database_url)?;
    tracing::info!("Database connection pool created");

    // Run embedded migrations
    tracing::info!("Running database migrations...");
    let mut conn = db_pool
        .get()
        .context("Failed to get database connection for migrations")?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {}", e))?;

    tracing::info!("Database migrations completed successfully");

    // Create email service
    let email_service =
        EmailService::new(config.email.clone()).context("Failed to create email service")?;
    tracing::info!(
        "Email service initialized (enabled: {})",
        email_service.is_enabled()
    );

    // Create HIBP service
    let hibp_service = HibpService::new(config.hibp.clone());
    tracing::info!(
        "HIBP password breach checking initialized (enabled: {})",
        hibp_service.is_enabled()
    );

    // Create endpoint rate limiter
    let rate_limiter = Arc::new(EndpointRateLimiter::new());
    tracing::info!("Endpoint rate limiter initialized");

    // Create metrics service
    let metrics_service = Arc::new(MetricsService::new());
    tracing::info!("Prometheus metrics service initialized");

    // Create application state
    let state = AppState {
        config: config.clone(),
        db_pool: db_pool.clone(),
        email_service: Arc::new(email_service),
        hibp_service: Arc::new(hibp_service),
        rate_limiter: rate_limiter.clone(),
        metrics_service,
    };

    // Spawn background cleanup job
    tokio::spawn(run_cleanup_jobs(db_pool, rate_limiter));
    tracing::info!("Background cleanup job scheduled (runs every 24 hours)");

    // Create application router with state
    let app = create_router(state);

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Health check available at http://{}/health", addr);
    tracing::info!("Prometheus metrics available at http://{}/metrics", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
