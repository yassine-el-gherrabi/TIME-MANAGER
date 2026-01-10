use axum::{
    extract::State,
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use validator::Validate;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::{RoleGuard, SuperAdmin};
use crate::models::{AuditContext, CreateOrganizationRequest, NewOrganization, OrganizationResponse};
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
pub struct CreateOrganizationResponse {
    pub message: String,
    pub organization: OrganizationResponse,
}

/// POST /api/v1/organizations
///
/// Create a new organization (Super Admin only)
pub async fn create_organization(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<SuperAdmin>,
    headers: HeaderMap,
    Json(request): Json<CreateOrganizationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Validate request
    request.validate().map_err(|e| {
        AppError::ValidationError(e.to_string())
    })?;

    // Extract audit context
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        None, // Organization-level operation, not org-specific
        extract_client_ip(&headers),
        headers.get(USER_AGENT).and_then(|v| v.to_str().ok()).map(String::from),
    );

    let org_repo = OrganizationRepository::new(state.db_pool.clone());

    // Check if slug already exists
    if org_repo.find_by_slug(&request.slug).await?.is_some() {
        return Err(AppError::ValidationError(
            "Organization with this slug already exists".to_string(),
        ));
    }

    // Create organization
    let new_org = NewOrganization::from_request(request);
    let organization = org_repo.create(new_org).await?;

    let response_org = OrganizationResponse::from_organization(&organization)
        .with_user_count(0);

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service.log_create(&audit_ctx, "organizations", organization.id, &response_org).await;

    Ok((
        StatusCode::CREATED,
        Json(CreateOrganizationResponse {
            message: "Organization created successfully".to_string(),
            organization: response_org,
        }),
    ))
}
