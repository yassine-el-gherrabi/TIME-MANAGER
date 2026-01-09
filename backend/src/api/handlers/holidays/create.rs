use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{CreateHolidayRequest, HolidayService};

/// POST /api/v1/holidays
///
/// Create a new holiday (Admin+ only)
pub async fn create_holiday(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateHolidayRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can create holidays".to_string(),
        ));
    }

    let service = HolidayService::new(state.db_pool.clone());
    let holiday = service.create(claims.org_id, body).await?;

    Ok((StatusCode::CREATED, Json(holiday)))
}
