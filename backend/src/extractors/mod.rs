// Request extractors for authentication and authorization
// This module contains Axum extractors for JWT validation and role-based access control

pub mod authenticated_user;
pub mod role_guard;

// Re-export commonly used types
pub use authenticated_user::{AuthError, AuthenticatedUser};
pub use role_guard::{Admin, Employee, Manager, RequiredRole, RoleError, RoleGuard};
