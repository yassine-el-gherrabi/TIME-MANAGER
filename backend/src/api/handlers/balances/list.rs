use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::LeaveBalanceFilter;
use crate::services::LeaveBalanceService;

#[derive(Debug, Deserialize)]
pub struct ListBalancesQuery {
    pub user_id: Option<Uuid>,
    pub absence_type_id: Option<Uuid>,
    pub year: Option<i32>,
}

/// GET /api/v1/balances
///
/// List all leave balances (Admin+ only)
pub async fn list_balances(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListBalancesQuery>,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can list all balances".to_string(),
        ));
    }

    let filter = LeaveBalanceFilter {
        user_id: query.user_id,
        absence_type_id: query.absence_type_id,
        year: query.year,
    };

    let service = LeaveBalanceService::new(state.db_pool.clone());
    let balances = service.list_balances(claims.org_id, filter).await?;

    Ok((StatusCode::OK, Json(balances)))
}
