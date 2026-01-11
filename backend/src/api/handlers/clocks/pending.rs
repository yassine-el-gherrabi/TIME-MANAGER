use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{Pagination, PendingClockFilter};
use crate::services::ClockService;

#[derive(Debug, Deserialize, Default)]
pub struct PendingQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    /// Filter by organization (SuperAdmin only)
    pub organization_id: Option<Uuid>,
    /// Filter by team (Admin/Manager)
    pub team_id: Option<Uuid>,
}

/// GET /api/v1/clocks/pending
///
/// List pending clock entries for approval (Manager+ only)
#[tracing::instrument(
    name = "clocks.pending",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id, page = ?query.page)
)]
pub async fn list_pending(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<PendingQuery>,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    let filter = PendingClockFilter {
        organization_id: query.organization_id,
        team_id: query.team_id,
    };

    let pending = clock_service
        .list_pending(claims.org_id, claims.sub, claims.role, filter, pagination)
        .await?;

    Ok((StatusCode::OK, Json(pending)))
}
