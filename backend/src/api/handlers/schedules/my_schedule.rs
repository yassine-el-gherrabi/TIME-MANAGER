use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::WorkScheduleService;

/// GET /api/v1/schedules/me
///
/// Get the authenticated user's work schedule
pub async fn get_my_schedule(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedule = schedule_service
        .get_user_schedule(claims.org_id, claims.sub)
        .await?;

    Ok((StatusCode::OK, Json(schedule)))
}
