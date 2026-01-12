use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::TeamService;
use crate::utils::to_json_value;

#[derive(Debug, Deserialize, Default)]
pub struct GetTeamQuery {
    pub include_members: Option<bool>,
}

/// GET /api/v1/teams/:id
///
/// Get a team by ID
#[tracing::instrument(
    name = "teams.get",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, team_id = %team_id)
)]
pub async fn get_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(team_id): Path<Uuid>,
    Query(query): Query<GetTeamQuery>,
) -> Result<impl IntoResponse, AppError> {
    let team_service = TeamService::new(state.db_pool.clone());

    if query.include_members.unwrap_or(false) {
        let team_with_members = team_service
            .get_team_with_members(claims.org_id, team_id)
            .await?;
        Ok((StatusCode::OK, Json(to_json_value(&team_with_members)?)))
    } else {
        let team = team_service.get_team(claims.org_id, team_id).await?;
        Ok((StatusCode::OK, Json(to_json_value(&team)?)))
    }
}
