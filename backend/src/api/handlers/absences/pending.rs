use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::Pagination;
use crate::services::AbsenceService;

#[derive(Debug, Deserialize)]
pub struct PendingAbsencesQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
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

    let service = AbsenceService::new(state.db_pool.clone());
    let absences = service
        .list_pending(claims.org_id, claims.sub, claims.role, pagination)
        .await?;

    Ok((StatusCode::OK, Json(absences)))
}
