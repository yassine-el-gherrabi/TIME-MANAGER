use crate::config::email::EmailConfig;
use crate::services::EmailService;
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: Pool<ConnectionManager<PgConnection>>,
    pub email_service: Arc<EmailService>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub rust_log: String,
    pub jwt_secret: String,
    pub jwt_access_token_expiry_seconds: u64,
    pub jwt_refresh_token_expiry_seconds: u64,
    pub cors_allowed_origins: Vec<String>,
    pub metrics_enabled: bool,
    pub email: EmailConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        // Load .env file if present
        dotenv().ok();

        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let app_port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;

        let database_url = env::var("DATABASE_URL")?;

        let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        let jwt_secret =
            env::var("JWT_SECRET").expect("JWT_SECRET environment variable must be set");

        let jwt_access_token_expiry_seconds = env::var("JWT_ACCESS_TOKEN_EXPIRY_SECONDS")
            .unwrap_or_else(|_| "900".to_string())
            .parse::<u64>()?;

        let jwt_refresh_token_expiry_seconds = env::var("JWT_REFRESH_TOKEN_EXPIRY_SECONDS")
            .unwrap_or_else(|_| "604800".to_string())
            .parse::<u64>()?;

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let metrics_enabled = env::var("METRICS_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()?;

        let email = EmailConfig::from_env()?;

        Ok(Self {
            app_host,
            app_port,
            database_url,
            rust_log,
            jwt_secret,
            jwt_access_token_expiry_seconds,
            jwt_refresh_token_expiry_seconds,
            cors_allowed_origins,
            metrics_enabled,
            email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // Set minimal required env vars
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("JWT_SECRET", "test-secret-key-for-unit-tests");

        let config = AppConfig::from_env().unwrap();

        assert_eq!(config.app_host, "0.0.0.0");
        assert_eq!(config.app_port, 8080);
        assert!(config.metrics_enabled);
        assert_eq!(config.jwt_secret, "test-secret-key-for-unit-tests");
    }
}
