use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{NewInviteToken, NewPasswordHistory, TokenPair};
use crate::repositories::password_history_repository::PasswordHistoryRepository;
use crate::repositories::{InviteTokenRepository, UserRepository};
use crate::utils::{JwtService, PasswordService};

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Service for handling user invitation workflow
pub struct InviteService {
    user_repo: UserRepository,
    invite_token_repo: InviteTokenRepository,
    password_history_repo: PasswordHistoryRepository,
    password_service: PasswordService,
    jwt_service: JwtService,
}

impl InviteService {
    pub fn new(pool: DbPool, jwt_service: JwtService) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            invite_token_repo: InviteTokenRepository::new(pool.clone()),
            password_history_repo: PasswordHistoryRepository::new(pool),
            password_service: PasswordService::new(),
            jwt_service,
        }
    }

    /// Create an invite token for a user (called when admin creates a user)
    /// Returns the raw token that should be sent via email
    pub async fn create_invite_token(&self, user_id: Uuid) -> Result<String, AppError> {
        // Invalidate any existing invite tokens for this user
        self.invite_token_repo
            .invalidate_all_for_user(user_id)
            .await?;

        // Generate invite token (32 random bytes as hex)
        let invite_token = generate_invite_token();

        // Hash the token for storage
        let token_hash = hash_token(&invite_token);

        // Set expiry (7 days from now)
        let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::days(7);

        // Create invite token record
        let new_token = NewInviteToken {
            user_id,
            token_hash,
            expires_at,
        };

        self.invite_token_repo.create(new_token).await?;

        Ok(invite_token)
    }

    /// Accept invitation - set password and activate account
    /// Returns JWT tokens for auto-login
    pub async fn accept_invite(
        &self,
        invite_token: &str,
        new_password: &str,
    ) -> Result<TokenPair, AppError> {
        // Validate password strength
        self.password_service
            .validate_password_strength(new_password)?;

        // Hash the token to look it up
        let token_hash = hash_token(invite_token);

        // Find and validate invite token
        let stored_token = self
            .invite_token_repo
            .find_by_token_hash(&token_hash)
            .await?;

        // Check if token is already used
        if stored_token.used_at.is_some() {
            return Err(AppError::Unauthorized(
                "Invite token has already been used".to_string(),
            ));
        }

        // Check if token is expired
        if stored_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::Unauthorized(
                "Invite token has expired. Please contact your administrator.".to_string(),
            ));
        }

        // Hash the new password
        let password_hash = self.password_service.hash_password(new_password)?;

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

        // Mark invite token as used
        self.invite_token_repo.mark_as_used(stored_token.id).await?;

        // Get user for token generation
        let user = self.user_repo.find_by_id(stored_token.user_id).await?;

        // Generate JWT tokens for auto-login
        let access_token =
            self.jwt_service
                .generate_access_token(user.id, user.organization_id, user.role)?;

        let refresh_token =
            self.jwt_service
                .generate_refresh_token(user.id, user.organization_id, user.role)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    /// Verify invite token is valid (without using it)
    pub async fn verify_invite_token(&self, invite_token: &str) -> Result<Uuid, AppError> {
        let token_hash = hash_token(invite_token);
        let stored_token = self
            .invite_token_repo
            .find_by_token_hash(&token_hash)
            .await?;

        if stored_token.used_at.is_some() {
            return Err(AppError::Unauthorized("Token already used".to_string()));
        }

        if stored_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::Unauthorized("Token expired".to_string()));
        }

        Ok(stored_token.user_id)
    }

    /// Resend invite - invalidate old token and create new one
    pub async fn resend_invite(&self, user_id: Uuid) -> Result<String, AppError> {
        // Check user exists
        self.user_repo.find_by_id(user_id).await?;

        // Create new invite token (this invalidates old ones)
        self.create_invite_token(user_id).await
    }
}

/// Generate a random invite token
fn generate_invite_token() -> String {
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
    fn test_generate_invite_token() {
        let token1 = generate_invite_token();
        let token2 = generate_invite_token();

        // Tokens should be different
        assert_ne!(token1, token2);

        // Tokens should be 64 characters (32 bytes as hex)
        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);
    }

    #[test]
    fn test_hash_token_consistency() {
        let token = "test_invite_token";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);

        // Same token should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be 64 characters (SHA-256 in hex)
        assert_eq!(hash1.len(), 64);
    }
}
