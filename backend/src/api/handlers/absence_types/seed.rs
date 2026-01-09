use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::services::AbsenceTypeService;

#[derive(Debug, Serialize)]
pub struct SeedResponse {
    pub message: String,
}

/// POST /api/v1/absence-types/seed
///
/// Seed default French absence types (Admin+ only)
pub async fn seed_absence_types(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Check authorization - Admin+ only
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only admins can seed absence types".to_string(),
        ));
    }

    let service = AbsenceTypeService::new(state.db_pool.clone());
    service.seed_default_types(claims.org_id).await?;

    Ok((
        StatusCode::OK,
        Json(SeedResponse {
            message: "Default absence types seeded successfully".to_string(),
        }),
    ))
}
