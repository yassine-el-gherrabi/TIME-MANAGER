use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::Pagination;
use crate::services::ClockService;

#[derive(Debug, Deserialize, Default)]
pub struct PendingQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// GET /api/v1/clocks/pending
///
/// List pending clock entries for approval (Manager+ only)
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

    let pending = clock_service
        .list_pending(claims.org_id, claims.sub, claims.role, pagination)
        .await?;

    Ok((StatusCode::OK, Json(pending)))
}
