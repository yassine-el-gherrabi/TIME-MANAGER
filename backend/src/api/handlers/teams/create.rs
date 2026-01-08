use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{CreateTeamRequest, TeamService};

/// POST /api/v1/teams
///
/// Create a new team (Admin+ only)
pub async fn create_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateTeamRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can create teams".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    let team = team_service.create_team(claims.org_id, body).await?;

    Ok((StatusCode::CREATED, Json(team)))
}
