use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{InviteToken, NewInviteToken};
use crate::schema::invite_tokens;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Invite token repository for database operations
pub struct InviteTokenRepository {
    pool: DbPool,
}

impl InviteTokenRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new invite token
    pub async fn create(&self, new_token: NewInviteToken) -> Result<InviteToken, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(invite_tokens::table)
            .values(&new_token)
            .get_result(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Find invite token by token hash
    pub async fn find_by_token_hash(&self, token_hash: &str) -> Result<InviteToken, AppError> {
        let mut conn = self.pool.get()?;

        invite_tokens::table
            .filter(invite_tokens::token_hash.eq(token_hash))
            .filter(invite_tokens::used_at.is_null())
            .first::<InviteToken>(&mut conn)
            .map_err(|_| AppError::NotFound("Invite token not found or already used".to_string()))
    }

    /// Find valid invite token for user
    pub async fn find_valid_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<InviteToken>, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        invite_tokens::table
            .filter(invite_tokens::user_id.eq(user_id))
            .filter(invite_tokens::used_at.is_null())
            .filter(invite_tokens::expires_at.gt(now))
            .first::<InviteToken>(&mut conn)
            .optional()
            .map_err(AppError::DatabaseError)
    }

    /// Mark token as used
    pub async fn mark_as_used(&self, token_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(invite_tokens::table.find(token_id))
            .set(invite_tokens::used_at.eq(Some(chrono::Utc::now().naive_utc())))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Invalidate all tokens for a user
    pub async fn invalidate_all_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(invite_tokens::table.filter(invite_tokens::user_id.eq(user_id)))
            .set(invite_tokens::used_at.eq(Some(chrono::Utc::now().naive_utc())))
            .execute(&mut conn)?;

        Ok(())
    }

    /// Delete expired tokens (cleanup operation)
    pub async fn delete_expired(&self) -> Result<usize, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        diesel::delete(invite_tokens::table.filter(invite_tokens::expires_at.lt(now)))
            .execute(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Check if token is valid (not expired, not used)
    pub async fn is_valid(&self, token_hash: &str) -> Result<bool, AppError> {
        let mut conn = self.pool.get()?;
        let now = chrono::Utc::now().naive_utc();

        let count = invite_tokens::table
            .filter(invite_tokens::token_hash.eq(token_hash))
            .filter(invite_tokens::used_at.is_null())
            .filter(invite_tokens::expires_at.gt(now))
            .count()
            .get_result::<i64>(&mut conn)?;

        Ok(count > 0)
    }
}
