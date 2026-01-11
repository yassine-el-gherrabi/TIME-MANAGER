use axum::{
    extract::{Path, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{Admin, RoleGuard};
use crate::models::{AuditContext, UserResponse};
use crate::repositories::UserRepository;
use crate::services::AuditService;

/// Extract client IP from request headers
fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            if let Some(ip) = value.split(',').next() {
                let ip = ip.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip) = real_ip.to_str() {
            return Some(ip.to_string());
        }
    }
    None
}

/// Delete user response
#[derive(Debug, Serialize)]
pub struct DeleteUserResponse {
    pub message: String,
}

/// DELETE /api/v1/users/:id
///
/// Soft delete a user (Admin+)
/// Sets deleted_at timestamp, user can be restored later
#[tracing::instrument(
    name = "users.delete",
    skip(state, headers),
    fields(
        admin_id = %user.0.sub,
        org_id = %user.0.org_id,
        target_user_id = %user_id
    )
)]
pub async fn delete_user(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<Admin>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Extract audit context from request
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(claims.org_id),
        extract_client_ip(&headers),
        headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(String::from),
    );

    // Prevent self-deletion
    if claims.sub == user_id {
        return Err(AppError::ValidationError(
            "You cannot delete your own account".to_string(),
        ));
    }

    // Get user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Check user exists and is in the same organization
    let user_to_delete = user_repo.find_by_id(user_id).await?;
    if user_to_delete.organization_id != claims.org_id {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Prevent deleting users with equal or higher role
    if user_to_delete.role >= claims.role {
        return Err(AppError::ValidationError(
            "Cannot delete a user with equal or higher role".to_string(),
        ));
    }

    // Capture user data for audit before deletion
    let old_user_response = UserResponse::from_user(&user_to_delete);

    // Soft delete user (sets deleted_at timestamp)
    user_repo.soft_delete(user_id).await?;

    // Log audit event (fire and forget)
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service
        .log_delete(&audit_ctx, "users", user_id, &old_user_response)
        .await;

    // Build response
    let response = DeleteUserResponse {
        message: "User deactivated successfully".to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}
