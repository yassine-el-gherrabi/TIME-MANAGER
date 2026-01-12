use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::NotificationService;

/// GET /api/v1/notifications/unread-count
///
/// Get the count of unread notifications for the authenticated user
pub async fn unread_count(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = NotificationService::new(state.db_pool.clone());

    let response = service.get_unread_count(claims.org_id, claims.sub).await?;

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use crate::models::UnreadCountResponse;

    #[test]
    fn test_unread_count_response_serialization() {
        let response = UnreadCountResponse { count: 42 };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"count\":42"));
    }

    #[test]
    fn test_unread_count_response_zero() {
        let response = UnreadCountResponse { count: 0 };
        assert_eq!(response.count, 0);
    }
}
