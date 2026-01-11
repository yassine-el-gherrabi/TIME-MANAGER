use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::ClockService;

/// POST /api/v1/clocks/out
///
/// Clock out - stop tracking time
#[tracing::instrument(
    name = "clocks.clock_out",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn clock_out(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let entry = clock_service.clock_out(claims.org_id, claims.sub).await?;

    Ok((StatusCode::OK, Json(entry)))
}
