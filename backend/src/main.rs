use anyhow::Context;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use timemanager_backend::{
    api::router::create_router,
    config::app::{AppConfig, AppState},
    config::database::{create_pool, DbPool},
    repositories::{
        InviteTokenRepository, LoginAttemptRepository, PasswordResetRepository,
        RefreshTokenRepository, UserSessionRepository,
    },
    services::{EmailService, EndpointRateLimiter, HibpService, MetricsService},
};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

// Embed migrations at compile time
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Cleanup interval: 24 hours
const CLEANUP_INTERVAL_SECS: u64 = 86400;

/// Initialize tracing with OpenTelemetry support for Tempo and JSON logging for Loki
fn init_tracing() -> anyhow::Result<()> {
    // Check if OTLP endpoint is configured
    let otel_endpoint = std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();

    // Build the tracing subscriber
    // Default to INFO level - DEBUG is too verbose for Loki
    // Filter out noisy dependencies (hyper, h2, tower, etc.)
    // Override with RUST_LOG env var if needed (e.g., RUST_LOG=timemanager_backend=debug)
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            "timemanager_backend=info,\
             hyper=warn,\
             h2=warn,\
             tower=warn,\
             tower_http=warn,\
             axum=warn,\
             axum::rejection=warn,\
             diesel=warn,\
             r2d2=warn"
                .into()
        });

    // JSON format layer for Loki - minimal output
    // Loki adds: timestamp, level detection, container labels
    // We only output: message + flattened contextual fields (user_id, method, etc.)
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .without_time()           // Loki adds its own timestamp
        .with_target(false)       // Don't include module path (too verbose)
        .with_current_span(false) // Don't include span info (Tempo handles tracing)
        .flatten_event(true)      // Flatten fields to root level
        .with_span_events(FmtSpan::NONE); // Don't log span open/close events

    if let Some(endpoint) = otel_endpoint {
        // OpenTelemetry OTLP exporter for Tempo
        let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(&endpoint)
            .build()
            .context("Failed to create OTLP exporter")?;

        let service_name = std::env::var("OTEL_SERVICE_NAME")
            .unwrap_or_else(|_| "timemanager-backend".to_string());

        let provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(otlp_exporter, runtime::Tokio)
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", service_name),
            ]))
            .build();

        let tracer = provider.tracer("timemanager-backend");
        let otel_layer = OpenTelemetryLayer::new(tracer);

        // Register the provider globally for shutdown
        opentelemetry::global::set_tracer_provider(provider);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(otel_layer)
            .init();

        tracing::info!("OpenTelemetry tracing initialized (endpoint: {})", endpoint);
    } else {
        // Fallback: JSON logging only (no OTLP)
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .init();

        tracing::info!("Tracing initialized (JSON logging only, no OTLP endpoint configured)");
    }

    Ok(())
}

/// Background cleanup job for expired/old data
async fn run_cleanup_jobs(pool: DbPool, rate_limiter: Arc<EndpointRateLimiter>) {
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
    // Initialize tracing with OpenTelemetry support
    init_tracing()?;

    // Load configuration
    let config = AppConfig::from_env()?;

    tracing::info!("Starting Time Manager Backend");
    tracing::info!("Environment: {}", config.rust_log);
    tracing::info!(
        "Server will listen on {}:{}",
        config.app_host,
        config.app_port
    );

    // Run embedded migrations (uses synchronous connection)
    tracing::info!("Running database migrations...");
    {
        let mut conn = PgConnection::establish(&config.database_url)
            .context("Failed to connect to database for migrations")?;
        conn.run_pending_migrations(MIGRATIONS)
            .map_err(|e| anyhow::anyhow!("Failed to run database migrations: {}", e))?;
    }
    tracing::info!("Database migrations completed successfully");

    // Create async database connection pool
    let db_pool = create_pool(&config.database_url).await?;
    tracing::info!("Async database connection pool created");

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
