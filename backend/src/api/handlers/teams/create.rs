use axum::{
    extract::State,
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::AuditContext;
use crate::services::{AuditService, CreateTeamRequest, TeamService};

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

/// POST /api/v1/teams
///
/// Create a new team (Admin+ only)
pub async fn create_team(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    headers: HeaderMap,
    Json(body): Json<CreateTeamRequest>,
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
            "Only admins can create teams".to_string(),
        ));
    }

    let team_service = TeamService::new(state.db_pool.clone());
    let team = team_service.create_team(claims.org_id, body).await?;

    // Log audit event
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service.log_create(&audit_ctx, "teams", team.id, &team).await;

    Ok((StatusCode::CREATED, Json(team)))
}
