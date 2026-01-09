use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::{UserResponse, UserUpdate};
use crate::repositories::UserRepository;

/// Update user request payload
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "First name must be between 1 and 100 characters"
    ))]
    pub first_name: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Last name must be between 1 and 100 characters"
    ))]
    pub last_name: Option<String>,

    pub role: Option<UserRole>,

    #[validate(length(max = 20, message = "Phone number must be at most 20 characters"))]
    pub phone: Option<String>,
}

/// Update user response
#[derive(Debug, Serialize)]
pub struct UpdateUserResponse {
    pub message: String,
    pub user: UserResponse,
}

/// PUT /api/v1/users/:id
///
/// Update a user
/// - Admin: can update any user in their organization (including role)
/// - Non-admin: can only update their own first_name and last_name
pub async fn update_user(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Get user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Get existing user
    let existing_user = user_repo.find_by_id(user_id).await?;

    // Determine what updates are allowed based on role
    // Admin+ (Admin or SuperAdmin) can update any user in their organization
    let is_admin = claims.role >= UserRole::Admin;
    let is_self = claims.sub == user_id;

    if !is_self && !is_admin {
        return Err(AppError::Forbidden(
            "You can only update your own user data".to_string(),
        ));
    }

    // Admin can only update users in their organization
    if is_admin && existing_user.organization_id != claims.org_id {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Build update based on permissions
    let update = if is_admin {
        // Admin can update all fields
        if let Some(ref email) = payload.email {
            if user_repo.email_exists_for_other(email, user_id).await? {
                return Err(AppError::Conflict(
                    "A user with this email already exists".to_string(),
                ));
            }
        }

        UserUpdate {
            email: payload.email.map(|e| e.to_lowercase()),
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            role: payload.role,
            phone: payload.phone.clone(),
        }
    } else {
        // Non-admin can only update their own name and phone
        if payload.email.is_some() || payload.role.is_some() {
            return Err(AppError::Forbidden(
                "You can only update your name and phone. Contact an administrator to change your email or role.".to_string(),
            ));
        }

        UserUpdate {
            email: None,
            first_name: payload.first_name.clone(),
            last_name: payload.last_name.clone(),
            role: None,
            phone: payload.phone.clone(),
        }
    };

    // Update user
    let updated_user = user_repo.update(user_id, update).await?;

    // Build response
    let response = UpdateUserResponse {
        message: "User updated successfully".to_string(),
        user: UserResponse::from_user(&updated_user),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_user_validation() {
        // Valid request with email
        let valid = UpdateUserRequest {
            email: Some("test@example.com".to_string()),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            role: Some(UserRole::Manager),
            phone: Some("+33612345678".to_string()),
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = UpdateUserRequest {
            email: Some("not-an-email".to_string()),
            first_name: None,
            last_name: None,
            role: None,
            phone: None,
        };
        assert!(invalid_email.validate().is_err());

        // Empty request is valid (no changes)
        let empty = UpdateUserRequest {
            email: None,
            first_name: None,
            last_name: None,
            role: None,
            phone: None,
        };
        assert!(empty.validate().is_ok());
    }
}
