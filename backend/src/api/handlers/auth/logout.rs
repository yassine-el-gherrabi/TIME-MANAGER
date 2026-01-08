use axum::{
    extract::State,
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::services::AuthService;

/// Logout response
#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
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

/// POST /api/v1/auth/logout
///
/// Logout user by revoking refresh token from HttpOnly cookie
pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Extract refresh token from cookie
    let refresh_token = extract_refresh_token(&headers)
        .ok_or_else(|| AppError::Unauthorized("No refresh token provided".to_string()))?;

    // Create JWT service and auth service
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Logout user
    auth_service.logout(&refresh_token).await?;

    // Build response
    let response = LogoutResponse {
        message: "Successfully logged out".to_string(),
    };

    // Clear cookies by setting Max-Age=0
    let clear_refresh_cookie = "refresh_token=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0";
    let clear_csrf_cookie = "csrf_token=; SameSite=Strict; Path=/; Max-Age=0";

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        SET_COOKIE,
        HeaderValue::from_static(clear_refresh_cookie),
    );
    response_headers.append(
        SET_COOKIE,
        HeaderValue::from_static(clear_csrf_cookie),
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
}
