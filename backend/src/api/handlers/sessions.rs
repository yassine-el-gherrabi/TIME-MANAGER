use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::SessionService;

/// User session information
#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub id: Uuid,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub last_activity: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
}

/// Active sessions response
#[derive(Debug, Serialize)]
pub struct ActiveSessionsResponse {
    pub sessions: Vec<SessionInfo>,
    pub total: usize,
}

/// GET /api/v1/auth/sessions
///
/// Get all active sessions for current user
pub async fn get_active_sessions(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Create session service
    let session_service = SessionService::new(state.db_pool.clone());

    // Get user sessions
    let sessions = session_service.get_user_sessions(claims.sub).await?;

    // Convert to response format
    let session_infos: Vec<SessionInfo> = sessions
        .into_iter()
        .map(|s| SessionInfo {
            id: s.id,
            ip_address: Some(s.ip_address),
            user_agent: s.user_agent,
            created_at: s.created_at,
            last_activity: s.last_activity,
            expires_at: s.expires_at,
        })
        .collect();

    let total = session_infos.len();

    // Build response
    let response = ActiveSessionsResponse {
        sessions: session_infos,
        total,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_info_structure() {
        let now = chrono::Utc::now().naive_utc();
        let session = SessionInfo {
            id: Uuid::new_v4(),
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
            created_at: now,
            last_activity: now,
            expires_at: now,
        };

        assert_eq!(session.ip_address, Some("127.0.0.1".to_string()));
        assert_eq!(session.user_agent, Some("Mozilla/5.0".to_string()));
    }

    #[test]
    fn test_active_sessions_response() {
        let response = ActiveSessionsResponse {
            sessions: vec![],
            total: 0,
        };

        assert_eq!(response.total, 0);
        assert_eq!(response.sessions.len(), 0);
    }
}
