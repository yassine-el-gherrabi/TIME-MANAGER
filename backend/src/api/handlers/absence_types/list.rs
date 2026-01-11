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
use crate::services::{AbsenceTypeService, CacheService};

#[derive(Debug, Deserialize, Default)]
pub struct ListAbsenceTypesQuery {
    /// Filter by organization (SuperAdmin only)
    pub organization_id: Option<Uuid>,
}

/// GET /api/v1/absence-types
///
/// List all absence types for the organization
/// Uses in-memory caching with 5-minute TTL
pub async fn list_absence_types(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListAbsenceTypesQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Determine organization ID - SuperAdmin can filter by organization
    let org_id = if claims.role == UserRole::SuperAdmin {
        query.organization_id.unwrap_or(claims.org_id)
    } else {
        claims.org_id // Non-superadmin always uses their org
    };

    // Check cache first
    if let Some(cached_types) = CacheService::get_absence_types(org_id) {
        return Ok((StatusCode::OK, [("x-cache", "HIT")], Json(cached_types)));
    }

    // Cache miss - fetch from database
    let service = AbsenceTypeService::new(state.db_pool.clone());
    let types = service.list(org_id).await?;

    // Store in cache
    CacheService::set_absence_types(org_id, types.clone());

    Ok((StatusCode::OK, [("x-cache", "MISS")], Json(types)))
}
