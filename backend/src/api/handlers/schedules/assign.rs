use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::WorkScheduleService;

#[derive(Debug, Deserialize)]
pub struct AssignScheduleRequest {
    pub schedule_id: Uuid,
}

/// PUT /api/v1/users/:user_id/schedule
///
/// Assign a schedule to a user (Admin+ only)
pub async fn assign_schedule(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(user_id): Path<Uuid>,
    Json(body): Json<AssignScheduleRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can assign schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    schedule_service
        .assign_to_user(claims.org_id, user_id, body.schedule_id)
        .await?;

    Ok(StatusCode::OK)
}
