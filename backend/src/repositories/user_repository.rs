use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;
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

    /// Find user by ID
    pub async fn find_by_id(&self, user_id: Uuid) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        users::table
            .find(user_id)
            .first::<User>(&mut conn)
            .map_err(|_| AppError::NotFound("User not found".to_string()))
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let mut conn = self.pool.get()?;

        users::table
            .filter(users::email.eq(email))
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
    pub async fn lock_account(&self, user_id: Uuid, locked_until: chrono::NaiveDateTime) -> Result<(), AppError> {
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
    pub async fn update_password(&self, user_id: Uuid, password_hash: &str) -> Result<(), AppError> {
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
}

// User model - this will be moved to models module later if not present
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
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
}
