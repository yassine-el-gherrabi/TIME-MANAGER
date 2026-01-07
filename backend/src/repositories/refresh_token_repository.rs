use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{NewRefreshToken, RefreshToken};
use crate::schema::refresh_tokens;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Refresh token repository for database operations
pub struct RefreshTokenRepository {
    pool: DbPool,
}

impl RefreshTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new refresh token
    pub async fn create(&self, new_token: NewRefreshToken) -> Result<RefreshToken, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(refresh_tokens::table)
            .values(&new_token)
            .get_result(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Find refresh token by token hash
    pub async fn find_by_token_hash(&self, token_hash: &str) -> Result<RefreshToken, AppError> {
        let mut conn = self.pool.get()?;

        refresh_tokens::table
            .filter(refresh_tokens::token_hash.eq(token_hash))
            .first::<RefreshToken>(&mut conn)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))
    }

    /// Revoke a refresh token
    pub async fn revoke(&self, token_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(refresh_tokens::table.find(token_id))
            .set(refresh_tokens::revoked_at.eq(Some(chrono::Utc::now().naive_utc())))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Revoke all refresh tokens for a user
    pub async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(refresh_tokens::table.filter(refresh_tokens::user_id.eq(user_id)))
            .set(refresh_tokens::revoked_at.eq(Some(chrono::Utc::now().naive_utc())))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Get all active tokens for a user
    pub async fn get_active_tokens_for_user(&self, user_id: Uuid) -> Result<Vec<RefreshToken>, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        refresh_tokens::table
            .filter(refresh_tokens::user_id.eq(user_id))
            .filter(refresh_tokens::revoked_at.is_null())
            .filter(refresh_tokens::expires_at.gt(now))
            .load::<RefreshToken>(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Delete expired tokens (cleanup operation)
    pub async fn delete_expired(&self) -> Result<usize, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        diesel::delete(refresh_tokens::table.filter(refresh_tokens::expires_at.lt(now)))
            .execute(&mut conn)
            .map_err(AppError::DatabaseError)
    }
}
