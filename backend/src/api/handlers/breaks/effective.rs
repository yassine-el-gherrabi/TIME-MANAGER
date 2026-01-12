use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::BreakService;

/// GET /api/v1/breaks/effective
///
/// Get the effective break policy for the current user
#[tracing::instrument(
    name = "breaks.get_effective",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn get_effective_policy(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let effective = service
        .get_effective_policy(claims.org_id, claims.sub)
        .await?;

    Ok((StatusCode::OK, Json(effective)))
}
