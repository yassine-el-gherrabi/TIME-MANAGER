use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::utils::JwtService;

/// JWT authentication middleware
///
/// Validates JWT token from Authorization header
/// Attaches claims to request extensions for downstream handlers
pub async fn auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AuthMiddlewareError> {
    // Extract Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthMiddlewareError::MissingToken)?;

    // Extract bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthMiddlewareError::InvalidFormat)?;

    // Get JWT secret from environment
    let jwt_secret = std::env::var("JWT_SECRET").map_err(|_| AuthMiddlewareError::ConfigError)?;

    // Validate token
    let jwt_service = JwtService::new(&jwt_secret);
    let claims = jwt_service
        .validate_token(token)
        .map_err(|_| AuthMiddlewareError::InvalidToken)?;

    // Attach claims to request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Optional authentication middleware (doesn't fail if no token)
///
/// If valid token is present, attaches claims to extensions
/// If no token or invalid token, continues without claims
pub async fn optional_auth_middleware(mut req: Request<Body>, next: Next) -> Response {
    // Try to extract and validate token
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
                    let jwt_service = JwtService::new(&jwt_secret);
                    if let Ok(claims) = jwt_service.validate_token(token) {
                        req.extensions_mut().insert(claims);
                    }
                }
            }
        }
    }

    next.run(req).await
}

/// Authentication middleware errors
#[derive(Debug)]
pub enum AuthMiddlewareError {
    MissingToken,
    InvalidFormat,
    InvalidToken,
    ConfigError,
}

impl IntoResponse for AuthMiddlewareError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthMiddlewareError::MissingToken => {
                (StatusCode::UNAUTHORIZED, "Missing authentication token")
            }
            AuthMiddlewareError::InvalidFormat => (
                StatusCode::UNAUTHORIZED,
                "Invalid authorization header format",
            ),
            AuthMiddlewareError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid or expired token")
            }
            AuthMiddlewareError::ConfigError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Server configuration error",
            ),
        };

        let body = Json(json!({
            "error": "Unauthorized",
            "message": message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_responses() {
        let error = AuthMiddlewareError::MissingToken;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let error = AuthMiddlewareError::InvalidFormat;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let error = AuthMiddlewareError::InvalidToken;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let error = AuthMiddlewareError::ConfigError;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
