use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::AbsenceService;

#[derive(Debug, Deserialize)]
pub struct RejectAbsenceBody {
    pub reason: Option<String>,
}

/// POST /api/v1/absences/:id/reject
///
/// Reject an absence request (Manager+ only)
pub async fn reject_absence(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(absence_id): Path<Uuid>,
    Json(body): Json<RejectAbsenceBody>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Manager+ only
    if claims.role < UserRole::Manager {
        return Err(AppError::Forbidden(
            "Only managers and admins can reject absences".to_string(),
        ));
    }

    let service = AbsenceService::new(state.db_pool.clone());
    let absence = service
        .reject(
            claims.org_id,
            absence_id,
            claims.sub,
            claims.role,
            body.reason,
        )
        .await?;

    Ok((StatusCode::OK, Json(absence)))
}
