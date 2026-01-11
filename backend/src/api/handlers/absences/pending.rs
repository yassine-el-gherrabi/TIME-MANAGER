use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{Pagination, PendingAbsenceFilter};
use crate::services::AbsenceService;

#[derive(Debug, Deserialize)]
pub struct PendingAbsencesQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    /// Filter by organization (SuperAdmin only)
    pub organization_id: Option<Uuid>,
    /// Filter by team (Admin/Manager)
    pub team_id: Option<Uuid>,
}

/// GET /api/v1/absences/pending
///
/// List pending absence requests (Manager+ only)
/// - Managers see their team's pending requests
/// - Admins see all pending requests
pub async fn list_pending_absences(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<PendingAbsencesQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Manager+ only
    if claims.role < UserRole::Manager {
        return Err(AppError::Forbidden(
            "Only managers and admins can view pending absences".to_string(),
        ));
    }

    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };

    let filter = PendingAbsenceFilter {
        organization_id: query.organization_id,
        team_id: query.team_id,
    };

    let service = AbsenceService::new(state.db_pool.clone());
    let absences = service
        .list_pending(claims.org_id, claims.sub, claims.role, filter, pagination)
        .await?;

    Ok((StatusCode::OK, Json(absences)))
}
