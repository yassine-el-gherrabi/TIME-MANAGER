use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::NotificationService;

/// PUT /api/v1/notifications/:id/read
///
/// Mark a notification as read
pub async fn mark_read(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(notification_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = NotificationService::new(state.db_pool.clone());

    let notification = service
        .mark_as_read(claims.org_id, claims.sub, notification_id)
        .await?;

    Ok((StatusCode::OK, Json(notification)))
}
