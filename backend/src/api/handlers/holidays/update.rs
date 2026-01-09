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
use crate::services::{HolidayService, UpdateHolidayRequest};

/// PUT /api/v1/holidays/:id
///
/// Update a holiday (Admin+ only)
pub async fn update_holiday(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(holiday_id): Path<Uuid>,
    Json(body): Json<UpdateHolidayRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can update holidays".to_string(),
        ));
    }

    let service = HolidayService::new(state.db_pool.clone());
    let holiday = service.update(claims.org_id, holiday_id, body).await?;

    Ok((StatusCode::OK, Json(holiday)))
}
