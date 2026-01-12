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
use crate::models::UpdateBreakPolicyRequest;
use crate::services::BreakService;

/// PUT /api/v1/breaks/policies/:id
///
/// Update a break policy (Admin+ only)
#[tracing::instrument(
    name = "breaks.update_policy",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id, policy_id = %id)
)]
pub async fn update_policy(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateBreakPolicyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let policy = service
        .update_policy(claims.org_id, id, body, claims.role)
        .await?;

    Ok((StatusCode::OK, Json(policy)))
}
