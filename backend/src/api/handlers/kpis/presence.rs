use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::KPIService;

/// GET /api/v1/kpis/presence
///
/// Get real-time presence overview (Manager+ only)
pub async fn get_presence(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Manager+ only
    if claims.role < UserRole::Manager {
        return Err(AppError::Forbidden(
            "Only managers can view presence data".to_string(),
        ));
    }

    let kpi_service = KPIService::new(state.db_pool.clone());
    let presence = kpi_service.get_real_time_presence(claims.org_id).await?;

    Ok((StatusCode::OK, Json(presence)))
}
