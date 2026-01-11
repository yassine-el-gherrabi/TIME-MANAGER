use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::models::Claims;

/// JWT configuration and utilities using RS256 algorithm
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    header: Header,
    access_token_expiry: i64,  // seconds
    refresh_token_expiry: i64, // seconds
}

impl JwtService {
    /// Create new JWT service from RSA PEM keys
    pub fn new(private_key_pem: &str, public_key_pem: &str) -> Result<Self, AppError> {
        let encoding_key = EncodingKey::from_rsa_pem(private_key_pem.as_bytes())
            .map_err(|e| AppError::ConfigError(format!("Invalid RSA private key: {}", e)))?;

        let decoding_key = DecodingKey::from_rsa_pem(public_key_pem.as_bytes())
            .map_err(|e| AppError::ConfigError(format!("Invalid RSA public key: {}", e)))?;

        let header = Header::new(Algorithm::RS256);

        Ok(Self {
            encoding_key,
            decoding_key,
            header,
            access_token_expiry: 15 * 60,           // 15 minutes
            refresh_token_expiry: 7 * 24 * 60 * 60, // 7 days
        })
    }

    /// Create JWT service from environment variables
    pub fn from_env() -> Result<Self, AppError> {
        let private_key = std::env::var("JWT_PRIVATE_KEY")
            .map_err(|_| AppError::ConfigError("JWT_PRIVATE_KEY not set".to_string()))?;

        let public_key = std::env::var("JWT_PUBLIC_KEY")
            .map_err(|_| AppError::ConfigError("JWT_PUBLIC_KEY not set".to_string()))?;

        Self::new(&private_key, &public_key)
    }

    /// Generate access token (15 minutes)
    pub fn generate_access_token(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        role: UserRole,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, org_id, role, self.access_token_expiry);

        encode(&self.header, &claims, &self.encoding_key)
            .map_err(|_| AppError::InternalError)
    }

    /// Generate refresh token (7 days)
    pub fn generate_refresh_token(
        &self,
        user_id: Uuid,
        org_id: Uuid,
        role: UserRole,
    ) -> Result<String, AppError> {
        let claims = Claims::new(user_id, org_id, role, self.refresh_token_expiry);

        encode(&self.header, &claims, &self.encoding_key)
            .map_err(|_e| AppError::InternalError)
    }

    /// Validate and decode token
    pub fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = true;

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
        let mut validation = Validation::new(Algorithm::RS256);
        validation.validate_exp = false;
        validation.required_spec_claims.clear();

        decode::<Claims>(token, &self.decoding_key, &validation)
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

    // Test RSA keys (2048-bit, for testing only)
    const TEST_PRIVATE_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQCfwD3rj7NWw0cH
