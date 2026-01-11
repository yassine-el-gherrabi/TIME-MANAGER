use axum::{
    extract::{Path, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{RoleGuard, SuperAdmin};
use crate::models::{
    AuditContext, OrganizationResponse, OrganizationUpdate, UpdateOrganizationRequest,
};
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
pub struct UpdateOrganizationResponse {
    pub message: String,
    pub organization: OrganizationResponse,
}

/// PUT /api/v1/organizations/:id
///
/// Update an organization (Super Admin only)
/// Note: Slug cannot be updated after creation
pub async fn update_organization(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<SuperAdmin>,
    headers: HeaderMap,
    Path(org_id): Path<Uuid>,
    Json(request): Json<UpdateOrganizationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Validate request
    request
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // Check if there's anything to update
    if request.name.is_none() && request.timezone.is_none() {
        return Err(AppError::ValidationError(
            "At least one field must be provided for update".to_string(),
        ));
    }

    // Extract audit context
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(org_id),
        extract_client_ip(&headers),
        headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(String::from),
    );

    let org_repo = OrganizationRepository::new(state.db_pool.clone());

    // Get existing organization for audit
    let old_org = org_repo.find_by_id(org_id).await?;
    let old_response = OrganizationResponse::from_organization(&old_org);

    // Update organization
    let update = OrganizationUpdate::from_request(request);
    let updated_org = org_repo.update(org_id, update).await?;

    let user_count = org_repo.get_user_count(org_id).await?;
    let new_response =
        OrganizationResponse::from_organization(&updated_org).with_user_count(user_count);

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service
        .log_update(
            &audit_ctx,
            "organizations",
            org_id,
            &old_response,
            &new_response,
        )
        .await;

    Ok((
        StatusCode::OK,
        Json(UpdateOrganizationResponse {
            message: "Organization updated successfully".to_string(),
            organization: new_response,
        }),
    ))
}
