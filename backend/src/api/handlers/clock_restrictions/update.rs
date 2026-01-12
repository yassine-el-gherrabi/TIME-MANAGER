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
use crate::models::UpdateClockRestrictionRequest;
use crate::services::ClockRestrictionService;

/// PUT /api/v1/clock-restrictions/:id
///
/// Update a clock restriction (Admin+ only)
#[tracing::instrument(
    name = "clock_restrictions.update",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id, restriction_id = %restriction_id)
)]
pub async fn update_restriction(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(restriction_id): Path<Uuid>,
    Json(body): Json<UpdateClockRestrictionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let restriction = service
        .update_restriction(claims.org_id, restriction_id, body, claims.role)
        .await?;

    Ok((StatusCode::OK, Json(restriction)))
}
