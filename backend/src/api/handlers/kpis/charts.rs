use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{DateRange, Granularity, KPIService};

#[derive(Debug, Deserialize, Default)]
pub struct ChartQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub user_id: Option<Uuid>,
    pub granularity: Option<String>,
}

/// GET /api/v1/kpis/charts
///
/// Get chart data for hours worked
pub async fn get_charts(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ChartQuery>,
) -> Result<impl IntoResponse, AppError> {
    let kpi_service = KPIService::new(state.db_pool.clone());

    let now = Utc::now();
    let start = query.start_date.unwrap_or_else(|| now - Duration::days(30));
    let end = query.end_date.unwrap_or(now);

    // If no user_id specified, use the authenticated user
    let user_id = query.user_id.or(Some(claims.sub));

    let granularity = match query.granularity.as_deref() {
        Some("week") => Granularity::Week,
        Some("month") => Granularity::Month,
        _ => Granularity::Day,
    };

    let period = DateRange { start, end };
    let chart_data = kpi_service
        .get_chart_data(claims.org_id, user_id, period, granularity)
        .await?;

    Ok((StatusCode::OK, Json(chart_data)))
}
