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
use crate::services::{AbsenceTypeService, UpdateAbsenceTypeRequest};

/// PUT /api/v1/absence-types/:id
///
/// Update an absence type (Admin+ only)
pub async fn update_absence_type(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(type_id): Path<Uuid>,
    Json(body): Json<UpdateAbsenceTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can update absence types".to_string(),
        ));
    }

    let service = AbsenceTypeService::new(state.db_pool.clone());
    let absence_type = service.update(claims.org_id, type_id, body).await?;

    Ok((StatusCode::OK, Json(absence_type)))
}
