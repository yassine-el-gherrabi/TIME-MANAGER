use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::InviteService;
use crate::utils::JwtService;

/// Accept invite request payload
#[derive(Debug, Deserialize, Validate)]
pub struct AcceptInviteRequest {
    #[validate(length(min = 1, message = "Invite token is required"))]
    pub token: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

/// Accept invite response
#[derive(Debug, Serialize)]
pub struct AcceptInviteResponse {
    pub message: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

/// POST /api/v1/auth/accept-invite
///
/// Accept an invitation and set password for a new user account
/// Returns JWT tokens for auto-login
pub async fn accept_invite(
    State(state): State<AppState>,
    Json(payload): Json<AcceptInviteRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create invite service
    let jwt_service = JwtService::new(&state.config.jwt_secret);
    let invite_service = InviteService::new(state.db_pool.clone(), jwt_service);

    // Accept the invite and get tokens
    let token_pair = invite_service
        .accept_invite(&payload.token, &payload.password)
        .await?;

    // Build response
    let response = AcceptInviteResponse {
        message: "Account activated successfully".to_string(),
        access_token: token_pair.access_token,
        token_type: "Bearer".to_string(),
        expires_in: 900, // 15 minutes in seconds
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Verify invite token request (optional - for frontend validation)
#[derive(Debug, Deserialize, Validate)]
pub struct VerifyInviteRequest {
    #[validate(length(min = 1, message = "Invite token is required"))]
    pub token: String,
}

/// Verify invite token response
#[derive(Debug, Serialize)]
pub struct VerifyInviteResponse {
    pub valid: bool,
    pub message: String,
}

/// POST /api/v1/auth/verify-invite
///
/// Verify if an invite token is valid (without using it)
pub async fn verify_invite(
    State(state): State<AppState>,
    Json(payload): Json<VerifyInviteRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create invite service
    let jwt_service = JwtService::new(&state.config.jwt_secret);
    let invite_service = InviteService::new(state.db_pool.clone(), jwt_service);

    // Verify the token
    match invite_service.verify_invite_token(&payload.token).await {
        Ok(_user_id) => {
            let response = VerifyInviteResponse {
                valid: true,
                message: "Invite token is valid".to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_) => {
            let response = VerifyInviteResponse {
                valid: false,
                message: "Invite token is invalid or expired".to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_invite_validation() {
        // Valid request
        let valid = AcceptInviteRequest {
            token: "abc123def456".to_string(),
            password: "SecurePass123!".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty token
        let empty_token = AcceptInviteRequest {
            token: "".to_string(),
            password: "SecurePass123!".to_string(),
        };
        assert!(empty_token.validate().is_err());

        // Password too short
        let short_password = AcceptInviteRequest {
            token: "abc123def456".to_string(),
            password: "short".to_string(),
        };
        assert!(short_password.validate().is_err());
    }

    #[test]
    fn test_verify_invite_validation() {
        // Valid request
        let valid = VerifyInviteRequest {
            token: "abc123def456".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty token
        let empty_token = VerifyInviteRequest {
            token: "".to_string(),
        };
        assert!(empty_token.validate().is_err());
    }
}
