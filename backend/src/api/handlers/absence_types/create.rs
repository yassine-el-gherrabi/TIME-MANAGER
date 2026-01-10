use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{AbsenceTypeService, CacheService, CreateAbsenceTypeRequest};

/// POST /api/v1/absence-types
///
/// Create a new absence type (Admin+ only)
pub async fn create_absence_type(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateAbsenceTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can create absence types".to_string(),
        ));
    }

    let service = AbsenceTypeService::new(state.db_pool.clone());
    let absence_type = service.create(claims.org_id, body).await?;

    // Invalidate cache
    CacheService::invalidate_absence_types(claims.org_id);

    Ok((StatusCode::CREATED, Json(absence_type)))
}
