use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{NewUserSession, UserSession};
use crate::schema::user_sessions;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// User session repository for active session management
pub struct UserSessionRepository {
    pool: DbPool,
}

impl UserSessionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new user session
    pub async fn create(&self, new_session: NewUserSession) -> Result<UserSession, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(user_sessions::table)
            .values(&new_session)
            .get_result(&mut conn)
            .map_err(|e| AppError::DatabaseError(e))
    }

    /// Get all active sessions for a user
    pub async fn get_active_sessions_for_user(&self, user_id: Uuid) -> Result<Vec<UserSession>, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        user_sessions::table
            .filter(user_sessions::user_id.eq(user_id))
            .filter(user_sessions::expires_at.gt(now))
            .order(user_sessions::last_activity.desc())
            .load::<UserSession>(&mut conn)
            .map_err(|e| AppError::DatabaseError(e))
    }

    /// Update session last activity
    pub async fn update_last_activity(&self, session_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(user_sessions::table.find(session_id))
            .set(user_sessions::last_activity.eq(chrono::Utc::now().naive_utc()))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Delete session by ID
    pub async fn delete(&self, session_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::delete(user_sessions::table.find(session_id))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Delete all sessions for a user
    pub async fn delete_all_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::delete(user_sessions::table.filter(user_sessions::user_id.eq(user_id)))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Delete session by refresh token ID
    pub async fn delete_by_refresh_token(&self, refresh_token_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::delete(user_sessions::table.filter(user_sessions::refresh_token_id.eq(refresh_token_id)))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Delete expired sessions (cleanup operation)
    pub async fn delete_expired(&self) -> Result<usize, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        diesel::delete(user_sessions::table.filter(user_sessions::expires_at.lt(now)))
            .execute(&mut conn)
            .map_err(|e| AppError::DatabaseError(e))
    }

    /// Count active sessions for a user
    pub async fn count_active_sessions(&self, user_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        user_sessions::table
            .filter(user_sessions::user_id.eq(user_id))
            .filter(user_sessions::expires_at.gt(now))
            .count()
            .get_result(&mut conn)
            .map_err(|e| AppError::DatabaseError(e))
    }
}
