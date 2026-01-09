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
use crate::services::AbsenceService;

/// POST /api/v1/absences/:id/approve
///
/// Approve an absence request (Manager+ only)
pub async fn approve_absence(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(absence_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Manager+ only
    if claims.role < UserRole::Manager {
        return Err(AppError::Forbidden(
            "Only managers and admins can approve absences".to_string(),
        ));
    }

    let service = AbsenceService::new(state.db_pool.clone());
    let absence = service
        .approve(claims.org_id, absence_id, claims.sub, claims.role)
        .await?;

    Ok((StatusCode::OK, Json(absence)))
}
