use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::config::AppState;
use crate::domain::enums::UserRole;
use crate::extractors::authenticated_user::AuthenticatedUser;

/// Role-based access control guard
///
/// Ensures authenticated user has required role
/// Usage: `async fn handler(RoleGuard(user, _): RoleGuard<Admin>)`
pub struct RoleGuard<T: RequiredRole>(pub AuthenticatedUser, pub std::marker::PhantomData<T>);

/// Trait for defining required roles
pub trait RequiredRole {
    fn required_role() -> UserRole;
    fn role_name() -> &'static str;
}

/// Super Admin role marker (highest privilege)
pub struct SuperAdmin;

impl RequiredRole for SuperAdmin {
    fn required_role() -> UserRole {
        UserRole::SuperAdmin
    }

    fn role_name() -> &'static str {
        "SuperAdmin"
    }
}

/// Admin role marker (includes SuperAdmin)
pub struct Admin;

impl RequiredRole for Admin {
    fn required_role() -> UserRole {
        UserRole::Admin
    }

    fn role_name() -> &'static str {
        "Admin"
    }
}

/// Manager role marker (includes Admin)
pub struct Manager;

impl RequiredRole for Manager {
    fn required_role() -> UserRole {
        UserRole::Manager
    }

    fn role_name() -> &'static str {
        "Manager"
    }
}

/// Employee role marker (all authenticated users)
pub struct Employee;

impl RequiredRole for Employee {
    fn required_role() -> UserRole {
        UserRole::Employee
    }

    fn role_name() -> &'static str {
        "Employee"
    }
}

#[async_trait]
impl<T> FromRequestParts<AppState> for RoleGuard<T>
where
    T: RequiredRole + Send + Sync,
{
    type Rejection = RoleError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // First authenticate the user
        let user = AuthenticatedUser::from_request_parts(parts, state)
            .await
            .map_err(|_| RoleError::Unauthorized)?;

        // Check role hierarchy: SuperAdmin > Admin > Manager > Employee
        let has_permission = match T::required_role() {
            UserRole::Employee => true, // All authenticated users are at least employees
            UserRole::Manager => matches!(
                user.0.role,
                UserRole::Manager | UserRole::Admin | UserRole::SuperAdmin
            ),
            UserRole::Admin => matches!(user.0.role, UserRole::Admin | UserRole::SuperAdmin),
            UserRole::SuperAdmin => matches!(user.0.role, UserRole::SuperAdmin),
        };

        if !has_permission {
            return Err(RoleError::InsufficientPermissions {
                required: T::role_name(),
                actual: format!("{:?}", user.0.role),
            });
        }

        Ok(RoleGuard(user, std::marker::PhantomData))
    }
}

/// Role authorization error
#[derive(Debug)]
pub enum RoleError {
    Unauthorized,
    InsufficientPermissions {
        required: &'static str,
        actual: String,
    },
}

impl IntoResponse for RoleError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            RoleError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Authentication required".to_string(),
            ),
            RoleError::InsufficientPermissions { required, actual } => (
                StatusCode::FORBIDDEN,
                format!(
                    "Insufficient permissions. Required: {}, Actual: {}",
                    required, actual
                ),
            ),
        };

        let body = Json(json!({
            "error": match self {
                RoleError::Unauthorized => "Unauthorized",
                RoleError::InsufficientPermissions { .. } => "Forbidden",
            },
            "message": message,
        }));

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_roles() {
        assert_eq!(SuperAdmin::required_role(), UserRole::SuperAdmin);
        assert_eq!(SuperAdmin::role_name(), "SuperAdmin");

        assert_eq!(Admin::required_role(), UserRole::Admin);
        assert_eq!(Admin::role_name(), "Admin");

        assert_eq!(Manager::required_role(), UserRole::Manager);
        assert_eq!(Manager::role_name(), "Manager");

        assert_eq!(Employee::required_role(), UserRole::Employee);
        assert_eq!(Employee::role_name(), "Employee");
    }

    #[test]
    fn test_role_error_responses() {
        let error = RoleError::Unauthorized;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let error = RoleError::InsufficientPermissions {
            required: "Admin",
            actual: "Employee".to_string(),
        };
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
