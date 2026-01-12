use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::ClockRestrictionMode;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::ClockRestrictionFilter;
use crate::services::ClockRestrictionService;

#[derive(Debug, Deserialize, Default)]
pub struct ListRestrictionsQuery {
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub mode: Option<ClockRestrictionMode>,
    pub is_active: Option<bool>,
}

/// GET /api/v1/clock-restrictions
///
/// List clock restrictions for the organization
#[tracing::instrument(
    name = "clock_restrictions.list",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn list_restrictions(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListRestrictionsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClockRestrictionService::new(state.db_pool.clone());

    let filter = ClockRestrictionFilter {
        team_id: query.team_id,
        user_id: query.user_id,
        mode: query.mode,
        is_active: query.is_active,
    };

    let restrictions = service.list_restrictions(claims.org_id, filter).await?;

    Ok((StatusCode::OK, Json(restrictions)))
}
