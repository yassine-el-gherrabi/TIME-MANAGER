use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::AuthService;

/// Logout all response
#[derive(Debug, Serialize)]
pub struct LogoutAllResponse {
    pub message: String,
}

/// POST /api/v1/auth/logout-all
///
/// Logout user from all devices by revoking all refresh tokens
pub async fn logout_all(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Create JWT service and auth service
    let jwt_service =
        crate::utils::JwtService::new(&state.config.jwt_private_key, &state.config.jwt_public_key)?;
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    // Logout from all devices
    auth_service.logout_all(claims.sub).await?;

    // Build response
    let response = LogoutAllResponse {
        message: "Successfully logged out from all devices".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logout_all_response_structure() {
        let response = LogoutAllResponse {
            message: "Successfully logged out from all devices".to_string(),
        };
        assert_eq!(response.message, "Successfully logged out from all devices");
    }
}
