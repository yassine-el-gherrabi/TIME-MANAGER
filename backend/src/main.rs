use std::net::SocketAddr;
use timemanager_backend::{api::router::create_router, config::app::AppConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Create application router
    let app = create_router();

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));

    tracing::info!("Server running on http://{}", addr);
    tracing::info!("Health check available at http://{}/health", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
