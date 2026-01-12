use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::ClockService;

#[derive(Debug, Deserialize, Default)]
pub struct ClockOutRequest {
    pub notes: Option<String>,
}

/// POST /api/v1/clocks/out
///
/// Clock out - stop tracking time with optional notes
#[tracing::instrument(
    name = "clocks.clock_out",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn clock_out(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    body: Option<Json<ClockOutRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let notes = body.and_then(|b| b.notes.clone());
    let entry = clock_service
        .clock_out(claims.org_id, claims.sub, notes)
        .await?;

    Ok((StatusCode::OK, Json(entry)))
}
