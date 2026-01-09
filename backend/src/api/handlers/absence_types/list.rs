use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::AbsenceTypeService;

/// GET /api/v1/absence-types
///
/// List all absence types for the organization
pub async fn list_absence_types(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = AbsenceTypeService::new(state.db_pool.clone());
    let types = service.list(claims.org_id).await?;

    Ok((StatusCode::OK, Json(types)))
}
