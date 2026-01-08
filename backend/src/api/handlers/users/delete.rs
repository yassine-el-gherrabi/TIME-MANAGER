use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{Admin, RoleGuard};
use crate::repositories::UserRepository;

/// Delete user response
#[derive(Debug, Serialize)]
pub struct DeleteUserResponse {
    pub message: String,
}

/// DELETE /api/v1/users/:id
///
/// Delete a user (Admin+)
/// Note: This is a hard delete. Consider implementing soft delete if needed.
pub async fn delete_user(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<Admin>,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Prevent self-deletion
    if claims.sub == user_id {
        return Err(AppError::ValidationError(
            "You cannot delete your own account".to_string(),
        ));
    }

    // Get user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Check user exists and is in the same organization
    let user = user_repo.find_by_id(user_id).await?;
    if user.organization_id != claims.org_id {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Delete user
    user_repo.delete(user_id).await?;

    // Build response
    let response = DeleteUserResponse {
        message: "User deleted successfully".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}
