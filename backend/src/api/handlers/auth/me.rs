use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::repositories::UserRepository;

/// Current user response
#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub phone: Option<String>,
    pub organization_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}

/// GET /api/v1/auth/me
///
/// Get current authenticated user information
pub async fn me(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<impl IntoResponse, AppError> {
    // Create user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Find user by ID
    let user = user_repo.find_by_id(claims.sub).await?;

    // Build response
    let response = MeResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
        phone: user.phone,
        organization_id: user.organization_id,
        created_at: user.created_at,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_me_response_structure() {
        let now = chrono::Utc::now().naive_utc();
        let response = MeResponse {
            id: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            role: UserRole::Employee,
            phone: Some("+33612345678".to_string()),
            organization_id: Uuid::new_v4(),
            created_at: now,
        };

        assert_eq!(response.email, "user@example.com");
        assert_eq!(response.first_name, "John");
        assert_eq!(response.last_name, "Doe");
        assert_eq!(response.role, UserRole::Employee);
    }
}
