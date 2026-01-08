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
use crate::services::{AddDayRequest, UpdateDayRequest, WorkScheduleService};

/// POST /api/v1/schedules/:id/days
///
/// Add a day to a schedule (Admin+ only)
pub async fn add_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(schedule_id): Path<Uuid>,
    Json(body): Json<AddDayRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can modify schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let day = schedule_service
        .add_day(claims.org_id, schedule_id, body)
        .await?;

    Ok((StatusCode::CREATED, Json(day)))
}

/// PUT /api/v1/schedules/days/:day_id
///
/// Update a schedule day (Admin+ only)
pub async fn update_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(day_id): Path<Uuid>,
    Json(body): Json<UpdateDayRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can modify schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let day = schedule_service.update_day(day_id, body).await?;

    Ok((StatusCode::OK, Json(day)))
}

/// DELETE /api/v1/schedules/days/:day_id
///
/// Remove a day from a schedule (Admin+ only)
pub async fn remove_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(day_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can modify schedules".to_string(),
        ));
    }

    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    schedule_service.remove_day(day_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
