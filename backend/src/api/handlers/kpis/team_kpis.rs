use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use uuid::Uuid;

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

/// GET /api/v1/kpis/teams/:id
///
/// Get KPIs for a team (Manager+ only)
pub async fn get_team_kpis(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(team_id): Path<Uuid>,
    Query(query): Query<KPIQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Manager+ only
    if claims.role < UserRole::Manager {
        return Err(AppError::Forbidden(
            "Only managers can view team KPIs".to_string(),
        ));
    }

    let kpi_service = KPIService::new(state.db_pool.clone());

    let now = Utc::now();
    let start = query.start_date.unwrap_or_else(|| now - Duration::days(30));
    let end = query.end_date.unwrap_or(now);

    let period = DateRange { start, end };
    let kpis = kpi_service
        .get_team_kpis(claims.org_id, team_id, period)
        .await?;

    Ok((StatusCode::OK, Json(kpis)))
}
