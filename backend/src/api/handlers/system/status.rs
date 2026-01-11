use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Serialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::schema::users;

/// System status response
#[derive(Debug, Serialize)]
pub struct SystemStatusResponse {
    /// Whether the system needs initial setup (no users exist)
    pub needs_setup: bool,
    /// Application version
    pub version: String,
}

/// GET /api/v1/system/status
///
/// Returns system status including whether initial setup is needed.
/// This is a public endpoint used by the frontend to determine
/// if the setup wizard should be shown.
#[tracing::instrument(name = "system.status", skip(state))]
pub async fn get_status(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let user_count = count_all_users(&state).await?;

    let response = SystemStatusResponse {
        needs_setup: user_count == 0,
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Count all users in the database
async fn count_all_users(state: &AppState) -> Result<i64, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .await
        .map_err(|e| AppError::PoolError(e.to_string()))?;

    users::table
        .count()
        .get_result(&mut conn)
        .await
        .map_err(AppError::DatabaseError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_response_serialization() {
        let response = SystemStatusResponse {
            needs_setup: true,
            version: "0.1.0".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("needs_setup"));
        assert!(json.contains("version"));
    }
}
