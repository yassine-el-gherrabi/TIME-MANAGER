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
use crate::services::AbsenceTypeService;

/// GET /api/v1/absence-types/:id
///
/// Get an absence type by ID
pub async fn get_absence_type(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(type_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = AbsenceTypeService::new(state.db_pool.clone());
    let absence_type = service.get(claims.org_id, type_id).await?;

    Ok((StatusCode::OK, Json(absence_type)))
}
