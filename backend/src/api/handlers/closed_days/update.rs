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
use crate::services::{CacheService, ClosedDayService, UpdateClosedDayRequest};

/// PUT /api/v1/closed-days/:id
///
/// Update a closed day (Admin+ only)
pub async fn update_closed_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(closed_day_id): Path<Uuid>,
    Json(body): Json<UpdateClosedDayRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can update closed days".to_string(),
        ));
    }

    let service = ClosedDayService::new(state.db_pool.clone());
    let closed_day = service.update(claims.org_id, closed_day_id, body).await?;

    // Invalidate cache
    CacheService::invalidate_closed_days();

    Ok((StatusCode::OK, Json(closed_day)))
}
