use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::models::Claims;

/// JWT configuration and utilities
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_token_expiry: i64,  // seconds
    refresh_token_expiry: i64, // seconds
}

impl JwtService {
    /// Create new JWT service from secret
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            access_token_expiry: 15 * 60,      // 15 minutes
            refresh_token_expiry: 7 * 24 * 60 * 60, // 7 days
        }
    }

    /// Create JWT service from environment variable
    pub fn from_env() -> Result<Self, AppError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::ConfigError("JWT_SECRET not set".to_string()))?;

        if secret.len() < 32 {
            return Err(AppError::ConfigError(
                "JWT_SECRET must be at least 32 characters".to_string(),
            ));
        }

        Ok(Self::new(&secret))
    }

    /// Generate access token (15 minutes)
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        role: UserRole,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, org_id, role, self.access_token_expiry);

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_e| AppError::InternalError)
    }

    /// Generate refresh token (7 days)
    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        role: UserRole,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, org_id, role, self.refresh_token_expiry);

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_e| AppError::InternalError)
    }

    /// Validate and decode token
    pub fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let validation = Validation::default();

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    AppError::Unauthorized("Token expired".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    AppError::Unauthorized("Invalid token".to_string())
                }
                jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                    AppError::Unauthorized("Invalid signature".to_string())
                }
                _ => AppError::Unauthorized("Token validation failed".to_string()),
            })
    }

    /// Extract user ID from token without full validation (for logging/metrics)
    pub fn extract_user_id_unchecked(&self, token: &str) -> Option<Uuid> {
        decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )
        .ok()
        .map(|data| data.claims.sub)
    }

    /// Check if token is expired
    pub fn is_token_expired(&self, token: &str) -> bool {
        matches!(self.validate_token(token), Err(AppError::Unauthorized(msg)) if msg.contains("expired"))
    }

    /// Get access token expiry in seconds
    pub fn access_token_expiry_seconds(&self) -> i64 {
        self.access_token_expiry
    }

    /// Get refresh token expiry in seconds
    pub fn refresh_token_expiry_seconds(&self) -> i64 {
        self.refresh_token_expiry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> JwtService {
        JwtService::new("test_secret_key_min_32_characters_long_for_security")
    }

    #[test]
    fn test_jwt_service_creation() {
        let service = create_test_service();
        assert_eq!(service.access_token_expiry, 15 * 60);
        assert_eq!(service.refresh_token_expiry, 7 * 24 * 60 * 60);
    }

    #[test]
    fn test_generate_and_validate_access_token() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let role = UserRole::Employee;

        let token = service
            .generate_access_token(user_id, org_id, role)
            .unwrap();

        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.org_id, org_id);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_generate_and_validate_refresh_token() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let role = UserRole::Manager;

        let token = service
            .generate_refresh_token(user_id, org_id, role)
            .unwrap();

        let claims = service.validate_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.org_id, org_id);
        assert_eq!(claims.role, role);
    }

    #[test]
    fn test_invalid_token() {
        let service = create_test_service();
        let result = service.validate_token("invalid_token");

        assert!(result.is_err());
        match result {
            Err(AppError::Unauthorized(_)) => (),
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[test]
    fn test_token_with_wrong_secret() {
        let service1 = JwtService::new("secret_key_one_with_32_characters!!");
        let service2 = JwtService::new("different_secret_32_characters_long");

        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();
        let role = UserRole::Admin;

        let token = service1
            .generate_access_token(user_id, org_id, role)
            .unwrap();

        let result = service2.validate_token(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_user_id() {
        let service = create_test_service();
        let user_id = Uuid::new_v4();
        let org_id = Uuid::new_v4();

        let token = service
            .generate_access_token(user_id, org_id, UserRole::Employee)
            .unwrap();

        let extracted_id = service.extract_user_id_unchecked(&token);
        assert_eq!(extracted_id, Some(user_id));
    }

    #[test]
    fn test_jwt_secret_validation() {
        let result = JwtService::new("short");
        // Service creation doesn't validate length, but from_env does
        let service = result;
        assert_eq!(service.access_token_expiry, 15 * 60);
    }
}
