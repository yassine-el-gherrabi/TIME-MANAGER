use anyhow::Result;
use std::env;

/// Email configuration for SMTP-based email sending
#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub from_email: String,
    pub from_name: String,
    pub frontend_url: String,
    pub enabled: bool,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self> {
        let enabled = env::var("EMAIL_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());

        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "1025".to_string())
            .parse::<u16>()
            .unwrap_or(1025);

        let smtp_username = env::var("SMTP_USERNAME").ok().filter(|s| !s.is_empty());
        let smtp_password = env::var("SMTP_PASSWORD").ok().filter(|s| !s.is_empty());

        let from_email =
            env::var("EMAIL_FROM").unwrap_or_else(|_| "noreply@timemanager.local".to_string());

        let from_name = env::var("EMAIL_FROM_NAME").unwrap_or_else(|_| "Time Manager".to_string());

        let frontend_url =
            env::var("FRONTEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

        Ok(Self {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            from_email,
            from_name,
            frontend_url,
            enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_config_defaults() {
        // Clear environment variables
        env::remove_var("SMTP_HOST");
        env::remove_var("SMTP_PORT");
        env::remove_var("EMAIL_ENABLED");

        let config = EmailConfig::from_env().unwrap();

        assert_eq!(config.smtp_host, "localhost");
        assert_eq!(config.smtp_port, 1025);
        assert!(!config.enabled);
    }
}
