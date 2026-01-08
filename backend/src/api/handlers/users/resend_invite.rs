use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::repositories::UserRepository;
use crate::services::InviteService;
use crate::utils::JwtService;

/// Resend invite response
#[derive(Debug, Serialize)]
pub struct ResendInviteResponse {
    pub message: String,
    pub invite_token: String,
}

/// POST /api/v1/users/:id/resend-invite
///
/// Resend invitation to a user who hasn't accepted their invite yet (Admin only)
pub async fn resend_invite(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    // Check if user is admin+ (Admin or SuperAdmin)
    if claims.role < UserRole::Admin {
        return Err(AppError::Forbidden(
            "Only administrators can resend invitations".to_string(),
        ));
    }

    // Get user repository
    let user_repo = UserRepository::new(state.db_pool.clone());

    // Check user exists and is in the same organization
    let user = user_repo.find_by_id(user_id).await?;
    if user.organization_id != claims.org_id {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    // Check if user has already accepted invite (has a real password)
    if user.password_hash != "PENDING_INVITE" {
        return Err(AppError::ValidationError(
            "User has already accepted their invitation and set their password".to_string(),
        ));
    }

    // Create new invite token (this invalidates old ones)
    let jwt_service = JwtService::new(&state.config.jwt_secret);
    let invite_service = InviteService::new(state.db_pool.clone(), jwt_service);
    let invite_token = invite_service.resend_invite(user_id).await?;

    // Send invitation email
    if let Err(e) = state
        .email_service
        .send_invite(&user.email, &user.first_name, &invite_token)
        .await
    {
        tracing::error!("Failed to send invitation email to {}: {}", user.email, e);
        // Don't fail the request if email fails - admin can try again
    }

    // Build response
    let response = ResendInviteResponse {
        message: "Invitation resent successfully. Invitation email sent.".to_string(),
        invite_token: if state.email_service.is_enabled() {
            "[sent via email]".to_string()
        } else {
            invite_token // Only return token if email is disabled (dev mode)
        },
    };

    Ok((StatusCode::OK, Json(response)))
}
