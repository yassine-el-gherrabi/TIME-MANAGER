use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{CacheService, WorkScheduleService};

#[derive(Debug, Deserialize, Default)]
pub struct ListSchedulesQuery {
    /// Filter by organization (SuperAdmin only)
    pub organization_id: Option<Uuid>,
}

/// GET /api/v1/schedules
///
/// List all work schedules for the organization
pub async fn list_schedules(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListSchedulesQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Determine organization ID - SuperAdmin can filter by organization
    let org_id = if claims.role == UserRole::SuperAdmin {
        query.organization_id.unwrap_or(claims.org_id)
    } else {
        claims.org_id // Non-superadmin always uses their org
    };

    // Check cache first
    if let Some(cached_schedules) = CacheService::get_schedules(org_id) {
        return Ok((StatusCode::OK, [("x-cache", "HIT")], Json(cached_schedules)));
    }

    // Cache miss - fetch from database
    let schedule_service = WorkScheduleService::new(state.db_pool.clone());
    let schedules = schedule_service.list_schedules(org_id).await?;

    // Store in cache
    CacheService::set_schedules(org_id, schedules.clone());

    Ok((StatusCode::OK, [("x-cache", "MISS")], Json(schedules)))
}
