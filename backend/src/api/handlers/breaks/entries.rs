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
use crate::models::{BreakEntryFilter, EndBreakRequest, Pagination, StartBreakRequest};
use crate::services::BreakService;

#[derive(Debug, Deserialize)]
pub struct StartBreakPath {
    pub clock_entry_id: Uuid,
}

/// POST /api/v1/breaks/entries/:clock_entry_id/start
///
/// Start a break (for explicit tracking mode)
#[tracing::instrument(
    name = "breaks.start_break",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id, clock_entry_id = %path.clock_entry_id)
)]
pub async fn start_break(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(path): Path<StartBreakPath>,
    Json(body): Json<StartBreakRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let entry = service
        .start_break(claims.org_id, claims.sub, path.clock_entry_id, body)
        .await?;

    Ok((StatusCode::CREATED, Json(entry)))
}

/// POST /api/v1/breaks/entries/end
///
/// End the current break (for explicit tracking mode)
#[tracing::instrument(
    name = "breaks.end_break",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn end_break(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<EndBreakRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let entry = service.end_break(claims.org_id, claims.sub, body).await?;

    Ok((StatusCode::OK, Json(entry)))
}

/// GET /api/v1/breaks/status
///
/// Get current break status for the user
#[tracing::instrument(
    name = "breaks.get_status",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn get_break_status(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let status = service
        .get_break_status(claims.org_id, claims.sub)
        .await?;

    Ok((StatusCode::OK, Json(status)))
}

#[derive(Debug, Deserialize, Default)]
pub struct ListEntriesQuery {
    pub user_id: Option<Uuid>,
    pub clock_entry_id: Option<Uuid>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

/// GET /api/v1/breaks/entries
///
/// List break entries
#[tracing::instrument(
    name = "breaks.list_entries",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn list_entries(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListEntriesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let filter = BreakEntryFilter {
        user_id: query.user_id,
        clock_entry_id: query.clock_entry_id,
        start_date: query.start_date,
        end_date: query.end_date,
    };

    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };

    let entries = service.list_entries(claims.org_id, filter, pagination).await?;

    Ok((StatusCode::OK, Json(entries)))
}
