use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::ClosedDayService;

/// GET /api/v1/closed-days/:id
///
/// Get a closed day by ID
pub async fn get_closed_day(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(closed_day_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = ClosedDayService::new(state.db_pool.clone());
    let closed_day = service.get(claims.org_id, closed_day_id).await?;

    Ok((StatusCode::OK, Json(closed_day)))
}
