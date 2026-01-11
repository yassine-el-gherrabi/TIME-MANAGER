use crate::config::database::DbPool;
use crate::config::email::EmailConfig;
use crate::config::hibp::HibpConfig;
use crate::services::{EmailService, EndpointRateLimiter, HibpService, MetricsService};
use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db_pool: DbPool,
    pub email_service: Arc<EmailService>,
    pub hibp_service: Arc<HibpService>,
    pub rate_limiter: Arc<EndpointRateLimiter>,
    pub metrics_service: Arc<MetricsService>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub rust_log: String,
    pub jwt_private_key: String,
    pub jwt_public_key: String,
    pub jwt_access_token_expiry_seconds: u64,
    pub jwt_refresh_token_expiry_seconds: u64,
    pub cors_allowed_origins: Vec<String>,
    pub metrics_enabled: bool,
    pub email: EmailConfig,
    pub hibp: HibpConfig,
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

        // Load JWT keys from PEM files
        let jwt_keys_path = env::var("JWT_KEYS_PATH").unwrap_or_else(|_| "./keys".to_string());
        let jwt_private_key = std::fs::read_to_string(format!("{}/jwt_private.pem", jwt_keys_path))
            .context(format!(
                "Failed to read JWT private key from {}/jwt_private.pem",
                jwt_keys_path
            ))?;
        let jwt_public_key = std::fs::read_to_string(format!("{}/jwt_public.pem", jwt_keys_path))
            .context(format!(
            "Failed to read JWT public key from {}/jwt_public.pem",
            jwt_keys_path
        ))?;

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
        let hibp = HibpConfig::from_env()?;

        Ok(Self {
            app_host,
            app_port,
            database_url,
            rust_log,
            jwt_private_key,
            jwt_public_key,
            jwt_access_token_expiry_seconds,
            jwt_refresh_token_expiry_seconds,
            cors_allowed_origins,
            metrics_enabled,
            email,
            hibp,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test RSA keys (2048-bit, for testing only)
    pub const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCfwD3rj7NWw0cH
su5hRvc5uo4lPhjFgx1HlZvxzt1pWafbr9zC1exr5HB/NxH1gN37e2nWOI8tPQWf
GsZecR2kGOc2LL3wWiBI9OXxZutf0mtzn3tcIz5vrjktVFM8Q1cVj1e+8wEcvyOw
netXKt+YDah0lBuxm61pg9omb0pnkCsXUjkrowe2c/50X6wBbT4zoYqeSu0EdVnR
ifRZKYbPTpKIh40AejfPHSHI7FPHq0BBnP3iUqtq+a2e9cO6kufZR/T1t9B2G/Zp
58PC/LYIYlINuk9+LcFHDZScoqmW9Aa5QP2JDmvZmbVD6Xd4CKOEh02fJZ8PkzEx
TzU10+1XAgMBAAECggEAJaFFlK7pWjcujKA36b8rLjSFFj293QypAXs63CdT3WSK
l0OiN1znz3RkkXrZ5qAf6gSkphr1kvzsTZGjh4ySpFxfXlIEvdClCTpyzb3mFNC+
keJPzyDYLLt36XcTEj90jHYS/75DFU/q6sgQLxzAxZL2Ctv2eAxJOXEfGm2ds64Q
9OYc/SnQQkpCYRLygfix93n2FlualLDuCZzlXBn/Usb8UzqylMrjzPUe7popIQ3+
QY6oJIgE3aeTBW1kfRgGK7fOcfJZY9q/M0mfAY8Zf3SxT1PTVTSFhunIDxo3Ay0K
XT+r9+YSyJ/0OycR7NsZSOifIwzBGOu/LAEGGA5wiQKBgQDb6J3R4IY55OEJpxJ/
pTwJdsEVmt5L/xoti029rMkwoEb5awBcK0bdQ06oJOHRInb5KTLFvZwhCWBSFyhC
FipnQXH5JRW8CNjlt7SGQZs5C7OJFxclAqfx0ba/oUzTyQ6ZfO1QBHNkx9XIX/DW
t/sEQ6xPWj5kcX1HxcwReCkiPwKBgQC5+B7gYLtSdt1gwHG4iZwTfo0AiZPdiSH7
kcN5JXWdJ4VP5dmtfuL3UOWRnbitfgIeBti//Po+Cd4h8i0CYFF20luOlj1Q4HH4
JPc61SGoykRs8a1DKFHm2YltWShHn5y3x5tarSzY38ndTPx/r1hvFoEHnF8+97gi
J49ozse+6QKBgD8dBtZqYvuQpcl4asW5rX5l18qUlQIop+G0Xk52nZNYHKaOwB6z
yPXN0HBPjYPRKWYfHdREs9+DamKFBOfaprbVwJkpvJAn1eAwFh6GC7+WjSNmPh1A
IuUzNAjRiVQrGwaQJSfW7ytYcxG7/0oQqXky1uw7UTbQn40Oxp+o5d1PAoGBAJh0
Peu3oRkjdKyCVzfvJ9IbZsBQCLYOW5t+jX7dJKQm5/Tt+xtt7+bLnMdZQzKHIHk5
J6uMWiFNuZqejCNsjpwYKxKjO7T3qrbApyTF4Igc+SdOoLlzbmEPaMgJ1SmSQcmv
iz40xZUtMLGJEV4jgx3elvyERti5/2uQftJu4fUxAoGBAMni474jFdHfz0WtHm/d
hTUmXvg1s9h033q0cqjT4CFHRi1JP8h7+Z8mYGa64+vgZFTl0c8+h27NGZdx33j7
T5Wb2QMgao7+BnKnHL2ymEIvaWhIbLXd7xTQsLBe4DvXQJmJCD3TeR6exrXK/lkI
/4D2c3OjbJmOwh4TOcI94I9Q
-----END PRIVATE KEY-----";
    pub const TEST_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAn8A964+zVsNHB7LuYUb3
ObqOJT4YxYMdR5Wb8c7daVmn26/cwtXsa+RwfzcR9YDd+3tp1jiPLT0FnxrGXnEd
pBjnNiy98FogSPTl8WbrX9Jrc597XCM+b645LVRTPENXFY9XvvMBHL8jsJ3rVyrf
mA2odJQbsZutaYPaJm9KZ5ArF1I5K6MHtnP+dF+sAW0+M6GKnkrtBHVZ0Yn0WSmG
z06SiIeNAHo3zx0hyOxTx6tAQZz94lKravmtnvXDupLn2Uf09bfQdhv2aefDwvy2
CGJSDbpPfi3BRw2UnKKplvQGuUD9iQ5r2Zm1Q+l3eAijhIdNnyWfD5MxMU81NdPt
VwIDAQAB
-----END PUBLIC KEY-----";

    #[test]
    fn test_config_defaults() {
        // Set minimal required env vars
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("JWT_PRIVATE_KEY", TEST_PRIVATE_KEY);
        env::set_var("JWT_PUBLIC_KEY", TEST_PUBLIC_KEY);

        let config = AppConfig::from_env().unwrap();

        assert_eq!(config.app_host, "0.0.0.0");
        assert_eq!(config.app_port, 8080);
        assert!(config.metrics_enabled);
        assert!(!config.jwt_private_key.is_empty());
        assert!(!config.jwt_public_key.is_empty());
    }
}
