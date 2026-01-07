use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::services::AuthService;

/// Registration request payload
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 1, message = "First name is required"))]
    pub first_name: String,

    #[validate(length(min = 1, message = "Last name is required"))]
    pub last_name: String,

    pub organization_id: Uuid,

    #[serde(default = "default_role")]
    pub role: UserRole,
}

fn default_role() -> UserRole {
    UserRole::Employee
}

/// Registration response
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub message: String,
}

/// POST /api/v1/auth/register
///
/// Register a new user account
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload.validate().map_err(|e| {
        AppError::ValidationError(format!("Validation failed: {}", e))
    })?;

    // Create JWT service and auth service
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Register user
    let token_pair = auth_service
        .register(
            &payload.email,
            &payload.password,
            &payload.first_name,
            &payload.last_name,
            payload.organization_id,
            payload.role,
        )
        .await?;

    // Build response
    let response = RegisterResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
        message: "User registered successfully".to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_request_validation() {
        // Valid request
        let valid = RegisterRequest {
            email: "user@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            organization_id: Uuid::new_v4(),
            role: UserRole::Employee,
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = RegisterRequest {
            email: "not-an-email".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            organization_id: Uuid::new_v4(),
            role: UserRole::Employee,
        };
        assert!(invalid_email.validate().is_err());

        // Password too short
        let short_password = RegisterRequest {
            email: "user@example.com".to_string(),
            password: "short".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            organization_id: Uuid::new_v4(),
            role: UserRole::Employee,
        };
        assert!(short_password.validate().is_err());

        // Empty first name
        let empty_first_name = RegisterRequest {
            email: "user@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "".to_string(),
            last_name: "Doe".to_string(),
            organization_id: Uuid::new_v4(),
            role: UserRole::Employee,
        };
        assert!(empty_first_name.validate().is_err());
    }

    #[test]
    fn test_default_role() {
        assert_eq!(default_role(), UserRole::Employee);
    }
}
