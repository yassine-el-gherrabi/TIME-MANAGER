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
use crate::extractors::{RoleGuard, SuperAdmin};
use crate::models::{AuditContext, OrganizationResponse};
use crate::repositories::OrganizationRepository;
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

#[derive(Debug, Serialize)]
pub struct DeleteOrganizationResponse {
    pub message: String,
}

/// DELETE /api/v1/organizations/:id
///
/// Delete an organization (Super Admin only)
/// Organization can only be deleted if it has no users
pub async fn delete_organization(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<SuperAdmin>,
    headers: HeaderMap,
    Path(org_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Extract audit context
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(org_id),
        extract_client_ip(&headers),
        headers.get(USER_AGENT).and_then(|v| v.to_str().ok()).map(String::from),
    );

    let org_repo = OrganizationRepository::new(state.db_pool.clone());

    // Get organization for audit before deletion
    let organization = org_repo.find_by_id(org_id).await?;
    let org_response = OrganizationResponse::from_organization(&organization);

    // Delete organization (will fail if users exist)
    org_repo.delete(org_id).await?;

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service.log_delete(&audit_ctx, "organizations", org_id, &org_response).await;

    Ok((
        StatusCode::OK,
        Json(DeleteOrganizationResponse {
            message: "Organization deleted successfully".to_string(),
        }),
    ))
}
