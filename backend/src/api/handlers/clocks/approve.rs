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
use crate::services::ClockService;

/// POST /api/v1/clocks/:id/approve
///
/// Approve a clock entry (Manager+ only)
pub async fn approve_entry(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(entry_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let entry = clock_service
        .approve_entry(claims.org_id, entry_id, claims.sub, claims.role)
        .await?;

    Ok((StatusCode::OK, Json(entry)))
}
