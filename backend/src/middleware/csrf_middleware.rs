use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// CSRF token validation middleware
///
/// Validates CSRF tokens for state-changing operations (POST, PUT, DELETE, PATCH)
/// GET, HEAD, OPTIONS requests are exempt from CSRF protection
pub async fn csrf_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, CsrfMiddlewareError> {
    // Skip CSRF check for safe methods
    let method = req.method();
    if method == "GET" || method == "HEAD" || method == "OPTIONS" {
        return Ok(next.run(req).await);
    }

    // Extract CSRF token from header
    let csrf_token = req
        .headers()
        .get("X-CSRF-Token")
        .and_then(|h| h.to_str().ok())
        .ok_or(CsrfMiddlewareError::MissingToken)?;

    // Extract CSRF token from cookie
    let cookie_token = req
        .headers()
        .get("Cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|cookie| {
                let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == "csrf_token" {
                    Some(parts[1])
                } else {
                    None
                }
            })
        })
        .ok_or(CsrfMiddlewareError::MissingCookie)?;

    // Validate tokens match
    if csrf_token != cookie_token {
        return Err(CsrfMiddlewareError::TokenMismatch);
    }

    Ok(next.run(req).await)
}

/// Optional CSRF middleware (doesn't fail if no token for GET requests)
///
/// Validates CSRF tokens for state-changing operations when present
/// Useful for APIs that need flexible CSRF protection
pub async fn optional_csrf_middleware(req: Request<Body>, next: Next) -> Response {
    // Skip CSRF check for safe methods
    let method = req.method();
    if method == "GET" || method == "HEAD" || method == "OPTIONS" {
        return next.run(req).await;
    }

    // Try to validate if tokens are present
    if let Some(csrf_token) = req
        .headers()
        .get("X-CSRF-Token")
        .and_then(|h| h.to_str().ok())
    {
        if let Some(cookie_token) = req.headers().get("Cookie").and_then(|h| h.to_str().ok()) {
            if let Some(cookie_value) = cookie_token.split(';').find_map(|cookie| {
                let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
                if parts.len() == 2 && parts[0] == "csrf_token" {
                    Some(parts[1])
                } else {
                    None
                }
            }) {
                // If tokens don't match, return error
                if csrf_token != cookie_value {
                    let body = Json(json!({
                        "error": "Forbidden",
                        "message": "CSRF token mismatch",
                    }));
                    return (StatusCode::FORBIDDEN, body).into_response();
                }
            }
        }
    }

    next.run(req).await
}

/// CSRF middleware errors
#[derive(Debug)]
pub enum CsrfMiddlewareError {
    MissingToken,
    MissingCookie,
    TokenMismatch,
}

impl IntoResponse for CsrfMiddlewareError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CsrfMiddlewareError::MissingToken => {
                (StatusCode::FORBIDDEN, "Missing CSRF token in header")
            }
            CsrfMiddlewareError::MissingCookie => {
                (StatusCode::FORBIDDEN, "Missing CSRF token in cookie")
            }
            CsrfMiddlewareError::TokenMismatch => (StatusCode::FORBIDDEN, "CSRF token mismatch"),
        };

        let body = Json(json!({
            "error": "Forbidden",
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
        let error = CsrfMiddlewareError::MissingToken;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let error = CsrfMiddlewareError::MissingCookie;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);

        let error = CsrfMiddlewareError::TokenMismatch;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
