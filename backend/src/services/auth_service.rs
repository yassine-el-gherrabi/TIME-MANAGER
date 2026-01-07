use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::models::{NewRefreshToken, TokenPair};
use crate::repositories::refresh_token_repository::RefreshTokenRepository;
use crate::repositories::user_repository::{User, UserRepository};
use crate::utils::{JwtService, PasswordService};

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Authentication service for user registration, login, and token management
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

    /// Register a new user
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        _first_name: &str,
        _last_name: &str,
        _organization_id: Uuid,
        _role: UserRole,
    ) -> Result<TokenPair, AppError> {
        // Validate password strength
        self.password_service.validate_password_strength(password)?;

        // Check if user already exists
        if self.user_repo.find_by_email(email).await.is_ok() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Hash password
        let _password_hash = self.password_service.hash_password(password)?;

        // Create user (this would use a NewUser model in real implementation)
        // For now, we'll assume the user is created and return their ID
        // In a real implementation, you'd have a create method in UserRepository

        // Generate tokens (using placeholder user_id for now)
        // In real implementation, this would use the created user's ID
        let user = self.user_repo.find_by_email(email).await?;

        self.generate_token_pair(user.id, user.organization_id, user.role)
            .await
    }

    /// Login user with email and password
    pub async fn login(&self, email: &str, password: &str) -> Result<TokenPair, AppError> {
        // Find user by email
        let user = self.user_repo.find_by_email(email).await?;

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
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // Reset failed attempts on successful login
        self.user_repo.reset_failed_attempts(user.id).await?;

        // Check if password is expired
        if self.user_repo.is_password_expired(user.id).await? {
            return Err(AppError::Unauthorized(
                "Password has expired, please reset your password".to_string(),
            ));
        }

        // Generate token pair
        self.generate_token_pair(user.id, user.organization_id, user.role)
            .await
    }

    /// Refresh access token using refresh token
    pub async fn refresh(&self, refresh_token: &str) -> Result<TokenPair, AppError> {
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

        // Generate new token pair
        let new_token_pair = self
            .generate_token_pair(claims.sub, claims.org_id, claims.role)
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

    /// Generate access and refresh token pair
    async fn generate_token_pair(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        role: UserRole,
    ) -> Result<TokenPair, AppError> {
        // Generate access token
        let access_token = self
            .jwt_service
            .generate_access_token(user_id, org_id, role)?;

        // Generate refresh token
        let refresh_token = self
            .jwt_service
            .generate_refresh_token(user_id, org_id, role)?;

        // Store refresh token in database
        let token_hash = hash_token(&refresh_token);
        let expires_at = chrono::Utc::now().naive_utc()
            + chrono::Duration::seconds(self.jwt_service.refresh_token_expiry_seconds());

        let new_refresh_token = NewRefreshToken {
            user_id,
            token_hash,
            expires_at,
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
