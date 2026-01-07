use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::{AuthService, BruteForceService};

/// Login request payload
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,

    #[serde(default)]
    pub ip_address: Option<String>,
}

/// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

/// POST /api/v1/auth/login
///
/// Authenticate user and return JWT tokens
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create services
    let brute_force_service = BruteForceService::new(state.db_pool.clone());
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Extract IP address for rate limiting
    let ip_address = payload.ip_address.as_deref().unwrap_or("unknown");

    // Check if IP is rate limited
    if brute_force_service.is_ip_rate_limited(ip_address).await? {
        return Err(AppError::TooManyRequests(
            "Too many login attempts from this IP. Please try again later.".to_string(),
        ));
    }

    // Check if email is rate limited
    if brute_force_service
        .is_email_rate_limited(&payload.email)
        .await?
    {
        return Err(AppError::TooManyRequests(
            "Too many failed login attempts for this account. Please try again later.".to_string(),
        ));
    }

    // Attempt login
    let token_pair = match auth_service.login(&payload.email, &payload.password).await {
        Ok(tokens) => {
            // Record successful login attempt
            brute_force_service
                .record_attempt(&payload.email, ip_address, true)
                .await?;
            tokens
        }
        Err(e) => {
            // Record failed login attempt
            brute_force_service
                .record_attempt(&payload.email, ip_address, false)
                .await?;
            return Err(e);
        }
    };

    // Build response
    let response = LoginResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_request_validation() {
        // Valid request
        let valid = LoginRequest {
            email: "user@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            ip_address: Some("127.0.0.1".to_string()),
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = LoginRequest {
            email: "not-an-email".to_string(),
            password: "SecurePass123!".to_string(),
            ip_address: None,
        };
        assert!(invalid_email.validate().is_err());

        // Empty password
        let empty_password = LoginRequest {
            email: "user@example.com".to_string(),
            password: "".to_string(),
            ip_address: None,
        };
        assert!(empty_password.validate().is_err());
    }

    #[test]
    fn test_ip_address_default() {
        let request = LoginRequest {
            email: "user@example.com".to_string(),
            password: "password".to_string(),
            ip_address: None,
        };
        assert_eq!(request.ip_address, None);
    }
}
