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
use crate::repositories::{OrganizationRepository, UserRepository};
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

/// Restore user response
#[derive(Debug, Serialize)]
pub struct RestoreUserResponse {
    pub message: String,
    pub user: UserResponse,
}

/// PUT /api/v1/users/:id/restore
///
/// Restore a soft-deleted user (Admin+)
/// Clears the deleted_at timestamp, reactivating the user
pub async fn restore_user(
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

    // Get user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Find the deleted user (including deleted)
    let user_to_restore = user_repo.find_by_id_including_deleted(user_id).await?;

    // Check user is in the same organization
    if user_to_restore.organization_id != claims.org_id {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Check user is actually deleted
    if user_to_restore.deleted_at.is_none() {
        return Err(AppError::ValidationError(
            "User is not deleted and cannot be restored".to_string(),
        ));
    }

    // Fetch organization name
    let org_repo = OrganizationRepository::new(state.db_pool.clone());
    let organization = org_repo.find_by_id(claims.org_id).await?;
    let org_name = organization.name;

    // Capture old state for audit
    let old_user_response = UserResponse::from_user(&user_to_restore, org_name.clone());

    // Restore the user
    let restored_user = user_repo.restore(user_id).await?;
    let new_user_response = UserResponse::from_user(&restored_user, org_name);

    // Log audit event (fire and forget) - log as update since we're changing deleted_at
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service
        .log_update(
            &audit_ctx,
            "users",
            user_id,
            &old_user_response,
            &new_user_response,
        )
        .await;

    // Build response
    let response = RestoreUserResponse {
        message: "User restored successfully".to_string(),
        user: new_user_response,
    };

    Ok((StatusCode::OK, Json(response)))
}
