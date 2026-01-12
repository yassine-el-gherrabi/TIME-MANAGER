use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{NewPasswordHistory, NewPasswordResetToken};
use crate::repositories::password_history_repository::PasswordHistoryRepository;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::user_repository::UserRepository;
use crate::utils::PasswordService;

/// Password reset and change service
pub struct PasswordResetService {
    user_repo: UserRepository,
    reset_token_repo: PasswordResetRepository,
    password_history_repo: PasswordHistoryRepository,
    password_service: PasswordService,
}

impl PasswordResetService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            reset_token_repo: PasswordResetRepository::new(pool.clone()),
            password_history_repo: PasswordHistoryRepository::new(pool),
            password_service: PasswordService::new(),
        }
    }

    /// Request password reset - generates token and returns it (would be emailed in real app)
    /// Returns Option<String> to allow silent failure when user doesn't exist (prevents user enumeration)
    pub async fn request_reset(&self, email: &str) -> Result<Option<String>, AppError> {
        // Find user by email - handle not found case to prevent user enumeration
        let user = match self.user_repo.find_by_email(email).await {
            Ok(u) => Some(u),
            Err(AppError::NotFound(_)) => None,
            Err(e) => return Err(e),
        };

        // Generate token ONLY if user exists, but always return same response
        if let Some(user) = user {
            // Generate reset token (32 random bytes as hex)
            let reset_token = generate_reset_token();

            // Hash the token for storage
            let token_hash = hash_token(&reset_token);

            // Set expiry (1 hour from now)
            let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(1);

            // Create reset token record
            let new_token = NewPasswordResetToken {
                user_id: user.id,
                token_hash: token_hash.clone(),
                expires_at,
            };

            self.reset_token_repo.create(new_token).await?;

            // In a real application, send email with reset_token here
            // For now, return the token (in production, this would be emailed)
            Ok(Some(reset_token))
        } else {
            // User doesn't exist - return None but don't reveal this to caller
            Ok(None)
        }
    }

    /// Reset password using reset token
    pub async fn reset_password(
        &self,
        reset_token: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // Validate password strength
        self.password_service
            .validate_password_strength(new_password)?;

        // Hash the token to look it up
        let token_hash = hash_token(reset_token);

        // Find and validate reset token
        let stored_token = self
            .reset_token_repo
            .find_by_token_hash(&token_hash)
            .await?;

        // Check if token is already used
        if stored_token.used_at.is_some() {
            return Err(AppError::Unauthorized(
                "Reset token has already been used".to_string(),
            ));
        }

        // Check if token is expired
        if stored_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::Unauthorized(
                "Reset token has expired".to_string(),
            ));
        }

        // Check password history (prevent reuse of last 5 passwords)
        let password_hash = self.password_service.hash_password(new_password)?;
        let was_used = self
            .password_history_repo
            .was_recently_used(stored_token.user_id, &password_hash, 5)
            .await?;

        if was_used {
            return Err(AppError::ValidationError(
                "Password cannot be one of your last 5 passwords".to_string(),
            ));
        }

        // Update user password
        self.user_repo
            .update_password(stored_token.user_id, &password_hash)
            .await?;

        // Add to password history
        let new_history = NewPasswordHistory {
            user_id: stored_token.user_id,
            password_hash: password_hash.clone(),
        };
        self.password_history_repo.add(new_history).await?;

        // Mark token as used
        self.reset_token_repo.mark_as_used(stored_token.id).await?;

        // Unlock account if it was locked
        self.user_repo.unlock_account(stored_token.user_id).await?;

        Ok(())
    }

    /// Change password for authenticated user
    pub async fn change_password(
        &self,
        user_id: Uuid,
        current_password: &str,
        new_password: &str,
    ) -> Result<(), AppError> {
        // Validate new password strength
        self.password_service
            .validate_password_strength(new_password)?;

        // Get user
        let user = self.user_repo.find_by_id(user_id).await?;

        // Verify current password
        let is_valid = self
            .password_service
            .verify_password(current_password, &user.password_hash)?;

        if !is_valid {
            return Err(AppError::Unauthorized(
                "Current password is incorrect".to_string(),
            ));
        }

        // Check password history (prevent reuse of last 5 passwords)
        let new_password_hash = self.password_service.hash_password(new_password)?;
        let was_used = self
            .password_history_repo
            .was_recently_used(user_id, &new_password_hash, 5)
            .await?;

        if was_used {
            return Err(AppError::ValidationError(
                "Password cannot be one of your last 5 passwords".to_string(),
            ));
        }

        // Update password
        self.user_repo
            .update_password(user_id, &new_password_hash)
            .await?;

        // Add to password history
        let new_history = NewPasswordHistory {
            user_id,
            password_hash: new_password_hash,
        };
        self.password_history_repo.add(new_history).await?;

        Ok(())
    }

    /// Verify password reset token is valid
    pub async fn verify_reset_token(&self, reset_token: &str) -> Result<(), AppError> {
        let token_hash = hash_token(reset_token);
        let stored_token = self
            .reset_token_repo
            .find_by_token_hash(&token_hash)
            .await?;

        if stored_token.used_at.is_some() {
            return Err(AppError::Unauthorized("Token already used".to_string()));
        }

        if stored_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::Unauthorized("Token expired".to_string()));
        }

        Ok(())
    }
}

/// Generate a random reset token
fn generate_reset_token() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    hex::encode(bytes)
}

/// Hash a token using SHA-256
fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_reset_token() {
        let token1 = generate_reset_token();
        let token2 = generate_reset_token();

        // Tokens should be different
        assert_ne!(token1, token2);

        // Tokens should be 64 characters (32 bytes as hex)
        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);
    }

    #[test]
    fn test_hash_token_consistency() {
        let token = "test_reset_token";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);

        // Same token should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be 64 characters (SHA-256 in hex)
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_hash_token_uniqueness() {
        let token1 = "token_1";
        let token2 = "token_2";

        let hash1 = hash_token(token1);
        let hash2 = hash_token(token2);

        // Different tokens should produce different hashes
        assert_ne!(hash1, hash2);
    }
}
