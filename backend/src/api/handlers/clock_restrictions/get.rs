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
use crate::services::ClockRestrictionService;

/// GET /api/v1/clock-restrictions/:id
///
/// Get a specific clock restriction
#[tracing::instrument(
    name = "clock_restrictions.get",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, restriction_id = %restriction_id)
)]
pub async fn get_restriction(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(restriction_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let restriction = service
        .get_restriction(claims.org_id, restriction_id)
        .await?;

    Ok((StatusCode::OK, Json(restriction)))
}
