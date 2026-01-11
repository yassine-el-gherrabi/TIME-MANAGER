use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::UserRole;
use crate::schema::users;

/// NewUser for creating users (by admin)
#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub organization_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
}

/// User update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = users)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: Option<UserRole>,
    pub phone: Option<String>,
}

/// User list filter options
#[derive(Debug, Clone, Default)]
pub struct UserFilter {
    pub role: Option<UserRole>,
    pub search: Option<String>,
    pub team_id: Option<Uuid>,
}

/// Pagination parameters
#[derive(Debug, Clone)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedUsers {
    pub data: Vec<UserResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// User response DTO (without password)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: UserRole,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub has_password: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<NaiveDateTime>,
}

impl UserResponse {
    pub fn from_user(user: &crate::repositories::User, organization_name: String) -> Self {
        // User has password if password_hash is not empty/placeholder
        let has_password = !user.password_hash.is_empty() && user.password_hash != "PENDING_INVITE";

        Self {
            id: user.id,
            organization_id: user.organization_id,
            organization_name,
            email: user.email.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            role: user.role,
            phone: user.phone.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            has_password,
            deleted_at: user.deleted_at,
        }
    }
}
