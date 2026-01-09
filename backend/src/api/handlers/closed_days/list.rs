use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::ClosedDayFilter;
use crate::services::ClosedDayService;

#[derive(Debug, Deserialize)]
pub struct ListClosedDaysQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}

/// GET /api/v1/closed-days
///
/// List all closed days for the organization
pub async fn list_closed_days(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListClosedDaysQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClosedDayService::new(state.db_pool.clone());

    let filter = ClosedDayFilter {
        start_date: query.start_date,
        end_date: query.end_date,
        is_recurring: query.is_recurring,
    };

    let closed_days = service.list(claims.org_id, filter).await?;

    Ok((StatusCode::OK, Json(closed_days)))
}
