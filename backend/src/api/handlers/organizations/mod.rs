// Organization management endpoints (Super Admin only)
// This module contains handlers for CRUD operations on organizations

pub mod create;
pub mod delete;
pub mod get;
pub mod list;
pub mod update;

// Re-export handler functions
pub use create::create_organization;
pub use delete::delete_organization;
pub use get::get_organization;
pub use list::list_organizations;
pub use update::update_organization;
