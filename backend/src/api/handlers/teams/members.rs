use axum::{
    extract::{Path, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::AuditContext;
use crate::services::{AuditService, TeamService};

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

#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub user_id: Uuid,
}

/// Audit data for team member operations
#[derive(Debug, Serialize)]
struct TeamMemberAuditData {
    team_id: Uuid,
    user_id: Uuid,
}

/// POST /api/v1/teams/:id/members
///
/// Add a member to a team (Admin+ only)
pub async fn add_member(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    headers: HeaderMap,
    Path(team_id): Path<Uuid>,
    Json(body): Json<AddMemberRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Extract audit context
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(claims.org_id),
        extract_client_ip(&headers),
        headers.get(USER_AGENT).and_then(|v| v.to_str().ok()).map(String::from),
    );

    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can add team members".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    let member = team_service
        .add_member(claims.org_id, team_id, body.user_id)
        .await?;

    // Log audit event (entity_id is the team_id, data contains both team and user)
    let audit_service = AuditService::new(state.db_pool.clone());
    let audit_data = TeamMemberAuditData { team_id, user_id: body.user_id };
    let _ = audit_service.log_create(&audit_ctx, "team_members", team_id, &audit_data).await;

    Ok((StatusCode::CREATED, Json(member)))
}

/// DELETE /api/v1/teams/:team_id/members/:user_id
///
/// Remove a member from a team (Admin+ only)
pub async fn remove_member(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    headers: HeaderMap,
    Path((team_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, AppError> {
    // Extract audit context
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(claims.org_id),
        extract_client_ip(&headers),
        headers.get(USER_AGENT).and_then(|v| v.to_str().ok()).map(String::from),
    );

    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can remove team members".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    team_service
        .remove_member(claims.org_id, team_id, user_id)
        .await?;

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let audit_data = TeamMemberAuditData { team_id, user_id };
    let _ = audit_service.log_delete(&audit_ctx, "team_members", team_id, &audit_data).await;

    Ok(StatusCode::NO_CONTENT)
}
