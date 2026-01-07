use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::AuthService;

/// Logout request payload
#[derive(Debug, Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

/// Logout response
#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

/// POST /api/v1/auth/logout
///
/// Logout user by revoking refresh token
pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create JWT service and auth service
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Logout user
    auth_service.logout(&payload.refresh_token).await?;

    // Build response
    let response = LogoutResponse {
        message: "Successfully logged out".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logout_request_validation() {
        // Valid request
        let valid = LogoutRequest {
            refresh_token: "valid_refresh_token_here".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty refresh token
        let empty_token = LogoutRequest {
            refresh_token: "".to_string(),
        };
        assert!(empty_token.validate().is_err());
    }
}
