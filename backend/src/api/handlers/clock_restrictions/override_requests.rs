use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::ClockOverrideStatus;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{
    ClockOverrideFilter, CreateOverrideRequest, Pagination, ReviewOverrideRequest,
};
use crate::services::ClockRestrictionService;

#[derive(Debug, Deserialize, Default)]
pub struct OverrideListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<ClockOverrideStatus>,
    pub requested_action: Option<String>,
}

/// POST /api/v1/clock-restrictions/overrides
///
/// Create a new override request (for flexible mode)
#[tracing::instrument(
    name = "clock_restrictions.create_override",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn create_override_request(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateOverrideRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let request = service
        .create_override_request(claims.org_id, claims.sub, body)
        .await?;

    Ok((StatusCode::CREATED, Json(request)))
}

/// GET /api/v1/clock-restrictions/overrides/pending
///
/// List pending override requests (Manager+ only)
#[tracing::instrument(
    name = "clock_restrictions.list_pending_overrides",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn list_pending_overrides(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<OverrideListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    let requests = service
        .list_pending_override_requests(claims.org_id, claims.sub, claims.role, pagination)
        .await?;

    Ok((StatusCode::OK, Json(requests)))
}

/// GET /api/v1/clock-restrictions/overrides/me
///
/// List user's own override requests
#[tracing::instrument(
    name = "clock_restrictions.list_user_overrides",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn list_user_overrides(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<OverrideListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    let filter = ClockOverrideFilter {
        user_id: None, // Will be set by service
        status: query.status,
        requested_action: query.requested_action,
    };

    let requests = service
        .list_user_override_requests(claims.org_id, claims.sub, filter, pagination)
        .await?;

    Ok((StatusCode::OK, Json(requests)))
}

/// POST /api/v1/clock-restrictions/overrides/:id/review
///
/// Review (approve/reject) an override request (Manager+ only)
#[tracing::instrument(
    name = "clock_restrictions.review_override",
    skip(state, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id, request_id = %request_id)
)]
pub async fn review_override_request(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(request_id): Path<Uuid>,
    Json(body): Json<ReviewOverrideRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let request = service
        .review_override_request(claims.org_id, request_id, claims.sub, claims.role, body)
        .await?;

    Ok((StatusCode::OK, Json(request)))
}
