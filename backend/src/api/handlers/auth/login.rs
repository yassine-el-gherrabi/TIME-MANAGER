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
use crate::services::{AuthService, BruteForceService};

/// Login request payload
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

/// Extract client IP from request headers
/// Checks X-Forwarded-For (proxy), X-Real-IP (nginx), then falls back to "unknown"
fn extract_client_ip(headers: &HeaderMap) -> String {
    // Check X-Forwarded-For first (may contain multiple IPs, take the first)
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            // X-Forwarded-For can be "client, proxy1, proxy2", take the first
            if let Some(ip) = value.split(',').next() {
                let ip = ip.trim();
                if !ip.is_empty() {
                    return ip.to_string();
                }
            }
        }
    }

    // Check X-Real-IP (common with nginx)
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            let ip = value.trim();
            if !ip.is_empty() {
                return ip.to_string();
            }
        }
    }

    // Fallback
    "unknown".to_string()
}

/// Token pair in login response
#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// Login response with tokens only (user fetched via /me endpoint)
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub tokens: TokenPair,
}

/// POST /api/v1/auth/login
///
/// Authenticate user and return JWT tokens
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create services
    let brute_force_service = BruteForceService::new(state.db_pool.clone());
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Extract client IP from headers (X-Forwarded-For, X-Real-IP, or fallback)
    let ip_address = extract_client_ip(&headers);

    // Extract User-Agent from headers for session tracking
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Check if IP is rate limited
    if brute_force_service.is_ip_rate_limited(&ip_address).await? {
        return Err(AppError::TooManyRequests(
            "Too many login attempts from this IP. Please try again later.".to_string(),
        ));
    }

    // Check if email is rate limited
    // Use generic message to prevent user enumeration
    if brute_force_service
        .is_email_rate_limited(&payload.email)
        .await?
    {
        return Err(AppError::TooManyRequests(
            "Too many requests. Please try again later.".to_string(),
        ));
    }

    // Attempt login with session info
    let token_pair = match auth_service
        .login(&payload.email, &payload.password, user_agent)
        .await
    {
        Ok(tokens) => {
            // Record successful login attempt
            brute_force_service
                .record_attempt(&payload.email, &ip_address, true)
                .await?;
            tokens
        }
        Err(e) => {
            // Record failed login attempt
            brute_force_service
                .record_attempt(&payload.email, &ip_address, false)
                .await?;
            return Err(e);
        }
    };

    // Build response with tokens only (user data fetched via /me endpoint)
    let response = LoginResponse {
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
    fn test_login_request_validation() {
        // Valid request
        let valid = LoginRequest {
            email: "user@example.com".to_string(),
            password: "SecurePass123!".to_string(),
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = LoginRequest {
            email: "not-an-email".to_string(),
            password: "SecurePass123!".to_string(),
        };
        assert!(invalid_email.validate().is_err());

        // Empty password
        let empty_password = LoginRequest {
            email: "user@example.com".to_string(),
            password: "".to_string(),
        };
        assert!(empty_password.validate().is_err());
    }

    #[test]
    fn test_extract_client_ip_x_forwarded_for() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "192.168.1.1, 10.0.0.1".parse().unwrap());
        assert_eq!(extract_client_ip(&headers), "192.168.1.1");
    }

    #[test]
    fn test_extract_client_ip_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "192.168.1.100".parse().unwrap());
        assert_eq!(extract_client_ip(&headers), "192.168.1.100");
    }

    #[test]
    fn test_extract_client_ip_fallback() {
        let headers = HeaderMap::new();
        assert_eq!(extract_client_ip(&headers), "unknown");
    }
}
