use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::models::{NewRefreshToken, TokenPair};
use crate::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::user_repository::{User, UserRepository};
use crate::utils::{JwtService, PasswordService};

/// Authentication service for user login and token management
pub struct AuthService {
    user_repo: UserRepository,
    token_repo: RefreshTokenRepository,
    jwt_service: JwtService,
    password_service: PasswordService,
}

impl AuthService {
    pub fn new(pool: DbPool, jwt_service: JwtService) -> Self {
        Self {
            user_repo: UserRepository::new(pool.clone()),
            token_repo: RefreshTokenRepository::new(pool),
            jwt_service,
            password_service: PasswordService::new(),
        }
    }

    /// Login user with email and password
    pub async fn login(
        &self,
        email: &str,
        password: &str,
        user_agent: Option<String>,
    ) -> Result<TokenPair, AppError> {
        // Find user by email - handle not found case to prevent user enumeration
        let user = match self.user_repo.find_by_email(email).await {
            Ok(u) => u,
            Err(AppError::NotFound(_)) => {
                // Perform dummy password verification to prevent timing attacks
                // This ensures authentication always takes similar time regardless of user existence
                let dummy_hash = "$argon2id$v=19$m=19456,t=2,p=1$aGVsbG93b3JsZA$CksGSbFneCWPxbHEUWZUKAFoJL0pfeFnJXnkPv2FsVo";
                let _ = self.password_service.verify_password(password, dummy_hash);
                return Err(AppError::Unauthorized(
                    "Invalid email or password".to_string(),
                ));
            }
            Err(e) => return Err(e),
        };

        // Check if account is locked
        if self.user_repo.is_locked(user.id).await? {
            return Err(AppError::Unauthorized(
                "Account is locked due to too many failed attempts".to_string(),
            ));
        }

        // Verify password
        let is_valid = self
            .password_service
            .verify_password(password, &user.password_hash)?;

        if !is_valid {
            // Increment failed attempts
            self.user_repo.increment_failed_attempts(user.id).await?;
            return Err(AppError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        // Reset failed attempts on successful login
        self.user_repo.reset_failed_attempts(user.id).await?;

        // Check if password is expired
        if self.user_repo.is_password_expired(user.id).await? {
            return Err(AppError::Unauthorized(
                "Password has expired, please reset your password".to_string(),
            ));
        }

        // Revoke any existing sessions from the same device (user_agent)
        // This ensures only one session per device
        if let Some(ref ua) = user_agent {
            let _ = self
                .token_repo
                .revoke_by_user_agent_for_user(user.id, ua)
                .await;
        }

        // Generate token pair with session info
        self.generate_token_pair(user.id, user.organization_id, user.role, user_agent)
            .await
    }

    /// Refresh access token using refresh token
    pub async fn refresh(
        &self,
        refresh_token: &str,
        user_agent: Option<String>,
    ) -> Result<TokenPair, AppError> {
        // Validate refresh token
        let claims = self.jwt_service.validate_token(refresh_token)?;

        // Hash the token to look it up
        let token_hash = hash_token(refresh_token);

        // Find refresh token in database
        let stored_token = self.token_repo.find_by_token_hash(&token_hash).await?;

        // Check if token is revoked
        if stored_token.revoked_at.is_some() {
            return Err(AppError::Unauthorized("Token has been revoked".to_string()));
        }

        // Check if token is expired
        if stored_token.expires_at < chrono::Utc::now().naive_utc() {
            return Err(AppError::Unauthorized("Refresh token expired".to_string()));
        }

        // Generate new token pair with session info
        let new_token_pair = self
            .generate_token_pair(claims.sub, claims.org_id, claims.role, user_agent)
            .await?;

        // Revoke old refresh token
        self.token_repo.revoke(stored_token.id).await?;

        Ok(new_token_pair)
    }

    /// Logout user by revoking refresh token
    pub async fn logout(&self, refresh_token: &str) -> Result<(), AppError> {
        let token_hash = hash_token(refresh_token);
        let stored_token = self.token_repo.find_by_token_hash(&token_hash).await?;
        self.token_repo.revoke(stored_token.id).await
    }

    /// Logout from all devices by revoking all refresh tokens
    pub async fn logout_all(&self, user_id: Uuid) -> Result<(), AppError> {
        self.token_repo.revoke_all_for_user(user_id).await
    }

    /// Get user by ID (for "me" endpoint)
    pub async fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        self.user_repo.find_by_id(user_id).await
    }

    /// Get user by email (for login response)
    pub async fn get_user_by_email(&self, email: &str) -> Result<User, AppError> {
        self.user_repo.find_by_email(email).await
    }

    /// Get all active sessions for a user
    pub async fn get_active_sessions(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<crate::models::RefreshToken>, AppError> {
        self.token_repo.get_active_tokens_for_user(user_id).await
    }

    /// Revoke a specific session by token ID
    pub async fn revoke_session(&self, user_id: Uuid, session_id: Uuid) -> Result<(), AppError> {
        // First verify the session belongs to this user
        let sessions = self.token_repo.get_active_tokens_for_user(user_id).await?;
        let session = sessions
            .iter()
            .find(|s| s.id == session_id)
            .ok_or_else(|| AppError::NotFound("Session not found".to_string()))?;

        self.token_repo.revoke(session.id).await
    }

    /// Generate access and refresh token pair
    async fn generate_token_pair(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        role: UserRole,
        user_agent: Option<String>,
    ) -> Result<TokenPair, AppError> {
        // Generate access token
        let access_token = self
            .jwt_service
            .generate_access_token(user_id, org_id, role)?;

        // Generate refresh token
        let refresh_token = self
            .jwt_service
            .generate_refresh_token(user_id, org_id, role)?;

        // Store refresh token in database with session info
        let token_hash = hash_token(&refresh_token);
        let expires_at = chrono::Utc::now().naive_utc()
            + chrono::Duration::seconds(self.jwt_service.refresh_token_expiry_seconds());

        let new_refresh_token = NewRefreshToken {
            user_id,
            token_hash,
            expires_at,
            user_agent,
        };

        self.token_repo.create(new_refresh_token).await?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
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
    fn test_hash_token() {
        let token = "test_token_123";
        let hash1 = hash_token(token);
        let hash2 = hash_token(token);

        // Same token should produce same hash
        assert_eq!(hash1, hash2);

        // Hash should be 64 characters (SHA-256 in hex)
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_hash_token_different_inputs() {
        let token1 = "token_1";
        let token2 = "token_2";

        let hash1 = hash_token(token1);
        let hash2 = hash_token(token2);

        // Different tokens should produce different hashes
        assert_ne!(hash1, hash2);
    }
}
