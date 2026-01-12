use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{DateRange, KPIService};

#[derive(Debug, Deserialize, Default)]
pub struct KPIQuery {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// GET /api/v1/kpis/organization
///
/// Get organization-wide KPIs (Admin+ only)
pub async fn get_org_kpis(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<KPIQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can view organization KPIs".to_string(),
        ));
    }

    let kpi_service = KPIService::new(state.db_pool.clone());

    let now = Utc::now();
    let start = query.start_date.unwrap_or_else(|| now - Duration::days(30));
    let end = query.end_date.unwrap_or(now);

    let period = DateRange { start, end };
    let kpis = kpi_service.get_org_kpis(claims.org_id, period).await?;

    Ok((StatusCode::OK, Json(kpis)))
}
