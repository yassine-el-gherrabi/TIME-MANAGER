use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::BreakTrackingMode;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{BreakPolicyFilter, Pagination};
use crate::services::BreakService;

#[derive(Debug, Deserialize, Default)]
pub struct ListPoliciesQuery {
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub tracking_mode: Option<BreakTrackingMode>,
    pub is_active: Option<bool>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

/// GET /api/v1/breaks/policies
///
/// List break policies for the organization
#[tracing::instrument(
    name = "breaks.list_policies",
    skip(state),
    fields(user_id = %claims.sub, org_id = %claims.org_id)
)]
pub async fn list_policies(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListPoliciesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = BreakService::new(state.db_pool.clone());

    let filter = BreakPolicyFilter {
        team_id: query.team_id,
        user_id: query.user_id,
        tracking_mode: query.tracking_mode,
        is_active: query.is_active,
    };

    let pagination = Pagination {
        page: query.page,
        per_page: query.per_page,
    };

    let policies = service
        .list_policies(claims.org_id, filter, pagination)
        .await?;

    Ok((StatusCode::OK, Json(policies)))
}
