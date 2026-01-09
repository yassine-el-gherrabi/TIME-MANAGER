use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::{AbsenceStatus, UserRole};
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{AbsenceFilter, Pagination};
use crate::services::AbsenceService;

#[derive(Debug, Deserialize)]
pub struct ListAbsencesQuery {
    pub user_id: Option<Uuid>,
    pub status: Option<AbsenceStatus>,
    pub type_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// GET /api/v1/absences
///
/// List absences with filters
/// - Employees see only their own absences
/// - Managers see their team's absences
/// - Admins see all absences
pub async fn list_absences(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListAbsencesQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = AbsenceService::new(state.db_pool.clone());

    // Build filter based on role
    let filter = match claims.role {
        UserRole::Employee => {
            // Employees can only see their own absences
            AbsenceFilter {
                user_id: Some(claims.sub),
                status: query.status,
                type_id: query.type_id,
                start_date: query.start_date,
                end_date: query.end_date,
            }
        }
        UserRole::Manager | UserRole::Admin | UserRole::SuperAdmin => {
            // Managers and admins can use all provided filters
            AbsenceFilter {
                user_id: query.user_id,
                status: query.status,
                type_id: query.type_id,
                start_date: query.start_date,
                end_date: query.end_date,
            }
        }
    };

    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };

    let absences = service.list(claims.org_id, filter, pagination).await?;

    Ok((StatusCode::OK, Json(absences)))
}
