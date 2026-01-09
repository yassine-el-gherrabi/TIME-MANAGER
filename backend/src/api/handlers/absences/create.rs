use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{AbsenceService, CreateAbsenceRequest};

/// POST /api/v1/absences
///
/// Create a new absence request (all authenticated users)
pub async fn create_absence(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateAbsenceRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = AbsenceService::new(state.db_pool.clone());
    let absence = service
        .create_request(claims.org_id, claims.sub, body)
        .await?;

    Ok((StatusCode::CREATED, Json(absence)))
}
