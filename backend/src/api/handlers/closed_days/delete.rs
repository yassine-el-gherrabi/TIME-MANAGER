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
use crate::services::{CacheService, ClosedDayService};

/// DELETE /api/v1/closed-days/:id
///
/// Delete a closed day (Admin+ only)
pub async fn delete_closed_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(closed_day_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can delete closed days".to_string(),
        ));
    }

    let service = ClosedDayService::new(state.db_pool.clone());
    service.delete(claims.org_id, closed_day_id).await?;

    // Invalidate cache
    CacheService::invalidate_closed_days();

    Ok(StatusCode::NO_CONTENT)
}
