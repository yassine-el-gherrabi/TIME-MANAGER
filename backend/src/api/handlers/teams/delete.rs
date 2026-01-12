use axum::{
    extract::{Path, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
};
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

/// DELETE /api/v1/teams/:id
///
/// Delete a team (Admin+ only)
#[tracing::instrument(
    name = "teams.delete",
    skip(state, headers),
    fields(user_id = %claims.sub, org_id = %claims.org_id, team_id = %team_id)
)]
pub async fn delete_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    headers: HeaderMap,
    Path(team_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Extract audit context
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(claims.org_id),
        extract_client_ip(&headers),
        headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(String::from),
    );

    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can delete teams".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());

    // Fetch team for audit before deletion
    let old_team = team_service.get_team(claims.org_id, team_id).await?;

    team_service.delete_team(claims.org_id, team_id).await?;

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service
        .log_delete(&audit_ctx, "teams", team_id, &old_team)
        .await;

    Ok(StatusCode::NO_CONTENT)
}
