use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::LeaveBalanceService;

/// GET /api/v1/balances/me
///
/// Get current user's leave balances for the current year
pub async fn get_my_balances(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    let service = LeaveBalanceService::new(state.db_pool.clone());
    let balances = service.get_my_balances(claims.org_id, claims.sub).await?;

    Ok((StatusCode::OK, Json(balances)))
}
