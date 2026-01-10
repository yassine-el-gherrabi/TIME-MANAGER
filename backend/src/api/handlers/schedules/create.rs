use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{CacheService, CreateScheduleRequest, WorkScheduleService};

/// POST /api/v1/schedules
///
/// Create a new work schedule (Admin+ only)
pub async fn create_schedule(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateScheduleRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can create schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedule = schedule_service.create_schedule(claims.org_id, body).await?;

    // Invalidate cache
    CacheService::invalidate_schedules(claims.org_id);

    Ok((StatusCode::CREATED, Json(schedule)))
}
