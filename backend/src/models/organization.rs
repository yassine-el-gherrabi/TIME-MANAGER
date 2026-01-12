use chrono::NaiveDateTime;
use diesel::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::schema::organizations;

/// Organization database model
#[derive(Debug, Clone, Queryable, Identifiable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = organizations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub timezone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Regex pattern for valid slug: lowercase alphanumeric with optional hyphens
pub static SLUG_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-z0-9][a-z0-9-]*[a-z0-9]$|^[a-z0-9]$").unwrap());

/// Custom validator for slug format
fn validate_slug(slug: &str) -> Result<(), validator::ValidationError> {
    if SLUG_REGEX.is_match(slug) {
        Ok(())
    } else {
        let mut error = validator::ValidationError::new("slug_format");
        error.message =
            Some("Slug must contain only lowercase letters, numbers, and hyphens".into());
        Err(error)
    }
}

/// Request to create a new organization
#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Name must be between 2 and 100 characters"
    ))]
    pub name: String,
    #[validate(length(
        min = 2,
        max = 50,
        message = "Slug must be between 2 and 50 characters"
    ))]
    #[validate(custom(function = "validate_slug"))]
    pub slug: String,
    #[validate(length(max = 100))]
    pub timezone: Option<String>,
}

/// New organization for database insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = organizations)]
pub struct NewOrganization {
    pub name: String,
    pub slug: String,
    pub timezone: String,
}

impl NewOrganization {
    pub fn from_request(req: CreateOrganizationRequest) -> Self {
        Self {
            name: req.name,
            slug: req.slug.to_lowercase(),
            timezone: req.timezone.unwrap_or_else(|| "Europe/Paris".to_string()),
        }
    }
}

/// Request to update an organization
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateOrganizationRequest {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Name must be between 2 and 100 characters"
    ))]
    pub name: Option<String>,
    #[validate(length(max = 100))]
    pub timezone: Option<String>,
}

/// Organization update for database (slug cannot be updated)
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = organizations)]
pub struct OrganizationUpdate {
    pub name: Option<String>,
    pub timezone: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

impl OrganizationUpdate {
    pub fn from_request(req: UpdateOrganizationRequest) -> Self {
        Self {
            name: req.name,
            timezone: req.timezone,
            updated_at: Some(chrono::Utc::now().naive_utc()),
        }
    }
}

/// Organization response DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub timezone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i64>,
}

impl OrganizationResponse {
    pub fn from_organization(org: &Organization) -> Self {
        Self {
            id: org.id,
            name: org.name.clone(),
            slug: org.slug.clone(),
            timezone: org.timezone.clone(),
            created_at: org.created_at,
            updated_at: org.updated_at,
            user_count: None,
        }
    }

    pub fn with_user_count(mut self, count: i64) -> Self {
        self.user_count = Some(count);
        self
    }
}

/// Organization filter for listing
#[derive(Debug, Clone, Default)]
pub struct OrganizationFilter {
    pub search: Option<String>,
}

/// Pagination parameters
#[derive(Debug, Clone)]
pub struct OrganizationPagination {
    pub page: i64,
    pub per_page: i64,
}

impl Default for OrganizationPagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 20,
        }
    }
}

/// Paginated organizations response
#[derive(Debug, Serialize)]
pub struct PaginatedOrganizations {
    pub data: Vec<OrganizationResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
