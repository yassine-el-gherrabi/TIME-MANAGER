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
use crate::services::{AbsenceTypeService, CacheService};

/// DELETE /api/v1/absence-types/:id
///
/// Delete an absence type (Admin+ only)
pub async fn delete_absence_type(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(type_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can delete absence types".to_string(),
        ));
    }

    let service = AbsenceTypeService::new(state.db_pool.clone());
    service.delete(claims.org_id, type_id).await?;

    // Invalidate cache
    CacheService::invalidate_absence_types(claims.org_id);

    Ok(StatusCode::NO_CONTENT)
}
