use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{NewUser, Pagination, UserFilter, UserUpdate};
use crate::schema::users;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// User repository for database operations
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Find user by ID (excludes deleted users by default)
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        users::table
            .find(user_id)
            .filter(users::deleted_at.is_null())
            .first::<User>(&mut conn)
            .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Find user by ID including deleted users (for restore operations)
    pub async fn find_by_id_including_deleted(&self, user_id: Uuid) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Find user by email (excludes deleted users)
    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        users::table
            .filter(users::email.eq(email))
            .filter(users::deleted_at.is_null())
            .first::<User>(&mut conn)
            .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Check if user account is locked
    pub async fn is_locked(&self, user_id: Uuid) -> Result<bool, AppError> {
        let mut conn = self.pool.get()?;

        let user = users::table
            .find(user_id)
            .select(users::locked_until)
            .first::<Option<chrono::NaiveDateTime>>(&mut conn)?;

        if let Some(locked_until) = user {
            Ok(locked_until > chrono::Utc::now().naive_utc())
        } else {
            Ok(false)
        }
    }

    /// Increment failed login attempts
    pub async fn increment_failed_attempts(&self, user_id: Uuid) -> Result<i32, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(users::table.find(user_id))
            .set(users::failed_login_attempts.eq(users::failed_login_attempts + 1))
            .execute(&mut conn)?;

        let attempts = users::table
            .find(user_id)
            .select(users::failed_login_attempts)
            .first::<i32>(&mut conn)?;

        Ok(attempts)
    }

    /// Reset failed login attempts
    pub async fn reset_failed_attempts(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(users::table.find(user_id))
            .set(users::failed_login_attempts.eq(0))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Lock user account until specified time
    pub async fn lock_account(
        &self,
        user_id: Uuid,
        locked_until: chrono::NaiveDateTime,
    ) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(users::table.find(user_id))
            .set(users::locked_until.eq(Some(locked_until)))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Unlock user account
    pub async fn unlock_account(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(users::table.find(user_id))
            .set((
                users::locked_until.eq::<Option<chrono::NaiveDateTime>>(None),
                users::failed_login_attempts.eq(0),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Update password and track history
    pub async fn update_password(
        &self,
        user_id: Uuid,
        password_hash: &str,
    ) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();
        let expires_at = now + chrono::Duration::days(90); // 90 days password expiry

        diesel::update(users::table.find(user_id))
            .set((
                users::password_hash.eq(password_hash),
                users::password_changed_at.eq(Some(now)),
                users::password_expires_at.eq(Some(expires_at)),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Check if password is expired
    pub async fn is_password_expired(&self, user_id: Uuid) -> Result<bool, AppError> {
        let mut conn = self.pool.get()?;

        let expires_at = users::table
            .find(user_id)
            .select(users::password_expires_at)
            .first::<Option<chrono::NaiveDateTime>>(&mut conn)?;

        if let Some(expires_at) = expires_at {
            Ok(expires_at < chrono::Utc::now().naive_utc())
        } else {
            Ok(false)
        }
    }

    /// Create a new user
    pub async fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict("A user with this email already exists".to_string()),
                _ => AppError::DatabaseError(e),
            })
    }

    /// List users with filters and pagination (excludes deleted users by default)
    pub async fn list(
        &self,
        organization_id: Uuid,
        filter: &UserFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<User>, i64), AppError> {
        self.list_with_deleted(organization_id, filter, pagination, false).await
    }

    /// List users with optional inclusion of deleted users
    pub async fn list_with_deleted(
        &self,
        organization_id: Uuid,
        filter: &UserFilter,
        pagination: &Pagination,
        include_deleted: bool,
    ) -> Result<(Vec<User>, i64), AppError> {
        let mut conn = self.pool.get()?;

        // Prepare search pattern if needed
        let search_pattern = filter
            .search
            .as_ref()
            .map(|s| format!("%{}%", s.to_lowercase()));

        // Get total count first
        let total: i64 = {
            let mut count_query = users::table
                .filter(users::organization_id.eq(organization_id))
                .into_boxed();

            // Filter out deleted users unless explicitly included
            if !include_deleted {
                count_query = count_query.filter(users::deleted_at.is_null());
            }

            if let Some(role) = filter.role {
                count_query = count_query.filter(users::role.eq(role));
            }

            if let Some(ref pattern) = search_pattern {
                count_query = count_query.filter(
                    users::email
                        .ilike(pattern)
                        .or(users::first_name.ilike(pattern))
                        .or(users::last_name.ilike(pattern)),
                );
            }

            count_query.count().get_result(&mut conn)?
        };

        // Build data query
        let mut query = users::table
            .filter(users::organization_id.eq(organization_id))
            .into_boxed();

        // Filter out deleted users unless explicitly included
        if !include_deleted {
            query = query.filter(users::deleted_at.is_null());
        }

        if let Some(role) = filter.role {
            query = query.filter(users::role.eq(role));
        }

        if let Some(ref pattern) = search_pattern {
            query = query.filter(
                users::email
                    .ilike(pattern)
                    .or(users::first_name.ilike(pattern))
                    .or(users::last_name.ilike(pattern)),
            );
        }

        // Apply pagination
        let offset = (pagination.page - 1) * pagination.per_page;
        let users = query
            .order(users::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<User>(&mut conn)?;

        Ok((users, total))
    }

    /// Update a user
    pub async fn update(&self, user_id: Uuid, update: UserUpdate) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(users::table.find(user_id))
            .set((
                &update,
                users::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => AppError::NotFound("User not found".to_string()),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict("A user with this email already exists".to_string()),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Soft delete a user (sets deleted_at timestamp)
    pub async fn soft_delete(&self, user_id: Uuid) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        diesel::update(users::table.find(user_id))
            .set((
                users::deleted_at.eq(Some(now)),
                users::updated_at.eq(now),
            ))
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound("User not found".to_string())
                }
                _ => AppError::DatabaseError(e),
            })
    }

    /// Restore a soft-deleted user (clears deleted_at timestamp)
    pub async fn restore(&self, user_id: Uuid) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        // First check if user exists and is deleted
        let user = users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        if user.deleted_at.is_none() {
            return Err(AppError::ValidationError("User is not deleted".to_string()));
        }

        diesel::update(users::table.find(user_id))
            .set((
                users::deleted_at.eq(None::<chrono::NaiveDateTime>),
                users::updated_at.eq(now),
            ))
            .get_result(&mut conn)
            .map_err(|e| AppError::DatabaseError(e))
    }

    /// Hard delete a user (permanent deletion - use with caution)
    pub async fn hard_delete(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let deleted = diesel::delete(users::table.find(user_id)).execute(&mut conn)?;

        if deleted == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    /// Check if email exists (among active users only)
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let mut conn = self.pool.get()?;

        let count = users::table
            .filter(users::email.eq(email))
            .filter(users::deleted_at.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;

        Ok(count > 0)
    }

    /// Check if email exists for another user (among active users only)
    pub async fn email_exists_for_other(
        &self,
        email: &str,
        user_id: Uuid,
    ) -> Result<bool, AppError> {
        let mut conn = self.pool.get()?;

        let count = users::table
            .filter(users::email.eq(email))
            .filter(users::id.ne(user_id))
            .filter(users::deleted_at.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;

        Ok(count > 0)
    }

    /// Count users in an organization (for organization deletion check)
    pub async fn count_by_organization(&self, organization_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self.pool.get()?;

        let count = users::table
            .filter(users::organization_id.eq(organization_id))
            .filter(users::deleted_at.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;

        Ok(count)
    }
}

// User model - this will be moved to models module later if not present
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub role: crate::domain::enums::UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub password_changed_at: Option<chrono::NaiveDateTime>,
    pub password_expires_at: Option<chrono::NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<chrono::NaiveDateTime>,
    pub work_schedule_id: Option<Uuid>,
    pub phone: Option<String>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
