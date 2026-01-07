use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::AuthService;

/// Refresh token request payload
#[derive(Debug, Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

/// Refresh token response
#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub refresh_token: String,
}

/// POST /api/v1/auth/refresh
///
/// Refresh access token using refresh token
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload.validate().map_err(|e| {
        AppError::ValidationError(format!("Validation failed: {}", e))
    })?;

    // Create JWT service and auth service
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Refresh tokens
    let token_pair = auth_service.refresh(&payload.refresh_token).await?;

    // Build response
    let response = RefreshResponse {
        access_token: token_pair.access_token,
        refresh_token: token_pair.refresh_token,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_request_validation() {
        // Valid request
        let valid = RefreshRequest {
            refresh_token: "valid_refresh_token_here".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Empty refresh token
        let empty_token = RefreshRequest {
            refresh_token: "".to_string(),
        };
        assert!(empty_token.validate().is_err());
    }
}
