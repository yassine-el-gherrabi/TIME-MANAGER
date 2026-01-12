use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::CreateBreakPolicyRequest;
use crate::services::BreakService;

/// POST /api/v1/breaks/policies
///
/// Create a new break policy (Admin+ only)
#[tracing::instrument(
    name = "breaks.create_policy",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn create_policy(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateBreakPolicyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let policy = service
        .create_policy(claims.org_id, body, claims.role)
        .await?;

    Ok((StatusCode::CREATED, Json(policy)))
}
