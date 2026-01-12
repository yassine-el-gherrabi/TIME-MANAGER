use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{LoginAttempt, NewLoginAttempt};
use crate::schema::login_attempts;

/// Login attempt repository for tracking and rate limiting
pub struct LoginAttemptRepository {
    pool: DbPool,
}

impl LoginAttemptRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Record a login attempt
    pub async fn record(&self, new_attempt: NewLoginAttempt) -> Result<LoginAttempt, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(login_attempts::table)
            .values(&new_attempt)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Count failed login attempts for email within timeframe (minutes, case-insensitive)
    pub async fn count_failed_attempts_for_email(
        &self,
        email: &str,
        within_minutes: i64,
    ) -> Result<i64, AppError> {
        let email_lower = email.to_lowercase();
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;
        let cutoff_time =
            chrono::Utc::now().naive_utc() - chrono::Duration::minutes(within_minutes);

        login_attempts::table
            .filter(login_attempts::email.eq(&email_lower))
            .filter(login_attempts::successful.eq(false))
            .filter(login_attempts::attempted_at.gt(cutoff_time))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Count failed login attempts for IP within timeframe (minutes)
    pub async fn count_failed_attempts_for_ip(
        &self,
        ip_address: &str,
        within_minutes: i64,
    ) -> Result<i64, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;
        let cutoff_time =
            chrono::Utc::now().naive_utc() - chrono::Duration::minutes(within_minutes);

        login_attempts::table
            .filter(login_attempts::ip_address.eq(ip_address))
            .filter(login_attempts::successful.eq(false))
            .filter(login_attempts::attempted_at.gt(cutoff_time))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Get recent login attempts for email (case-insensitive)
    pub async fn get_recent_attempts_for_email(
        &self,
        email: &str,
        limit: i64,
    ) -> Result<Vec<LoginAttempt>, AppError> {
        let email_lower = email.to_lowercase();
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        login_attempts::table
            .filter(login_attempts::email.eq(&email_lower))
            .order(login_attempts::attempted_at.desc())
            .limit(limit)
            .load::<LoginAttempt>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Delete old login attempts (cleanup operation)
    pub async fn delete_older_than(&self, days: i64) -> Result<usize, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;
        let cutoff_time = chrono::Utc::now().naive_utc() - chrono::Duration::days(days);

        diesel::delete(login_attempts::table.filter(login_attempts::attempted_at.lt(cutoff_time)))
            .execute(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }
}
