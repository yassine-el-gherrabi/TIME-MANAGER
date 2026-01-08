use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::AuthService;

/// Session information response
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub user_agent: Option<String>,
    pub created_at: String,
    pub last_activity: String,
    pub expires_at: String,
}

/// Active sessions response
#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveSessionsResponse {
    pub sessions: Vec<SessionInfo>,
    pub total: usize,
}

/// Revoke session response
#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeSessionResponse {
    pub message: String,
}

/// GET /api/v1/auth/sessions
///
/// Get all active sessions for the authenticated user
pub async fn get_sessions(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    let tokens = auth_service.get_active_sessions(claims.sub).await?;

    let sessions: Vec<SessionInfo> = tokens
        .into_iter()
        .map(|token| SessionInfo {
            id: token.id.to_string(),
            user_agent: token.user_agent,
            created_at: token.created_at.and_utc().to_rfc3339(),
            last_activity: token.last_used_at.and_utc().to_rfc3339(),
            expires_at: token.expires_at.and_utc().to_rfc3339(),
        })
        .collect();

    let total = sessions.len();
    Ok((
        StatusCode::OK,
        Json(ActiveSessionsResponse { sessions, total }),
    ))
}

/// DELETE /api/v1/auth/sessions/:id
///
/// Revoke a specific session
pub async fn revoke_session(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(session_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let session_uuid = Uuid::parse_str(&session_id)
        .map_err(|_| AppError::ValidationError("Invalid session ID format".to_string()))?;

    let jwt_service = crate::utils::JwtService::new(&state.config.jwt_secret);
    let auth_service = AuthService::new(state.db_pool.clone(), jwt_service);

    auth_service
        .revoke_session(claims.sub, session_uuid)
        .await?;

    Ok((
        StatusCode::OK,
        Json(RevokeSessionResponse {
            message: "Session revoked successfully".to_string(),
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_info_structure() {
        let session = SessionInfo {
            id: "test-id".to_string(),
            user_agent: Some("Mozilla/5.0".to_string()),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            last_activity: "2024-01-01T00:00:00Z".to_string(),
            expires_at: "2024-01-08T00:00:00Z".to_string(),
        };
        assert_eq!(session.id, "test-id");
        assert_eq!(session.user_agent, Some("Mozilla/5.0".to_string()));
    }

    #[test]
    fn test_active_sessions_response_structure() {
        let response = ActiveSessionsResponse {
            sessions: vec![],
            total: 0,
        };
        assert_eq!(response.total, 0);
        assert!(response.sessions.is_empty());
    }

    #[test]
    fn test_revoke_session_response_structure() {
        let response = RevokeSessionResponse {
            message: "Session revoked successfully".to_string(),
        };
        assert_eq!(response.message, "Session revoked successfully");
    }
}
