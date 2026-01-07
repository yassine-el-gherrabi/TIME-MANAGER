use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::PasswordResetService;

/// Password reset payload
#[derive(Debug, Deserialize, Validate)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 1, message = "Reset token is required"))]
    pub reset_token: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,
}

/// Password reset response
#[derive(Debug, Serialize)]
pub struct ResetPasswordResponse {
    pub message: String,
}

/// POST /api/v1/auth/password/reset
///
/// Reset password using reset token
pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<ResetPasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create password reset service
    let reset_service = PasswordResetService::new(state.db_pool.clone());

    // Reset password
    reset_service
        .reset_password(&payload.reset_token, &payload.new_password)
        .await?;

    // Build response
    let response = ResetPasswordResponse {
        message: "Password successfully reset".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reset_password_validation() {
        // Valid request
        let valid = ResetPasswordRequest {
            reset_token: "valid_token_here".to_string(),
            new_password: "NewSecurePass123!".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty token
        let empty_token = ResetPasswordRequest {
            reset_token: "".to_string(),
            new_password: "NewSecurePass123!".to_string(),
        };
        assert!(empty_token.validate().is_err());

        // Password too short
        let short_password = ResetPasswordRequest {
            reset_token: "valid_token_here".to_string(),
            new_password: "short".to_string(),
        };
        assert!(short_password.validate().is_err());
    }

    #[test]
    fn test_response_structure() {
        let response = ResetPasswordResponse {
            message: "Password successfully reset".to_string(),
        };
        assert_eq!(response.message, "Password successfully reset");
    }
}
