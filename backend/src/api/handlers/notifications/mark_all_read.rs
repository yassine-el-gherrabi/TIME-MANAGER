use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::NotificationService;

#[derive(Debug, Serialize)]
pub struct MarkAllReadResponse {
    pub marked_count: i64,
}

/// PUT /api/v1/notifications/read-all
///
/// Mark all notifications as read for the authenticated user
pub async fn mark_all_read(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = NotificationService::new(state.db_pool.clone());

    let marked_count = service.mark_all_as_read(claims.org_id, claims.sub).await?;

    Ok((StatusCode::OK, Json(MarkAllReadResponse { marked_count })))
}
