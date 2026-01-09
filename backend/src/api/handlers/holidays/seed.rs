use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::HolidayService;

#[derive(Debug, Serialize)]
pub struct SeedResponse {
    pub message: String,
}

/// POST /api/v1/holidays/seed
///
/// Seed default French holidays (Admin+ only)
pub async fn seed_holidays(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can seed holidays".to_string(),
        ));
    }

    let service = HolidayService::new(state.db_pool.clone());
    service.seed_french_holidays(claims.org_id).await?;

    Ok((
        StatusCode::OK,
        Json(SeedResponse {
            message: "French holidays seeded successfully".to_string(),
        }),
    ))
}
