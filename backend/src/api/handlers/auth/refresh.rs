use axum::{
    extract::State,
    http::{
        header::{SET_COOKIE, USER_AGENT},
        HeaderMap, HeaderValue, StatusCode,
    },
    response::IntoResponse,
    Json,
};
use rand::RngCore;
use serde::Serialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::AuthService;

/// Refresh response with access token only (refresh token sent as HttpOnly cookie)
#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
}

/// Extract refresh token from Cookie header
fn extract_refresh_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == "refresh_token" {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            })
        })
}

/// Generate a cryptographically secure CSRF token
fn generate_csrf_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// POST /api/v1/auth/refresh
///
/// Refresh access token using refresh token from HttpOnly cookie
#[tracing::instrument(
    name = "auth.refresh",
    skip(state, headers)
)]
pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract refresh token from cookie
    let refresh_token = extract_refresh_token(&headers)
        .ok_or_else(|| AppError::Unauthorized("No refresh token provided".to_string()))?;

    // Create JWT service and auth service
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_private_key, &state.config.jwt_public_key)?;
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Extract User-Agent from headers for session tracking
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Refresh tokens with session info
    let token_pair = auth_service.refresh(&refresh_token, user_agent).await?;

    // Generate new CSRF token
    let csrf_token = generate_csrf_token();

    // Build refresh token cookie (HttpOnly, Secure, SameSite=Strict)
    let refresh_cookie = format!(
        "refresh_token={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800",
        token_pair.refresh_token
    );

    // Build CSRF token cookie (NOT HttpOnly so JS can read it)
    let csrf_cookie = format!(
        "csrf_token={}; SameSite=Strict; Path=/; Max-Age=604800",
        csrf_token
    );

    // Build response with access token only
    let response = RefreshResponse {
        access_token: token_pair.access_token,
    };

    // Create response with cookies
    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&refresh_cookie).map_err(|_| AppError::InternalError)?,
    );
    response_headers.append(
        SET_COOKIE,
        HeaderValue::from_str(&csrf_cookie).map_err(|_| AppError::InternalError)?,
    );

    Ok((StatusCode::OK, response_headers, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_refresh_token_from_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Cookie",
            "refresh_token=abc123; other_cookie=value".parse().unwrap(),
        );
        assert_eq!(
            extract_refresh_token(&headers),
            Some("abc123".to_string())
        );
    }

    #[test]
    fn test_extract_refresh_token_no_cookie() {
        let headers = HeaderMap::new();
        assert_eq!(extract_refresh_token(&headers), None);
    }

    #[test]
    fn test_extract_refresh_token_wrong_cookie() {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", "other_cookie=value".parse().unwrap());
        assert_eq!(extract_refresh_token(&headers), None);
    }

    #[test]
    fn test_generate_csrf_token() {
        let token1 = generate_csrf_token();
        let token2 = generate_csrf_token();
        // Tokens should be 64 characters (32 bytes hex-encoded)
        assert_eq!(token1.len(), 64);
        assert_eq!(token2.len(), 64);
        // Tokens should be different
        assert_ne!(token1, token2);
    }
}
