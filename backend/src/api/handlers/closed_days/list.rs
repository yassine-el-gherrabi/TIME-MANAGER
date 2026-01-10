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
use crate::services::{CacheService, ClosedDayService};

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
    // Build cache key components
    let start_str = query.start_date.map(|d| d.to_string());
    let end_str = query.end_date.map(|d| d.to_string());

    // Check cache first
    if let Some(cached_days) = CacheService::get_closed_days(
        claims.org_id,
        start_str.as_deref(),
        end_str.as_deref(),
        query.is_recurring,
    ) {
        return Ok((StatusCode::OK, [("x-cache", "HIT")], Json(cached_days)));
    }

    // Cache miss - fetch from database
    let service = ClosedDayService::new(state.db_pool.clone());

    let filter = ClosedDayFilter {
        start_date: query.start_date,
        end_date: query.end_date,
        is_recurring: query.is_recurring,
    };

    let closed_days = service.list(claims.org_id, filter).await?;

    // Store in cache
    CacheService::set_closed_days(
        claims.org_id,
        start_str.as_deref(),
        end_str.as_deref(),
        query.is_recurring,
        closed_days.clone(),
    );

    Ok((StatusCode::OK, [("x-cache", "MISS")], Json(closed_days)))
}
