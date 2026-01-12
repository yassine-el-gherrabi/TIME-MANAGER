use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
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

/// Extract client IP from request headers
fn extract_client_ip(headers: &HeaderMap) -> String {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            if let Some(ip) = value.split(',').next() {
                let ip = ip.trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip) = real_ip.to_str() {
            return ip.to_string();
        }
    }
    "unknown".to_string()
}

/// POST /api/v1/auth/accept-invite
///
/// Accept an invitation and set password for a new user account
/// Returns JWT tokens for auto-login
pub async fn accept_invite(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<AcceptInviteRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Check rate limit (5 requests per 5 minutes per IP)
    let ip_address = extract_client_ip(&headers);
    state
        .rate_limiter
        .check_rate_limit("accept_invite", &ip_address)?;

    // Check password against HIBP breach database
    state
        .hibp_service
        .validate_not_compromised(&payload.password)
        .await?;

    // Create invite service
    let jwt_service = JwtService::new(&state.config.jwt_private_key, &state.config.jwt_public_key)?;
    let invite_service = InviteService::new(state.db_pool.clone(), jwt_service);

    // Accept the invite and get tokens
    let token_pair = invite_service
        .accept_invite(&payload.token, &payload.password)
        .await?;

    // Generate CSRF token for double submit cookie
    let csrf_token = generate_csrf_token();

    // Build response - only access_token in JSON, refresh_token in HttpOnly cookie
    let response = AcceptInviteResponse {
        message: "Account activated successfully".to_string(),
        access_token: token_pair.access_token,
        token_type: "Bearer".to_string(),
        expires_in: 900, // 15 minutes in seconds
    };

    // Set HttpOnly cookie for refresh token
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800",
        token_pair.refresh_token
    );

    // Set CSRF token cookie (readable by JS for double submit)
    let csrf_cookie = format!(
        "csrf_token={}; SameSite=Strict; Path=/; Max-Age=604800",
        csrf_token
    );

    Ok((
        StatusCode::OK,
        [
            (axum::http::header::SET_COOKIE, refresh_cookie),
            (axum::http::header::SET_COOKIE, csrf_cookie),
        ],
        Json(response),
    ))
}

/// Generate a CSRF token (64 hex characters)
fn generate_csrf_token() -> String {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
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
    headers: HeaderMap,
    Json(payload): Json<VerifyInviteRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Check rate limit (10 requests per 5 minutes per IP)
    let ip_address = extract_client_ip(&headers);
    state
        .rate_limiter
        .check_rate_limit("verify_invite", &ip_address)?;

    // Create invite service
    let jwt_service = JwtService::new(&state.config.jwt_private_key, &state.config.jwt_public_key)?;
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
