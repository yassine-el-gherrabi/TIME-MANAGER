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
use crate::services::BreakService;

/// GET /api/v1/breaks/policies/:id
///
/// Get a break policy by ID
#[tracing::instrument(
    name = "breaks.get_policy",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, policy_id = %id)
)]
pub async fn get_policy(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let policy = service.get_policy(claims.org_id, id).await?;

    Ok((StatusCode::OK, Json(policy)))
}
