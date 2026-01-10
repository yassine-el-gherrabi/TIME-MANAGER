use axum::{
    extract::State,
    http::{header::USER_AGENT, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::{Admin, RoleGuard};
use crate::models::{AuditContext, NewUser, UserResponse};
use crate::repositories::UserRepository;
use crate::services::{AuditService, InviteService};
use crate::utils::JwtService;

/// Extract client IP from request headers
fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded.to_str() {
            if let Some(ip) = value.split(',').next() {
                let ip = ip.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
    }
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip) = real_ip.to_str() {
            return Some(ip.to_string());
        }
    }
    None
}

/// Create user request payload
#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, max = 100, message = "First name is required"))]
    pub first_name: String,

    #[validate(length(min = 1, max = 100, message = "Last name is required"))]
    pub last_name: String,

    pub role: UserRole,
}

/// Create user response
#[derive(Debug, Serialize)]
pub struct CreateUserResponse {
    pub message: String,
    pub user: UserResponse,
    pub invite_token: String,
}

/// POST /api/v1/users
///
/// Create a new user (Admin+)
/// This creates a user with a placeholder password and generates an invite token
pub async fn create_user(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<Admin>,
    headers: HeaderMap,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let claims = user.0;

    // Extract audit context from request
    let audit_ctx = AuditContext::new(
        Some(claims.sub),
        Some(claims.org_id),
        extract_client_ip(&headers),
        headers.get(USER_AGENT).and_then(|v| v.to_str().ok()).map(String::from),
    );

    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Create user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Check if email already exists
    if user_repo.email_exists(&payload.email).await? {
        return Err(AppError::Conflict(
            "A user with this email already exists".to_string(),
        ));
    }

    // Create user with placeholder password
    let new_user = NewUser {
        organization_id: claims.org_id,
        email: payload.email.to_lowercase(),
        password_hash: "PENDING_INVITE".to_string(), // Placeholder until user accepts invite
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
        role: payload.role,
    };

    let user = user_repo.create(new_user).await?;

    // Log audit event (fire and forget)
    let audit_service = AuditService::new(state.db_pool.clone());
    let _ = audit_service.log_create(&audit_ctx, "users", user.id, &UserResponse::from_user(&user)).await;

    // Generate invite token
    let jwt_service = JwtService::new(&state.config.jwt_private_key, &state.config.jwt_public_key)?;
    let invite_service = InviteService::new(state.db_pool.clone(), jwt_service);
    let invite_token = invite_service.create_invite_token(user.id).await?;

    // Send invitation email
    if let Err(e) = state
        .email_service
        .send_invite(&user.email, &user.first_name, &invite_token)
        .await
    {
        tracing::error!("Failed to send invitation email to {}: {}", user.email, e);
        // Don't fail the request if email fails - user is created, admin can resend later
    }

    // Build response (don't include token since it's sent via email)
    let response = CreateUserResponse {
        message: "User created successfully. Invitation email sent.".to_string(),
        user: UserResponse::from_user(&user),
        invite_token: if state.email_service.is_enabled() {
            "[sent via email]".to_string()
        } else {
            invite_token // Only return token if email is disabled (dev mode)
        },
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_validation() {
        // Valid request
        let valid = CreateUserRequest {
            email: "test@example.com".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            role: UserRole::Employee,
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = CreateUserRequest {
            email: "not-an-email".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            role: UserRole::Employee,
        };
        assert!(invalid_email.validate().is_err());

        // Empty first name
        let empty_name = CreateUserRequest {
            email: "test@example.com".to_string(),
            first_name: "".to_string(),
            last_name: "Doe".to_string(),
            role: UserRole::Employee,
        };
        assert!(empty_name.validate().is_err());
    }
}
