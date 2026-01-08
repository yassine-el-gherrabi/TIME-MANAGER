use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{UpdateScheduleRequest, WorkScheduleService};

/// PUT /api/v1/schedules/:id
///
/// Update a work schedule (Admin+ only)
pub async fn update_schedule(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(schedule_id): Path<Uuid>,
    Json(body): Json<UpdateScheduleRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can update schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedule = schedule_service
        .update_schedule(claims.org_id, schedule_id, body)
        .await?;

    Ok((StatusCode::OK, Json(schedule)))
}
