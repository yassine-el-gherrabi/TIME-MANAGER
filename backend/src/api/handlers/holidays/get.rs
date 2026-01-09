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
use crate::services::HolidayService;

/// GET /api/v1/holidays/:id
///
/// Get a holiday by ID
pub async fn get_holiday(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(holiday_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = HolidayService::new(state.db_pool.clone());
    let holiday = service.get(claims.org_id, holiday_id).await?;

    Ok((StatusCode::OK, Json(holiday)))
}
