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
use crate::services::{AdjustBalanceRequest, LeaveBalanceService};

/// PUT /api/v1/balances/:id/adjust
///
/// Adjust a leave balance (Admin+ only)
pub async fn adjust_balance(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(balance_id): Path<Uuid>,
    Json(body): Json<AdjustBalanceRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can adjust balances".to_string(),
        ));
    }

    let service = LeaveBalanceService::new(state.db_pool.clone());
    let balance = service
        .adjust_balance(claims.org_id, balance_id, body)
        .await?;

    Ok((StatusCode::OK, Json(balance)))
}
