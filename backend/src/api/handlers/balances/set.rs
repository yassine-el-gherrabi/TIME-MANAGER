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
use crate::services::{LeaveBalanceService, SetBalanceRequest};

/// POST /api/v1/users/:user_id/balances
///
/// Set initial balance for a user (Admin+ only)
pub async fn set_balance(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(user_id): Path<Uuid>,
    Json(body): Json<SetBalanceRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can set balances".to_string(),
        ));
    }

    let service = LeaveBalanceService::new(state.db_pool.clone());
    let balance = service.set_balance(claims.org_id, user_id, body).await?;

    Ok((StatusCode::CREATED, Json(balance)))
}
