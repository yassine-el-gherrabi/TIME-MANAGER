use axum::{
    extract::{Query, State},
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
pub struct ValidateQuery {
    /// The action to validate: "clock_in" or "clock_out"
    pub action: String,
}

/// GET /api/v1/clock-restrictions/validate
///
/// Check if a clock action is currently allowed
#[tracing::instrument(
    name = "clock_restrictions.validate",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, action = %query.action)
)]
pub async fn validate_clock_action(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ValidateQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockService::new(state.db_pool.clone());

    let validation = service
        .validate_clock_action(claims.org_id, claims.sub, &query.action)
        .await?;

    Ok((StatusCode::OK, Json(validation)))
}
