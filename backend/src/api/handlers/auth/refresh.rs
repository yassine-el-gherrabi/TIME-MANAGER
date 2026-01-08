use axum::{
    extract::State,
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
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

/// Token pair in refresh response
#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// Refresh token response (consistent with login response format)
#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub tokens: TokenPair,
}

/// POST /api/v1/auth/refresh
///
/// Refresh access token using refresh token
pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create JWT service and auth service
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Extract User-Agent from headers for session tracking
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Refresh tokens with session info
    let token_pair = auth_service
        .refresh(&payload.refresh_token, user_agent)
        .await?;

    // Build response (wrapped in tokens for consistency with login)
    let response = RefreshResponse {
        tokens: TokenPair {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
        },
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
