use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{
    NewOrganization, Organization, OrganizationFilter, OrganizationPagination,
    OrganizationResponse, OrganizationUpdate, PaginatedOrganizations,
};
use crate::schema::{organizations, users};

pub struct OrganizationRepository {
    pool: DbPool,
}

impl OrganizationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Find organization by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Organization, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        organizations::table
            .find(id)
            .first::<Organization>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound(format!("Organization {} not found", id)))
    }

    /// Find organization by slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<Organization>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        organizations::table
            .filter(organizations::slug.eq(slug.to_lowercase()))
            .first::<Organization>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)
    }

    /// List all organizations with pagination and filtering
    pub async fn list(
        &self,
        filter: &OrganizationFilter,
        pagination: &OrganizationPagination,
    ) -> Result<(Vec<Organization>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Build base query
        let mut query = organizations::table.into_boxed();

        // Apply search filter
        if let Some(ref search) = filter.search {
            let search_pattern = format!("%{}%", search.to_lowercase());
            query = query.filter(
                organizations::name
                    .ilike(search_pattern.clone())
                    .or(organizations::slug.ilike(search_pattern)),
            );
        }

        // Get total count
        let total: i64 = {
            let mut count_query = organizations::table.into_boxed();
            if let Some(ref search) = filter.search {
                let search_pattern = format!("%{}%", search.to_lowercase());
                count_query = count_query.filter(
                    organizations::name
                        .ilike(search_pattern.clone())
                        .or(organizations::slug.ilike(search_pattern)),
                );
            }
            count_query
                .count()
                .get_result(&mut conn)
                .await
                .map_err(AppError::DatabaseError)?
        };

        // Apply pagination
        let offset = (pagination.page - 1) * pagination.per_page;
        let organizations = query
            .order(organizations::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<Organization>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((organizations, total))
    }

    /// Create a new organization
    pub async fn create(&self, new_org: NewOrganization) -> Result<Organization, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(organizations::table)
            .values(&new_org)
            .get_result::<Organization>(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::ValidationError("Organization with this slug already exists".to_string()),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Update an organization
    pub async fn update(
        &self,
        id: Uuid,
        update: OrganizationUpdate,
    ) -> Result<Organization, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::update(organizations::table.find(id))
            .set(&update)
            .get_result::<Organization>(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound(format!("Organization {} not found", id))
                }
                _ => AppError::DatabaseError(e),
            })
    }

    /// Delete an organization (only if no users exist)
    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Check if organization has any users (active or deleted)
        let user_count: i64 = users::table
            .filter(users::organization_id.eq(id))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        if user_count > 0 {
            return Err(AppError::Conflict(
                "Cannot delete organization with existing users".to_string(),
            ));
        }

        let deleted = diesel::delete(organizations::table.find(id))
            .execute(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound(format!("Organization {} not found", id)));
        }

        Ok(())
    }

    /// Get user count for an organization (active users only)
    pub async fn get_user_count(&self, org_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        users::table
            .filter(users::organization_id.eq(org_id))
            .filter(users::deleted_at.is_null())
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// List organizations with user counts
    pub async fn list_with_user_counts(
        &self,
        filter: &OrganizationFilter,
        pagination: &OrganizationPagination,
    ) -> Result<PaginatedOrganizations, AppError> {
        let (organizations, total) = self.list(filter, pagination).await?;

        // Get user counts for each organization
        let mut responses = Vec::with_capacity(organizations.len());
        for org in organizations {
            let user_count = self.get_user_count(org.id).await?;
            responses.push(
                OrganizationResponse::from_organization(&org)
                    .with_user_count(user_count),
            );
        }

        let total_pages = (total + pagination.per_page - 1) / pagination.per_page;

        Ok(PaginatedOrganizations {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }
}
