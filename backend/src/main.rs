use anyhow::Context;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use std::sync::Arc;
use timemanager_backend::{
    api::router::create_router,
    config::app::{AppConfig, AppState},
    config::database::create_pool,
    services::{EmailService, HibpService},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Embed migrations at compile time
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

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

    // Create application state
    let state = AppState {
        config: config.clone(),
        db_pool,
        email_service: Arc::new(email_service),
        hibp_service: Arc::new(hibp_service),
    };

    // Create application router with state
    let app = create_router(state);

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Health check available at http://{}/health", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
