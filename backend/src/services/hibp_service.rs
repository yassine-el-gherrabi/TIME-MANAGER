use crate::config::hibp::HibpConfig;
use crate::error::AppError;
use sha1::{Digest, Sha1};
use std::time::Duration;

/// Service for checking passwords against the Have I Been Pwned database
/// Uses the k-Anonymity model to protect privacy
pub struct HibpService {
    config: HibpConfig,
    client: Option<reqwest::Client>,
}

impl HibpService {
    /// Create a new HIBP service instance
    pub fn new(config: HibpConfig) -> Self {
        let client = if config.enabled {
            Some(
                reqwest::Client::builder()
                    .timeout(Duration::from_secs(config.timeout_seconds))
                    .build()
                    .unwrap_or_default(),
            )
        } else {
            None
        };

        Self { config, client }
    }

    /// Check if a password has been found in known data breaches
    /// Uses the k-Anonymity model: only the first 5 characters of the SHA-1 hash are sent
    /// Returns the number of times the password was found in breaches, or 0 if not found
    pub async fn check_password(&self, password: &str) -> Result<u32, AppError> {
        if !self.config.enabled {
            tracing::debug!("HIBP service disabled, skipping password check");
            return Ok(0);
        }

        let client = match &self.client {
            Some(c) => c,
            None => return Ok(0),
        };

        // SHA-1 hash the password
        let mut hasher = Sha1::new();
        hasher.update(password.as_bytes());
        let hash = hex::encode(hasher.finalize()).to_uppercase();

        // Split into prefix (first 5 chars) and suffix (remaining 35 chars)
        let (prefix, suffix) = hash.split_at(5);

        // Query the HIBP API with only the prefix
        let url = format!("{}/range/{}", self.config.api_url, prefix);

        let response = client
            .get(&url)
            .header("Add-Padding", "true")
            .send()
            .await
            .map_err(|e| {
                tracing::warn!("HIBP API request failed: {}", e);
                AppError::InternalError
            })?;

        if !response.status().is_success() {
            tracing::warn!("HIBP API returned status: {}", response.status());
            return Ok(0); // Fail open - don't block user if API is down
        }

        let body = response.text().await.map_err(|e| {
            tracing::warn!("Failed to read HIBP response: {}", e);
            AppError::InternalError
        })?;

        // Search for our suffix in the response
        for line in body.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let response_suffix = parts[0].trim();
                if response_suffix == suffix {
                    if let Ok(count) = parts[1].trim().parse::<u32>() {
                        return Ok(count);
                    }
                }
            }
        }

        Ok(0)
    }

    /// Check if a password is compromised (breach count >= threshold)
    pub async fn is_password_compromised(&self, password: &str) -> Result<bool, AppError> {
        let breach_count = self.check_password(password).await?;
        Ok(breach_count >= self.config.min_breach_threshold)
    }

    /// Validate password is not compromised, returning an error if it is
    pub async fn validate_not_compromised(&self, password: &str) -> Result<(), AppError> {
        if !self.config.enabled {
            return Ok(());
        }

        let breach_count = self.check_password(password).await?;

        if breach_count >= self.config.min_breach_threshold {
            tracing::info!(
                "Password rejected - found in {} data breaches",
                breach_count
            );
            return Err(AppError::ValidationError(format!(
                "This password has been found in {} data breach(es). Please choose a different password.",
                breach_count
            )));
        }

        Ok(())
    }

    /// Check if HIBP service is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hibp_service_disabled() {
        let config = HibpConfig::disabled();
        let service = HibpService::new(config);
        assert!(!service.is_enabled());
    }

    #[test]
    fn test_sha1_hash_format() {
        // Test that SHA-1 hashing produces expected format
        let mut hasher = Sha1::new();
        hasher.update(b"password");
        let hash = hex::encode(hasher.finalize()).to_uppercase();

        // SHA-1 produces 40-character hex string
        assert_eq!(hash.len(), 40);
        // Known SHA-1 hash of "password"
        assert_eq!(hash, "5BAA61E4C9B93F3F0682250B6CF8331B7EE68FD8");
    }

    #[tokio::test]
    async fn test_check_password_disabled() {
        let config = HibpConfig::disabled();
        let service = HibpService::new(config);

        // When disabled, should return 0
        let result = service.check_password("password123").await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_validate_not_compromised_disabled() {
        let config = HibpConfig::disabled();
        let service = HibpService::new(config);

        // When disabled, should always succeed
        let result = service.validate_not_compromised("password123").await;
        assert!(result.is_ok());
    }
}
