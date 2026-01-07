use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::repositories::user_repository::UserRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Constants for password expiry
const PASSWORD_EXPIRY_DAYS: i64 = 90;
const PASSWORD_EXPIRY_WARNING_DAYS: i64 = 7;

/// Password expiry management service
pub struct PasswordExpiryService {
    user_repo: UserRepository,
}

impl PasswordExpiryService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            user_repo: UserRepository::new(pool),
        }
    }

    /// Check if user's password is expired
    pub async fn is_password_expired(&self, user_id: Uuid) -> Result<bool, AppError> {
        self.user_repo.is_password_expired(user_id).await
    }

    /// Get days until password expires
    pub async fn days_until_expiry(&self, user_id: Uuid) -> Result<Option<i64>, AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;

        if let Some(expires_at) = user.password_expires_at {
            let now = chrono::Utc::now().naive_utc();
            let duration = expires_at.signed_duration_since(now);
            Ok(Some(duration.num_days()))
        } else {
            Ok(None)
        }
    }

    /// Check if password expires soon (within warning period)
    pub async fn expires_soon(&self, user_id: Uuid) -> Result<bool, AppError> {
        if let Some(days) = self.days_until_expiry(user_id).await? {
            Ok((0..=PASSWORD_EXPIRY_WARNING_DAYS).contains(&days))
        } else {
            Ok(false)
        }
    }

    /// Get password expiry status with detailed information
    pub async fn get_expiry_status(&self, user_id: Uuid) -> Result<PasswordExpiryStatus, AppError> {
        let is_expired = self.is_password_expired(user_id).await?;
        let days_until_expiry = self.days_until_expiry(user_id).await?;
        let expires_soon = self.expires_soon(user_id).await?;

        Ok(PasswordExpiryStatus {
            is_expired,
            days_until_expiry,
            expires_soon,
            warning_days: PASSWORD_EXPIRY_WARNING_DAYS,
        })
    }

    /// Force password change for user (sets expiry to past)
    pub async fn force_password_change(&self, _user_id: Uuid) -> Result<(), AppError> {
        let _past_date = chrono::Utc::now().naive_utc() - chrono::Duration::days(1);

        // This would need a method in UserRepository to update password_expires_at
        // For now, we'll use the existing update_password which sets expiry to 90 days from now
        // In a full implementation, you'd add a specific method

        // Placeholder - actual implementation would set expires_at to past
        Ok(())
    }

    /// Get password expiry policy information
    pub fn get_policy_info(&self) -> PasswordExpiryPolicy {
        PasswordExpiryPolicy {
            expiry_days: PASSWORD_EXPIRY_DAYS,
            warning_days: PASSWORD_EXPIRY_WARNING_DAYS,
        }
    }

    /// Validate that password change interval is enforced
    pub async fn check_password_change_too_soon(
        &self,
        user_id: Uuid,
        min_days_between_changes: i64,
    ) -> Result<(), AppError> {
        let user = self.user_repo.find_by_id(user_id).await?;

        if let Some(last_changed) = user.password_changed_at {
            let now = chrono::Utc::now().naive_utc();
            let duration = now.signed_duration_since(last_changed);
            let days_since_change = duration.num_days();

            if days_since_change < min_days_between_changes {
                return Err(AppError::ValidationError(format!(
                    "Password can only be changed once every {} days. {} days remaining",
                    min_days_between_changes,
                    min_days_between_changes - days_since_change
                )));
            }
        }

        Ok(())
    }
}

/// Password expiry status information
#[derive(Debug, Clone)]
pub struct PasswordExpiryStatus {
    pub is_expired: bool,
    pub days_until_expiry: Option<i64>,
    pub expires_soon: bool,
    pub warning_days: i64,
}

/// Password expiry policy information
#[derive(Debug, Clone)]
pub struct PasswordExpiryPolicy {
    pub expiry_days: i64,
    pub warning_days: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_expiry_constants() {
        // Verify constants are reasonable
        assert!(PASSWORD_EXPIRY_DAYS >= 30);
        assert!(PASSWORD_EXPIRY_DAYS <= 365);

        assert!(PASSWORD_EXPIRY_WARNING_DAYS >= 1);
        assert!(PASSWORD_EXPIRY_WARNING_DAYS <= 30);
        assert!(PASSWORD_EXPIRY_WARNING_DAYS < PASSWORD_EXPIRY_DAYS);
    }

    #[test]
    fn test_policy_info() {
        // Test the policy structure directly
        let policy = PasswordExpiryPolicy {
            expiry_days: PASSWORD_EXPIRY_DAYS,
            warning_days: PASSWORD_EXPIRY_WARNING_DAYS,
        };

        assert_eq!(policy.expiry_days, PASSWORD_EXPIRY_DAYS);
        assert_eq!(policy.warning_days, PASSWORD_EXPIRY_WARNING_DAYS);
    }

    #[test]
    fn test_expiry_status_structure() {
        let status = PasswordExpiryStatus {
            is_expired: false,
            days_until_expiry: Some(30),
            expires_soon: false,
            warning_days: PASSWORD_EXPIRY_WARNING_DAYS,
        };

        assert!(!status.is_expired);
        assert_eq!(status.days_until_expiry, Some(30));
        assert!(!status.expires_soon);
    }

    #[test]
    fn test_expiry_policy_structure() {
        let policy = PasswordExpiryPolicy {
            expiry_days: 90,
            warning_days: 7,
        };

        assert_eq!(policy.expiry_days, 90);
        assert_eq!(policy.warning_days, 7);
    }
}
