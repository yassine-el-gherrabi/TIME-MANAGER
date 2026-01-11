use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde_json::json;

use crate::config::AppState;
use crate::models::Claims;
use crate::utils::JwtService;

/// Extractor for authenticated user claims
///
/// Extracts and validates JWT from Authorization header
/// Usage: `async fn handler(AuthenticatedUser(claims): AuthenticatedUser)`
pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingToken)?;

        // Get JWT keys from AppState config (loaded from PEM files)
        let jwt_service = JwtService::new(
            &state.config.jwt_private_key,
            &state.config.jwt_public_key,
        )
        .map_err(|_| AuthError::InvalidToken)?;

        // Validate token
        let claims = jwt_service
            .validate_token(bearer.token())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(AuthenticatedUser(claims))
    }
}

/// Authentication error responses
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid or expired token"),
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
    fn test_auth_error_display() {
        let error = AuthError::MissingToken;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let error = AuthError::InvalidToken;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
