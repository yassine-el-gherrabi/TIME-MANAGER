use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::error::AppError;

/// Password hashing and verification service using Argon2
pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl PasswordService {
    /// Create new password service with default Argon2 configuration
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    /// Hash a password using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String, AppError> {
        // Validate password length
        if password.is_empty() {
            return Err(AppError::ValidationError(
                "Password cannot be empty".to_string(),
            ));
        }

        if password.len() < 8 {
            return Err(AppError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if password.len() > 128 {
            return Err(AppError::ValidationError(
                "Password must not exceed 128 characters".to_string(),
            ));
        }

        // Generate salt
        let salt = SaltString::generate(&mut OsRng);

        // Hash password
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_e| AppError::InternalError)?;

        Ok(password_hash.to_string())
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        // Parse the stored hash
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AppError::InternalError)?;

        // Verify password
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(_) => Err(AppError::InternalError),
        }
    }

    /// Validate password strength
    pub fn validate_password_strength(&self, password: &str) -> Result<(), AppError> {
        if password.len() < 8 {
            return Err(AppError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if password.len() > 128 {
            return Err(AppError::ValidationError(
                "Password must not exceed 128 characters".to_string(),
            ));
        }

        // Check for at least one uppercase letter
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }

        // Check for at least one lowercase letter
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one lowercase letter".to_string(),
            ));
        }

        // Check for at least one digit
        if !password.chars().any(|c| c.is_numeric()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one digit".to_string(),
            ));
        }

        // Check for at least one special character
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        if !password.chars().any(|c| special_chars.contains(c)) {
            return Err(AppError::ValidationError(
                "Password must contain at least one special character".to_string(),
            ));
        }

        Ok(())
    }

    /// Check if password has been compromised (placeholder for future HIBP integration)
    pub fn is_password_compromised(&self, _password: &str) -> Result<bool, AppError> {
        // TODO: Integrate with Have I Been Pwned API
        Ok(false)
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_service() -> PasswordService {
        PasswordService::new()
    }

    #[test]
    fn test_hash_password() {
        let service = create_test_service();
        let password = "TestPassword123!";

        let hash = service.hash_password(password).unwrap();

        // Hash should not be empty
        assert!(!hash.is_empty());

        // Hash should start with $argon2
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_correct_password() {
        let service = create_test_service();
        let password = "TestPassword123!";

        let hash = service.hash_password(password).unwrap();
        let is_valid = service.verify_password(password, &hash).unwrap();

        assert!(is_valid);
    }

    #[test]
    fn test_verify_incorrect_password() {
        let service = create_test_service();
        let password = "TestPassword123!";
        let wrong_password = "WrongPassword123!";

        let hash = service.hash_password(password).unwrap();
        let is_valid = service.verify_password(wrong_password, &hash).unwrap();

        assert!(!is_valid);
    }

    #[test]
    fn test_hash_uniqueness() {
        let service = create_test_service();
        let password = "TestPassword123!";

        let hash1 = service.hash_password(password).unwrap();
        let hash2 = service.hash_password(password).unwrap();

        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(service.verify_password(password, &hash1).unwrap());
        assert!(service.verify_password(password, &hash2).unwrap());
    }

    #[test]
    fn test_empty_password() {
        let service = create_test_service();
        let result = service.hash_password("");

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(_)) => (),
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_password_too_short() {
        let service = create_test_service();
        let result = service.hash_password("short");

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(msg)) => {
                assert!(msg.contains("at least 8 characters"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_password_too_long() {
        let service = create_test_service();
        let long_password = "a".repeat(129);
        let result = service.hash_password(&long_password);

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(msg)) => {
                assert!(msg.contains("must not exceed 128 characters"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_password_strength_valid() {
        let service = create_test_service();
        let result = service.validate_password_strength("StrongPass123!");

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_password_strength_no_uppercase() {
        let service = create_test_service();
        let result = service.validate_password_strength("weakpass123!");

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(msg)) => {
                assert!(msg.contains("uppercase"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_password_strength_no_lowercase() {
        let service = create_test_service();
        let result = service.validate_password_strength("WEAKPASS123!");

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(msg)) => {
                assert!(msg.contains("lowercase"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_password_strength_no_digit() {
        let service = create_test_service();
        let result = service.validate_password_strength("WeakPassword!");

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(msg)) => {
                assert!(msg.contains("digit"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_validate_password_strength_no_special() {
        let service = create_test_service();
        let result = service.validate_password_strength("WeakPassword123");

        assert!(result.is_err());
        match result {
            Err(AppError::ValidationError(msg)) => {
                assert!(msg.contains("special character"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_is_password_compromised_placeholder() {
        let service = create_test_service();
        let result = service.is_password_compromised("TestPassword123!");

        // Should return Ok(false) until HIBP integration
        assert_eq!(result.unwrap(), false);
    }
}
