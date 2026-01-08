use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::TeamService;

/// DELETE /api/v1/teams/:id
///
/// Delete a team (Admin+ only)
pub async fn delete_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(team_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can delete teams".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    team_service.delete_team(claims.org_id, team_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