su5hRvc5uo4lPhjFgx1HlZvxzt1pWafbr9zC1exr5HB/NxH1gN37e2nWOI8tPQWf
GsZecR2kGOc2LL3wWiBI9OXxZutf0mtzn3tcIz5vrjktVFM8Q1cVj1e+8wEcvyOw
netXKt+YDah0lBuxm61pg9omb0pnkCsXUjkrowe2c/50X6wBbT4zoYqeSu0EdVnR
ifRZKYbPTpKIh40AejfPHSHI7FPHq0BBnP3iUqtq+a2e9cO6kufZR/T1t9B2G/Zp
58PC/LYIYlINuk9+LcFHDZScoqmW9Aa5QP2JDmvZmbVD6Xd4CKOEh02fJZ8PkzEx
TzU10+1XAgMBAAECggEAJaFFlK7pWjcujKA36b8rLjSFFj293QypAXs63CdT3WSK
l0OiN1znz3RkkXrZ5qAf6gSkphr1kvzsTZGjh4ySpFxfXlIEvdClCTpyzb3mFNC+
keJPzyDYLLt36XcTEj90jHYS/75DFU/q6sgQLxzAxZL2Ctv2eAxJOXEfGm2ds64Q
9OYc/SnQQkpCYRLygfix93n2FlualLDuCZzlXBn/Usb8UzqylMrjzPUe7popIQ3+
QY6oJIgE3aeTBW1kfRgGK7fOcfJZY9q/M0mfAY8Zf3SxT1PTVTSFhunIDxo3Ay0K
XT+r9+YSyJ/0OycR7NsZSOifIwzBGOu/LAEGGA5wiQKBgQDb6J3R4IY55OEJpxJ/
pTwJdsEVmt5L/xoti029rMkwoEb5awBcK0bdQ06oJOHRInb5KTLFvZwhCWBSFyhC
FipnQXH5JRW8CNjlt7SGQZs5C7OJFxclAqfx0ba/oUzTyQ6ZfO1QBHNkx9XIX/DW
t/sEQ6xPWj5kcX1HxcwReCkiPwKBgQC5+B7gYLtSdt1gwHG4iZwTfo0AiZPdiSH7
kcN5JXWdJ4VP5dmtfuL3UOWRnbitfgIeBti//Po+Cd4h8i0CYFF20luOlj1Q4HH4
JPc61SGoykRs8a1DKFHm2YltWShHn5y3x5tarSzY38ndTPx/r1hvFoEHnF8+97gi
J49ozse+6QKBgD8dBtZqYvuQpcl4asW5rX5l18qUlQIop+G0Xk52nZNYHKaOwB6z
yPXN0HBPjYPRKWYfHdREs9+DamKFBOfaprbVwJkpvJAn1eAwFh6GC7+WjSNmPh1A
IuUzNAjRiVQrGwaQJSfW7ytYcxG7/0oQqXky1uw7UTbQn40Oxp+o5d1PAoGBAJh0
Peu3oRkjdKyCVzfvJ9IbZsBQCLYOW5t+jX7dJKQm5/Tt+xtt7+bLnMdZQzKHIHk5
J6uMWiFNuZqejCNsjpwYKxKjO7T3qrbApyTF4Igc+SdOoLlzbmEPaMgJ1SmSQcmv
iz40xZUtMLGJEV4jgx3elvyERti5/2uQftJu4fUxAoGBAMni474jFdHfz0WtHm/d
hTUmXvg1s9h033q0cqjT4CFHRi1JP8h7+Z8mYGa64+vgZFTl0c8+h27NGZdx33j7
T5Wb2QMgao7+BnKnHL2ymEIvaWhIbLXd7xTQsLBe4DvXQJmJCD3TeR6exrXK/lkI
/4D2c3OjbJmOwh4TOcI94I9Q
-----END PRIVATE KEY-----";
    const TEST_PUBLIC_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAn8A964+zVsNHB7LuYUb3
ObqOJT4YxYMdR5Wb8c7daVmn26/cwtXsa+RwfzcR9YDd+3tp1jiPLT0FnxrGXnEd
pBjnNiy98FogSPTl8WbrX9Jrc597XCM+b645LVRTPENXFY9XvvMBHL8jsJ3rVyrf
mA2odJQbsZutaYPaJm9KZ5ArF1I5K6MHtnP+dF+sAW0+M6GKnkrtBHVZ0Yn0WSmG
z06SiIeNAHo3zx0hyOxTx6tAQZz94lKravmtnvXDupLn2Uf09bfQdhv2aefDwvy2
CGJSDbpPfi3BRw2UnKKplvQGuUD9iQ5r2Zm1Q+l3eAijhIdNnyWfD5MxMU81NdPt
VwIDAQAB
-----END PUBLIC KEY-----";

    fn create_test_service() -> JwtService {
        JwtService::new(TEST_PRIVATE_KEY, TEST_PUBLIC_KEY).unwrap()
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
    fn test_invalid_private_key() {
        let result = JwtService::new("invalid-key", TEST_PUBLIC_KEY);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_public_key() {
        let result = JwtService::new(TEST_PRIVATE_KEY, "invalid-key");
        assert!(result.is_err());
    }
}
