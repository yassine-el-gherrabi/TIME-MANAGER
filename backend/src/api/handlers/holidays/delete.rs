use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::HolidayService;

/// DELETE /api/v1/holidays/:id
///
/// Delete a holiday (Admin+ only)
pub async fn delete_holiday(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(holiday_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can delete holidays".to_string(),
        ));
    }

    let service = HolidayService::new(state.db_pool.clone());
    service.delete(claims.org_id, holiday_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
