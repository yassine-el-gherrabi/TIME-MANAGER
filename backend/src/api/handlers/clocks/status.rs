use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::ClockService;

/// GET /api/v1/clocks/status
///
/// Get current clock status for the authenticated user
#[tracing::instrument(
    name = "clocks.status",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn get_status(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let status = clock_service
        .get_current_status(claims.org_id, claims.sub)
        .await?;

    Ok((StatusCode::OK, Json(status)))
}
