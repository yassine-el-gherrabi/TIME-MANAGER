use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::WorkScheduleService;

/// GET /api/v1/schedules
///
/// List all work schedules for the organization
pub async fn list_schedules(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedules = schedule_service.list_schedules(claims.org_id).await?;

    Ok((StatusCode::OK, Json(schedules)))
}
