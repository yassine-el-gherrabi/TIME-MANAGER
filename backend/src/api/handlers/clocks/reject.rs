use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::ClockService;

#[derive(Debug, Deserialize)]
pub struct RejectRequest {
    pub reason: Option<String>,
}

/// POST /api/v1/clocks/:id/reject
///
/// Reject a clock entry (Manager+ only)
#[tracing::instrument(
    name = "clocks.reject",
    skip(state, body),
    fields(rejecter_id = %claims.sub, org_id = %claims.org_id, entry_id = %entry_id)
)]
pub async fn reject_entry(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(entry_id): Path<Uuid>,
    Json(body): Json<RejectRequest>,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let entry = clock_service
        .reject_entry(claims.org_id, entry_id, claims.sub, claims.role, body.reason)
        .await?;

    Ok((StatusCode::OK, Json(entry)))
}
