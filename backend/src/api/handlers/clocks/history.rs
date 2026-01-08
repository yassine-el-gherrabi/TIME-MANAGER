use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::config::AppState;
use crate::domain::enums::ClockEntryStatus;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{ClockFilter, Pagination};
use crate::services::ClockService;

#[derive(Debug, Deserialize, Default)]
pub struct HistoryQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<String>,
}

/// GET /api/v1/clocks/history
///
/// Get clock history for the authenticated user
pub async fn get_history(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<HistoryQuery>,
) -> Result<impl IntoResponse, AppError> {
    let clock_service = ClockService::new(state.db_pool.clone());

    let status_filter = query.status.as_ref().and_then(|s| match s.to_lowercase().as_str() {
        "pending" => Some(ClockEntryStatus::Pending),
        "approved" => Some(ClockEntryStatus::Approved),
        "rejected" => Some(ClockEntryStatus::Rejected),
        _ => None,
    });

    // Convert NaiveDate to DateTime<Utc> for filtering
    let filter = ClockFilter {
        start_date: query.start_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc()),
        end_date: query.end_date.map(|d| d.and_hms_opt(23, 59, 59).unwrap().and_utc()),
        status: status_filter,
        user_id: None,
    };

    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    let history = clock_service
        .get_history(claims.org_id, claims.sub, filter, pagination)
        .await?;

    Ok((StatusCode::OK, Json(history)))
}
