use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::ClockService;

#[derive(Debug, Deserialize)]
pub struct ClockInRequest {
    pub notes: Option<String>,
}

/// POST /api/v1/clocks/in
///
/// Clock in - start tracking time
pub async fn clock_in(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<ClockInRequest>,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let entry = clock_service
        .clock_in(claims.org_id, claims.sub, body.notes)
        .await?;

    Ok((StatusCode::CREATED, Json(entry)))
}
