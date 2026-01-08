use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{TeamService, UpdateTeamRequest};

/// PUT /api/v1/teams/:id
///
/// Update a team (Admin+ only)
pub async fn update_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(team_id): Path<Uuid>,
    Json(body): Json<UpdateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can update teams".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    let team = team_service
        .update_team(claims.org_id, team_id, body)
        .await?;

    Ok((StatusCode::OK, Json(team)))
}
