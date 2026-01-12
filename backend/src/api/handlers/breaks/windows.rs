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
use crate::models::CreateBreakWindowRequest;
use crate::services::BreakService;

/// GET /api/v1/breaks/policies/:policy_id/windows
///
/// Get break windows for a policy
#[tracing::instrument(
    name = "breaks.get_windows",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, policy_id = %policy_id)
)]
pub async fn get_windows(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(policy_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let windows = service.get_windows(claims.org_id, policy_id).await?;

    Ok((StatusCode::OK, Json(windows)))
}

/// POST /api/v1/breaks/policies/:policy_id/windows
///
/// Add a break window to a policy (Admin+ only)
#[tracing::instrument(
    name = "breaks.add_window",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id, policy_id = %policy_id)
)]
pub async fn add_window(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(policy_id): Path<Uuid>,
    Json(body): Json<CreateBreakWindowRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let window = service
        .add_window(claims.org_id, policy_id, body, claims.role)
        .await?;

    Ok((StatusCode::CREATED, Json(window)))
}

#[derive(Debug, serde::Deserialize)]
pub struct WindowPath {
    pub policy_id: Uuid,
    pub window_id: Uuid,
}

/// DELETE /api/v1/breaks/policies/:policy_id/windows/:window_id
///
/// Delete a break window (Admin+ only)
#[tracing::instrument(
    name = "breaks.delete_window",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, policy_id = %path.policy_id, window_id = %path.window_id)
)]
pub async fn delete_window(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(path): Path<WindowPath>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    service
        .delete_window(claims.org_id, path.policy_id, path.window_id, claims.role)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
