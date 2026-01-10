use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{CacheService, WorkScheduleService};

/// GET /api/v1/schedules
///
/// List all work schedules for the organization
pub async fn list_schedules(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Check cache first
    if let Some(cached_schedules) = CacheService::get_schedules(claims.org_id) {
        return Ok((StatusCode::OK, [("x-cache", "HIT")], Json(cached_schedules)));
    }

    // Cache miss - fetch from database
    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedules = schedule_service.list_schedules(claims.org_id).await?;

    // Store in cache
    CacheService::set_schedules(claims.org_id, schedules.clone());

    Ok((StatusCode::OK, [("x-cache", "MISS")], Json(schedules)))
}
