use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::error::AppError;
use crate::models::{NewOrganization, NewUser, OrganizationResponse, UserResponse};
use crate::repositories::{OrganizationRepository, UserRepository};
use crate::schema::users;
use crate::utils::{JwtService, PasswordService};

/// Bootstrap request payload
#[derive(Debug, Deserialize, Validate)]
pub struct BootstrapRequest {
    #[validate(length(min = 2, max = 100, message = "Organization name must be 2-100 characters"))]
    pub organization_name: String,

    #[validate(length(min = 2, max = 50, message = "Slug must be 2-50 characters"))]
    #[validate(regex(
        path = "SLUG_REGEX",
        message = "Slug must be lowercase alphanumeric with hyphens only"
    ))]
    pub organization_slug: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, max = 100, message = "First name is required"))]
    pub first_name: String,

    #[validate(length(min = 1, max = 100, message = "Last name is required"))]
    pub last_name: String,

    #[validate(length(min = 8, max = 128, message = "Password must be 8-128 characters"))]
    pub password: String,

    #[validate(length(min = 2, max = 50, message = "Timezone must be 2-50 characters"))]
    pub timezone: Option<String>,
}

lazy_static::lazy_static! {
    static ref SLUG_REGEX: regex::Regex = regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
}

/// Bootstrap response
#[derive(Debug, Serialize)]
pub struct BootstrapResponse {
    pub message: String,
    pub organization: OrganizationResponse,
    pub user: UserResponse,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// POST /api/v1/auth/bootstrap
///
/// Bootstrap the first superadmin when database is empty.
/// This endpoint only works when zero users exist in the database.
/// Creates organization, superadmin user with password, and returns access token.
#[tracing::instrument(
    name = "auth.bootstrap",
    skip(state, payload),
    fields(
        org_name = %payload.organization_name,
        admin_email = %payload.email
    )
)]
pub async fn bootstrap(
    State(state): State<AppState>,
    Json(payload): Json<BootstrapRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate payload
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

    // Validate password strength
    let password_service = PasswordService::new();
    password_service.validate_password_strength(&payload.password)?;

    // Check if ANY users exist in the database
    let user_count = count_all_users(&state).await?;
    if user_count > 0 {
        return Err(AppError::Forbidden(
            "Bootstrap already completed. Users exist in the database.".to_string(),
        ));
    }

    // Check if organization slug already exists
    let org_repo = OrganizationRepository::new(state.db_pool.clone());
    if org_repo
        .find_by_slug(&payload.organization_slug)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict(
            "Organization with this slug already exists".to_string(),
        ));
    }

    // Create the organization
    let new_org = NewOrganization {
        name: payload.organization_name.clone(),
        slug: payload.organization_slug.to_lowercase(),
        timezone: payload.timezone.unwrap_or_else(|| "Europe/Paris".to_string()),
    };

    let organization = org_repo.create(new_org).await?;

    // Hash the password
    let password_hash = password_service.hash_password(&payload.password)?;

    // Create the superadmin user with hashed password
    let user_repo = UserRepository::new(state.db_pool.clone());
    let new_user = NewUser {
        organization_id: organization.id,
        email: payload.email.to_lowercase(),
        password_hash,
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
        role: UserRole::SuperAdmin,
    };

    let user = user_repo.create(new_user).await?;

    // Generate access token for immediate login
    let jwt_service = JwtService::new(&state.config.jwt_private_key, &state.config.jwt_public_key)?;
    let access_token = jwt_service.generate_access_token(
        user.id,
        user.organization_id,
        user.role.clone(),
    )?;

    tracing::info!(
        "Bootstrap completed: organization '{}' created with superadmin '{}'",
        organization.name,
        user.email
    );

    let response = BootstrapResponse {
        message: "Setup completed successfully. You are now logged in.".to_string(),
        organization: OrganizationResponse::from_organization(&organization),
        user: UserResponse::from_user(&user),
        access_token,
        token_type: "Bearer".to_string(),
        expires_in: state.config.jwt_access_token_expiry_seconds as i64,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// Count all users in the database (including deleted)
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
    fn test_bootstrap_validation() {
        // Valid request
        let valid = BootstrapRequest {
            organization_name: "My Company".to_string(),
            organization_slug: "my-company".to_string(),
            email: "admin@company.com".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            password: "SecurePass123@".to_string(),
            timezone: Some("Europe/Paris".to_string()),
        };
        assert!(valid.validate().is_ok());

        // Invalid email
        let invalid_email = BootstrapRequest {
            organization_name: "My Company".to_string(),
            organization_slug: "my-company".to_string(),
            email: "not-an-email".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            password: "SecurePass123@".to_string(),
            timezone: None,
        };
        assert!(invalid_email.validate().is_err());

        // Invalid slug (uppercase)
        let invalid_slug = BootstrapRequest {
            organization_name: "My Company".to_string(),
            organization_slug: "My-Company".to_string(),
            email: "admin@company.com".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            password: "SecurePass123@".to_string(),
            timezone: None,
        };
        assert!(invalid_slug.validate().is_err());
    }
}
