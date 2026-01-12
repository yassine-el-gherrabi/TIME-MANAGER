use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::{CacheService, ClosedDayService, CreateClosedDayRequest};

/// POST /api/v1/closed-days
///
/// Create a new closed day (Admin+ only)
pub async fn create_closed_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Json(body): Json<CreateClosedDayRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can create closed days".to_string(),
        ));
    }

    let service = ClosedDayService::new(state.db_pool.clone());
    let closed_day = service.create(claims.org_id, body).await?;

    // Invalidate cache
    CacheService::invalidate_closed_days();

    Ok((StatusCode::CREATED, Json(closed_day)))
}
