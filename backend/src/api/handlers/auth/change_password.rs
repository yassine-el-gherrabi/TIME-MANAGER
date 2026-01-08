use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::PasswordResetService;

/// Change password request payload
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 1, message = "Current password is required"))]
    pub current_password: String,

    #[validate(length(min = 8, message = "New password must be at least 8 characters"))]
    pub new_password: String,
}

/// Change password response
#[derive(Debug, Serialize)]
pub struct ChangePasswordResponse {
    pub message: String,
}

/// PUT /api/v1/auth/change-password
///
/// Change password for authenticated user
pub async fn change_password(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Check password against HIBP breach database
    state
        .hibp_service
        .validate_not_compromised(&payload.new_password)
        .await?;

    // Create password reset service
    let password_service = PasswordResetService::new(state.db_pool.clone());

    // Change password
    password_service
        .change_password(claims.sub, &payload.current_password, &payload.new_password)
        .await?;

    // Build response
    let response = ChangePasswordResponse {
        message: "Password successfully changed".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_password_validation() {
        // Valid request
        let valid = ChangePasswordRequest {
            current_password: "OldPassword123!".to_string(),
            new_password: "NewSecurePass123!".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty current password
        let empty_current = ChangePasswordRequest {
            current_password: "".to_string(),
            new_password: "NewSecurePass123!".to_string(),
        };
        assert!(empty_current.validate().is_err());

        // New password too short
        let short_new = ChangePasswordRequest {
            current_password: "OldPassword123!".to_string(),
            new_password: "short".to_string(),
        };
        assert!(short_new.validate().is_err());
    }
}
