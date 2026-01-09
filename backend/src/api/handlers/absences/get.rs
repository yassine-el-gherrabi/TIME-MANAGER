use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::AbsenceService;

/// GET /api/v1/absences/:id
///
/// Get an absence by ID
/// - Employees can only see their own absences
/// - Managers can see their team's absences
/// - Admins can see all absences
pub async fn get_absence(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(absence_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = AbsenceService::new(state.db_pool.clone());
    let absence = service.get(claims.org_id, absence_id).await?;

    // Check access rights
    match claims.role {
        UserRole::Employee => {
            if absence.user_id != claims.sub {
                return Err(AppError::Forbidden(
                    "You can only view your own absences".to_string(),
                ));
            }
        }
        UserRole::Manager | UserRole::Admin | UserRole::SuperAdmin => {
            // Managers and admins can see absences in the org
        }
    }

    Ok((StatusCode::OK, Json(absence)))
}
