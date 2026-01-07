use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::NewLoginAttempt;
use crate::repositories::login_attempt_repository::LoginAttemptRepository;
use crate::repositories::user_repository::UserRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Constants for brute force protection
const MAX_FAILED_ATTEMPTS: i32 = 5;
const LOCKOUT_DURATION_MINUTES: i64 = 15;
const ATTEMPT_WINDOW_MINUTES: i64 = 15;

/// Brute force protection service
pub struct BruteForceService {
    user_repo: UserRepository,
    attempt_repo: LoginAttemptRepository,
}

impl BruteForceService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            attempt_repo: LoginAttemptRepository::new(pool),
        }
    }

    /// Record a login attempt
    pub async fn record_attempt(
        &self,
        email: &str,
        ip_address: &str,
        successful: bool,
    ) -> Result<(), AppError> {
        let new_attempt = NewLoginAttempt {
            email: email.to_string(),
            ip_address: ip_address.to_string(),
            successful,
        };

        self.attempt_repo.record(new_attempt).await?;
        Ok(())
    }

    /// Check if IP address is rate limited
    pub async fn is_ip_rate_limited(&self, ip_address: &str) -> Result<bool, AppError> {
        let attempts = self
            .attempt_repo
            .count_failed_attempts_for_ip(ip_address, ATTEMPT_WINDOW_MINUTES)
            .await?;

        Ok(attempts >= MAX_FAILED_ATTEMPTS as i64)
    }

    /// Check if email is rate limited
    pub async fn is_email_rate_limited(&self, email: &str) -> Result<bool, AppError> {
        let attempts = self
            .attempt_repo
            .count_failed_attempts_for_email(email, ATTEMPT_WINDOW_MINUTES)
            .await?;

        Ok(attempts >= MAX_FAILED_ATTEMPTS as i64)
    }

    /// Check and lock account if too many failed attempts
    pub async fn check_and_lock_account(&self, user_id: Uuid) -> Result<(), AppError> {
        let failed_attempts = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .failed_login_attempts;

        if failed_attempts >= MAX_FAILED_ATTEMPTS {
            let locked_until = chrono::Utc::now().naive_utc()
                + chrono::Duration::minutes(LOCKOUT_DURATION_MINUTES);

            self.user_repo.lock_account(user_id, locked_until).await?;

            return Err(AppError::Unauthorized(format!(
                "Account locked due to {} failed attempts. Try again in {} minutes",
                MAX_FAILED_ATTEMPTS, LOCKOUT_DURATION_MINUTES
            )));
        }

        Ok(())
    }

    /// Validate rate limits before login attempt
    pub async fn validate_rate_limits(
        &self,
        email: &str,
        ip_address: Option<&str>,
    ) -> Result<(), AppError> {
        // Check email rate limit
        if self.is_email_rate_limited(email).await? {
            return Err(AppError::TooManyRequests(
                "Too many failed login attempts for this email".to_string(),
            ));
        }

        // Check IP rate limit if provided
        if let Some(ip) = ip_address {
            if self.is_ip_rate_limited(ip).await? {
                return Err(AppError::TooManyRequests(
                    "Too many failed login attempts from this IP address".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Get recent failed attempts for email
    pub async fn get_failed_attempts_count(
        &self,
        email: &str,
        minutes: i64,
    ) -> Result<i64, AppError> {
        self.attempt_repo
            .count_failed_attempts_for_email(email, minutes)
            .await
    }

    /// Clean up old login attempts (older than 30 days)
    pub async fn cleanup_old_attempts(&self) -> Result<usize, AppError> {
        self.attempt_repo.delete_older_than(30).await
    }

    /// Get lockout duration in minutes
    pub fn get_lockout_duration_minutes(&self) -> i64 {
        LOCKOUT_DURATION_MINUTES
    }

    /// Get max allowed failed attempts
    pub fn get_max_failed_attempts(&self) -> i32 {
        MAX_FAILED_ATTEMPTS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_are_reasonable() {
        // Verify constants are within reasonable bounds
        assert!(MAX_FAILED_ATTEMPTS >= 3);
        assert!(MAX_FAILED_ATTEMPTS <= 10);

        assert!(LOCKOUT_DURATION_MINUTES >= 5);
        assert!(LOCKOUT_DURATION_MINUTES <= 60);

        assert!(ATTEMPT_WINDOW_MINUTES >= 5);
        assert!(ATTEMPT_WINDOW_MINUTES <= 60);
    }

    #[test]
    fn test_lockout_duration_getter() {
        // Test the constant directly since getters just return the constant
        assert_eq!(LOCKOUT_DURATION_MINUTES, 15);
    }

    #[test]
    fn test_max_attempts_getter() {
        // Test the constant directly since getters just return the constant
        assert_eq!(MAX_FAILED_ATTEMPTS, 5);
    }
}
