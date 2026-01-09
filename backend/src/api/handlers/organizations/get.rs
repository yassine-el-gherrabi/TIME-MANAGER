use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{RoleGuard, SuperAdmin};
use crate::models::OrganizationResponse;
use crate::repositories::OrganizationRepository;

/// GET /api/v1/organizations/:id
///
/// Get a specific organization by ID (Super Admin only)
pub async fn get_organization(
    State(state): State<AppState>,
    RoleGuard(_user, _): RoleGuard<SuperAdmin>,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let org_repo = OrganizationRepository::new(state.db_pool.clone());

    let organization = org_repo.find_by_id(org_id).await?;
    let user_count = org_repo.get_user_count(org_id).await?;

    let response = OrganizationResponse::from_organization(&organization)
        .with_user_count(user_count);

    Ok((StatusCode::OK, Json(response)))
}
