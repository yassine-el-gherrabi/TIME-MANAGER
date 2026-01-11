use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::TeamService;

/// GET /api/v1/teams/my
///
/// Get teams for the authenticated user
#[tracing::instrument(
    name = "teams.my",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn get_my_teams(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let team_service = TeamService::new(state.db_pool.clone());
    let teams = team_service.get_user_teams(claims.org_id, claims.sub).await?;

    Ok((StatusCode::OK, Json(teams)))
}
