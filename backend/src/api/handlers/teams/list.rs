use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{Pagination, TeamFilter, TeamResponse};
use crate::services::TeamService;

#[derive(Debug, Deserialize, Default)]
pub struct ListTeamsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub manager_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListTeamsResponse {
    pub teams: Vec<TeamResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// GET /api/v1/teams
///
/// List teams with pagination
#[tracing::instrument(
    name = "teams.list",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, page = ?query.page)
)]
pub async fn list_teams(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListTeamsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let team_service = TeamService::new(state.db_pool.clone());

    let filter = TeamFilter {
        search: query.search,
        manager_id: query.manager_id.and_then(|s| s.parse().ok()),
    };

    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    let (teams, total) = team_service
        .list_teams(claims.org_id, filter, pagination.clone())
        .await?;

    Ok((
        StatusCode::OK,
        Json(ListTeamsResponse {
            teams,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
        }),
    ))
}
