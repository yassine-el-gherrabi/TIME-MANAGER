use axum::{
    extract::{Path, State},
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::AuditContext;
use crate::services::{AuditService, TeamService, UpdateTeamRequest};

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

/// PUT /api/v1/teams/:id
///
/// Update a team (Admin+ only)
#[tracing::instrument(
    name = "teams.update",
    skip(state, headers, body),
    fields(user_id = %claims.sub, org_id = %claims.org_id, team_id = %team_id)
)]
pub async fn update_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    headers: HeaderMap,
    Path(team_id): Path<Uuid>,
    Json(body): Json<UpdateTeamRequest>,
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
            "Only admins can update teams".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());

    // Fetch old team for audit
    let old_team = team_service.get_team(claims.org_id, team_id).await?;

    let _updated = team_service
        .update_team(claims.org_id, team_id, body)
        .await?;

    // Fetch updated team as TeamResponse for consistent audit logging
    let new_team = team_service.get_team(claims.org_id, team_id).await?;

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service.log_update(&audit_ctx, "teams", team_id, &old_team, &new_team).await;

    Ok((StatusCode::OK, Json(new_team)))
}
