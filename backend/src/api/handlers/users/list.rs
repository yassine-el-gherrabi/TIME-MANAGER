use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::{Admin, RoleGuard};
use crate::models::{PaginatedUsers, Pagination, UserFilter, UserResponse};
use crate::repositories::UserRepository;

/// Query parameters for listing users
#[derive(Debug, Deserialize, Default)]
pub struct ListUsersQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub role: Option<String>,
    pub search: Option<String>,
    pub include_deleted: Option<bool>,
}

/// GET /api/v1/users
///
/// List all users in the organization (Admin+)
#[tracing::instrument(
    name = "users.list",
    skip(state),
    fields(
        user_id = %user.0.sub,
        org_id = %user.0.org_id,
        page = ?query.page,
        role_filter = ?query.role
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<Admin>,
    Query(query): Query<ListUsersQuery>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Parse role filter
    let role_filter = query
        .role
        .as_ref()
        .and_then(|r| match r.to_lowercase().as_str() {
            "super_admin" | "superadmin" => Some(UserRole::SuperAdmin),
            "admin" => Some(UserRole::Admin),
            "manager" => Some(UserRole::Manager),
            "employee" => Some(UserRole::Employee),
            _ => None,
        });

    // Build filter
    let filter = UserFilter {
        role: role_filter,
        search: query.search.clone(),
    };

    // Build pagination
    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).clamp(1, 100),
    };

    // Get users
    let user_repo = UserRepository::new(state.db_pool.clone());
    let include_deleted = query.include_deleted.unwrap_or(false);
    let (users, total) = user_repo
        .list_with_deleted(claims.org_id, &filter, &pagination, include_deleted)
        .await?;

    // Build response
    let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;
    let response = PaginatedUsers {
        data: users.iter().map(UserResponse::from_user).collect(),
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages,
    };

    Ok((StatusCode::OK, Json(response)))
}
