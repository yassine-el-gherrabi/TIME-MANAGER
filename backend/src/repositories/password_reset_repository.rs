use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{NewPasswordResetToken, PasswordResetToken};
use crate::schema::password_reset_tokens;

/// Password reset token repository for database operations
pub struct PasswordResetRepository {
    pool: DbPool,
}

impl PasswordResetRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new password reset token
    pub async fn create(
        &self,
        new_token: NewPasswordResetToken,
    ) -> Result<PasswordResetToken, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(password_reset_tokens::table)
            .values(&new_token)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Find password reset token by token hash
    pub async fn find_by_token_hash(
        &self,
        token_hash: &str,
    ) -> Result<PasswordResetToken, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        password_reset_tokens::table
            .filter(password_reset_tokens::token_hash.eq(token_hash))
            .filter(password_reset_tokens::used_at.is_null())
            .first::<PasswordResetToken>(&mut conn)
            .await
            .map_err(|_| {
                AppError::NotFound("Password reset token not found or already used".to_string())
            })
    }

    /// Mark token as used
    pub async fn mark_as_used(&self, token_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::update(password_reset_tokens::table.find(token_id))
            .set(password_reset_tokens::used_at.eq(Some(chrono::Utc::now().naive_utc())))
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    /// Invalidate all tokens for a user
    pub async fn invalidate_all_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::update(
            password_reset_tokens::table.filter(password_reset_tokens::user_id.eq(user_id)),
        )
        .set(password_reset_tokens::used_at.eq(Some(chrono::Utc::now().naive_utc())))
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    /// Delete expired tokens (cleanup operation)
    pub async fn delete_expired(&self) -> Result<usize, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;
        let now = chrono::Utc::now().naive_utc();

        diesel::delete(
            password_reset_tokens::table.filter(password_reset_tokens::expires_at.lt(now)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)
    }

    /// Check if token is valid (not expired, not used)
    pub async fn is_valid(&self, token_hash: &str) -> Result<bool, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;
        let now = chrono::Utc::now().naive_utc();

        let count = password_reset_tokens::table
            .filter(password_reset_tokens::token_hash.eq(token_hash))
            .filter(password_reset_tokens::used_at.is_null())
            .filter(password_reset_tokens::expires_at.gt(now))
            .count()
            .get_result::<i64>(&mut conn)
            .await?;

        Ok(count > 0)
    }
}
