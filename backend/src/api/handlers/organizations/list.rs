use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{RoleGuard, SuperAdmin};
use crate::models::{OrganizationFilter, OrganizationPagination};
use crate::repositories::OrganizationRepository;

#[derive(Debug, Deserialize, Default)]
pub struct ListOrganizationsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

/// GET /api/v1/organizations
///
/// List all organizations with pagination (Super Admin only)
pub async fn list_organizations(
    State(state): State<AppState>,
    RoleGuard(_user, _): RoleGuard<SuperAdmin>,
    Query(query): Query<ListOrganizationsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let org_repo = OrganizationRepository::new(state.db_pool.clone());

    let filter = OrganizationFilter {
        search: query.search,
    };

    let pagination = OrganizationPagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    let paginated = org_repo.list_with_user_counts(&filter, &pagination).await?;

    Ok((StatusCode::OK, Json(paginated)))
}
