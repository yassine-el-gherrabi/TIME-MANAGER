use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::PasswordResetService;

/// Password reset request payload
#[derive(Debug, Deserialize, Validate)]
pub struct RequestResetRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

/// Password reset request response
#[derive(Debug, Serialize)]
pub struct RequestResetResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_token: Option<String>,
}

/// POST /api/v1/auth/password/request-reset
///
/// Request password reset token (sends email in production)
pub async fn request_reset(
    State(state): State<AppState>,
    Json(payload): Json<RequestResetRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create password reset service
    let reset_service = PasswordResetService::new(state.db_pool.clone());

    // Request password reset - service returns None if user doesn't exist
    let _reset_token = reset_service.request_reset(&payload.email).await?;

    // In a real application, the token would be sent via email if it exists
    // NEVER expose the token in HTTP response to prevent:
    // - Token leakage in logs, browser history, and proxy caches
    // - User enumeration attacks

    // Always return the same response regardless of whether user exists
    let response = RequestResetResponse {
        message: "If that email exists, a password reset link has been sent.".to_string(),
        reset_token: None, // NEVER expose tokens in HTTP responses
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_reset_validation() {
        // Valid request
        let valid = RequestResetRequest {
            email: "user@example.com".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = RequestResetRequest {
            email: "not-an-email".to_string(),
        };
        assert!(invalid_email.validate().is_err());
    }

    #[test]
    fn test_response_structure() {
        let response = RequestResetResponse {
            message: "Email sent".to_string(),
            reset_token: Some("token123".to_string()),
        };
        assert_eq!(response.message, "Email sent");
        assert_eq!(response.reset_token, Some("token123".to_string()));
    }
}
