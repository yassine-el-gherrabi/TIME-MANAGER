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
use crate::services::AbsenceService;

/// POST /api/v1/absences/:id/cancel
///
/// Cancel an absence request (owner only)
pub async fn cancel_absence(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(absence_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = AbsenceService::new(state.db_pool.clone());
    let absence = service
        .cancel(claims.org_id, absence_id, claims.sub)
        .await?;

    Ok((StatusCode::OK, Json(absence)))
}
