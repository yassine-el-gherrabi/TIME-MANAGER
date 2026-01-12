use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{NewPasswordHistory, PasswordHistory};
use crate::schema::password_history;

/// Password history repository for preventing password reuse
pub struct PasswordHistoryRepository {
    pool: DbPool,
}

impl PasswordHistoryRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Add password to history
    pub async fn add(&self, new_history: NewPasswordHistory) -> Result<PasswordHistory, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(password_history::table)
            .values(&new_history)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Get recent password hashes for a user (for preventing reuse)
    pub async fn get_recent_hashes(
        &self,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<String>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        password_history::table
            .filter(password_history::user_id.eq(user_id))
            .order(password_history::created_at.desc())
            .limit(limit)
            .select(password_history::password_hash)
            .load::<String>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Check if password was recently used
    pub async fn was_recently_used(
        &self,
        user_id: Uuid,
        password_hash: &str,
        check_last_n: i64,
    ) -> Result<bool, AppError> {
        let recent_hashes = self.get_recent_hashes(user_id, check_last_n).await?;
        Ok(recent_hashes.contains(&password_hash.to_string()))
    }

    /// Delete old password history entries (cleanup operation)
    pub async fn delete_older_than(
        &self,
        user_id: Uuid,
        keep_last_n: i64,
    ) -> Result<usize, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Get IDs of entries to keep
        let ids_to_keep: Vec<Uuid> = password_history::table
            .filter(password_history::user_id.eq(user_id))
            .order(password_history::created_at.desc())
            .limit(keep_last_n)
            .select(password_history::id)
            .load(&mut conn)
            .await?;

        // Delete entries not in the keep list
        diesel::delete(
            password_history::table
                .filter(password_history::user_id.eq(user_id))
                .filter(password_history::id.ne_all(ids_to_keep)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    /// Get password history count for a user
    pub async fn count_for_user(&self, user_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        password_history::table
            .filter(password_history::user_id.eq(user_id))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }
}
