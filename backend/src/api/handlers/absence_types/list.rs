use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{AbsenceTypeService, CacheService};

/// GET /api/v1/absence-types
///
/// List all absence types for the organization
/// Uses in-memory caching with 5-minute TTL
pub async fn list_absence_types(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Check cache first
    if let Some(cached_types) = CacheService::get_absence_types(claims.org_id) {
        return Ok((
            StatusCode::OK,
            [("x-cache", "HIT")],
            Json(cached_types),
        ));
    }

    // Cache miss - fetch from database
    let service = AbsenceTypeService::new(state.db_pool.clone());
    let types = service.list(claims.org_id).await?;

    // Store in cache
    CacheService::set_absence_types(claims.org_id, types.clone());

    Ok((
        StatusCode::OK,
        [("x-cache", "MISS")],
        Json(types),
    ))
}
