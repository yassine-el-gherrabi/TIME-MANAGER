use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::BreakService;

/// DELETE /api/v1/breaks/policies/:id
///
/// Delete a break policy (Admin+ only)
#[tracing::instrument(
    name = "breaks.delete_policy",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, policy_id = %id)
)]
pub async fn delete_policy(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    service.delete_policy(claims.org_id, id, claims.role).await?;

    Ok(StatusCode::NO_CONTENT)
}
