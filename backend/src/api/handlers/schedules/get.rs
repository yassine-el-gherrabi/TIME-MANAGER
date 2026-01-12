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
use crate::services::WorkScheduleService;

/// GET /api/v1/schedules/:id
///
/// Get a work schedule by ID
pub async fn get_schedule(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(schedule_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedule = schedule_service
        .get_schedule(claims.org_id, schedule_id)
        .await?;

    Ok((StatusCode::OK, Json(schedule)))
}
