use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::CreateClockRestrictionRequest;
use crate::services::ClockRestrictionService;

/// POST /api/v1/clock-restrictions
///
/// Create a new clock restriction (Admin+ only)
#[tracing::instrument(
    name = "clock_restrictions.create",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn create_restriction(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateClockRestrictionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let restriction = service
        .create_restriction(claims.org_id, body, claims.role)
        .await?;

    Ok((StatusCode::CREATED, Json(restriction)))
}
