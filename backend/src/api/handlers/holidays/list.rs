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
use crate::models::HolidayFilter;
use crate::services::HolidayService;

#[derive(Debug, Deserialize)]
pub struct ListHolidaysQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}

/// GET /api/v1/holidays
///
/// List all holidays for the organization
pub async fn list_holidays(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListHolidaysQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = HolidayService::new(state.db_pool.clone());

    let filter = HolidayFilter {
        start_date: query.start_date,
        end_date: query.end_date,
        is_recurring: query.is_recurring,
    };

    let holidays = service.list(claims.org_id, filter).await?;

    Ok((StatusCode::OK, Json(holidays)))
}
