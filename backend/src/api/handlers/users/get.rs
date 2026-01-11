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
use crate::models::UserResponse;
use crate::repositories::UserRepository;

/// GET /api/v1/users/:id
///
/// Get a specific user by ID
/// - Admin: can get any user in their organization
/// - Non-admin: can only get their own user data
#[tracing::instrument(
    name = "users.get",
    skip(state),
    fields(
        requester_id = %claims.sub,
        org_id = %claims.org_id,
        target_user_id = %user_id
    )
)]
pub async fn get_user(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Get user
    let user_repo = UserRepository::new(state.db_pool.clone());
    let user = user_repo.find_by_id(user_id).await?;

    // Check authorization
    // - User can view their own data
    // - Admin+ can view any user in their organization
    if user.id != claims.sub {
        if claims.role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "You can only view your own user data".to_string(),
            ));
        }

        // Admin can only view users in their organization
        if user.organization_id != claims.org_id {
            return Err(AppError::NotFound("User not found".to_string()));
        }
    }

    // Build response
    let response = UserResponse::from_user(&user);

    Ok((StatusCode::OK, Json(response)))
}
