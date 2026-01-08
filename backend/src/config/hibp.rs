use anyhow::Result;
use std::env;

/// HIBP (Have I Been Pwned) configuration for password breach checking
#[derive(Debug, Clone)]
pub struct HibpConfig {
    pub enabled: bool,
    pub api_url: String,
    pub timeout_seconds: u64,
    pub min_breach_threshold: u32,
}

impl HibpConfig {
    pub fn from_env() -> Result<Self> {
        let enabled = env::var("HIBP_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let api_url = env::var("HIBP_API_URL")
            .unwrap_or_else(|_| "https://api.pwnedpasswords.com".to_string());

        let timeout_seconds = env::var("HIBP_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .unwrap_or(5);

        let min_breach_threshold = env::var("HIBP_MIN_BREACH_THRESHOLD")
            .unwrap_or_else(|_| "1".to_string())
            .parse::<u32>()
            .unwrap_or(1);

        Ok(Self {
            enabled,
            api_url,
            timeout_seconds,
            min_breach_threshold,
        })
    }

    /// Create a disabled HIBP config (for testing)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            api_url: "https://api.pwnedpasswords.com".to_string(),
            timeout_seconds: 5,
            min_breach_threshold: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hibp_config_defaults() {
        env::remove_var("HIBP_ENABLED");
        env::remove_var("HIBP_API_URL");

        let config = HibpConfig::from_env().unwrap();

        assert!(!config.enabled);
        assert_eq!(config.api_url, "https://api.pwnedpasswords.com");
        assert_eq!(config.timeout_seconds, 5);
        assert_eq!(config.min_breach_threshold, 1);
    }

    #[test]
    fn test_hibp_config_disabled() {
        let config = HibpConfig::disabled();
        assert!(!config.enabled);
    }
}
