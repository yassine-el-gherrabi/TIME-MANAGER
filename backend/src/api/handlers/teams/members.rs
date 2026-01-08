use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::TeamService;

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
}

/// POST /api/v1/teams/:id/members
///
/// Add a member to a team (Admin+ only)
pub async fn add_member(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(team_id): Path<Uuid>,
    Json(body): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can add team members".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    let member = team_service
        .add_member(claims.org_id, team_id, body.user_id)
        .await?;

    Ok((StatusCode::CREATED, Json(member)))
}

/// DELETE /api/v1/teams/:team_id/members/:user_id
///
/// Remove a member from a team (Admin+ only)
pub async fn remove_member(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can remove team members".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    team_service
        .remove_member(claims.org_id, team_id, user_id)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
