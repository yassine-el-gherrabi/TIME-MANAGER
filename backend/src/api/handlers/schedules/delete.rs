use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::WorkScheduleService;

/// DELETE /api/v1/schedules/:id
///
/// Delete a work schedule (Admin+ only)
pub async fn delete_schedule(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(schedule_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can delete schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    schedule_service
        .delete_schedule(claims.org_id, schedule_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
