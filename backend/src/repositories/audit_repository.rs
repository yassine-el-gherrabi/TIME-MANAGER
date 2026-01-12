use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{AuditLog, AuditLogFilter, AuditUserInfo, NewAuditLog, Pagination};
use crate::schema::{audit_logs, users};
use crate::utils::{end_of_day_tz, start_of_day_tz};

/// Maximum number of records for CSV export
const MAX_EXPORT_RECORDS: i64 = 10_000;

/// Audit log repository for database operations
pub struct AuditRepository {
    pool: DbPool,
}

impl AuditRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new audit log entry
    pub async fn create(&self, new_audit_log: NewAuditLog) -> Result<AuditLog, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(audit_logs::table)
            .values(&new_audit_log)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// List audit logs with filters and pagination
    /// Returns (audit_logs, total_count)
    pub async fn list(
        &self,
        org_id: Option<Uuid>,
        filter: &AuditLogFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<AuditLog>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Build base query
        let mut query = audit_logs::table.into_boxed();
        let mut count_query = audit_logs::table.into_boxed();

        // Apply organization filter (Super Admin can see all, others see their org only)
        if let Some(org_id) = org_id {
            query = query.filter(audit_logs::organization_id.eq(org_id));
            count_query = count_query.filter(audit_logs::organization_id.eq(org_id));
        }

        // Apply entity_type filter
        if let Some(ref entity_type) = filter.entity_type {
            query = query.filter(audit_logs::entity_type.eq(entity_type));
            count_query = count_query.filter(audit_logs::entity_type.eq(entity_type));
        }

        // Apply action filter
        if let Some(action) = filter.action {
            query = query.filter(audit_logs::action.eq(action));
            count_query = count_query.filter(audit_logs::action.eq(action));
        }

        // Apply user_id filter
        if let Some(user_id) = filter.user_id {
            query = query.filter(audit_logs::user_id.eq(user_id));
            count_query = count_query.filter(audit_logs::user_id.eq(user_id));
        }

        // Apply entity_id filter
        if let Some(entity_id) = filter.entity_id {
            query = query.filter(audit_logs::entity_id.eq(entity_id));
            count_query = count_query.filter(audit_logs::entity_id.eq(entity_id));
        }

        // Apply date range filters
        if let Some(start_date) = filter.start_date {
            let start_datetime = start_of_day_tz(start_date);
            query = query.filter(audit_logs::created_at.ge(start_datetime));
            count_query = count_query.filter(audit_logs::created_at.ge(start_datetime));
        }

        if let Some(end_date) = filter.end_date {
            let end_datetime = end_of_day_tz(end_date);
            query = query.filter(audit_logs::created_at.le(end_datetime));
            count_query = count_query.filter(audit_logs::created_at.le(end_datetime));
        }

        // Get total count
        let total: i64 = count_query
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        // Apply pagination and ordering
        let offset = (pagination.page - 1) * pagination.per_page;
        let results = query
            .order(audit_logs::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<AuditLog>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((results, total))
    }

    /// List audit logs for CSV export (no pagination, with hard limit)
    pub async fn list_for_export(
        &self,
        org_id: Option<Uuid>,
        filter: &AuditLogFilter,
    ) -> Result<Vec<AuditLog>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Build base query
        let mut query = audit_logs::table.into_boxed();

        // Apply organization filter
        if let Some(org_id) = org_id {
            query = query.filter(audit_logs::organization_id.eq(org_id));
        }

        // Apply entity_type filter
        if let Some(ref entity_type) = filter.entity_type {
            query = query.filter(audit_logs::entity_type.eq(entity_type));
        }

        // Apply action filter
        if let Some(action) = filter.action {
            query = query.filter(audit_logs::action.eq(action));
        }

        // Apply user_id filter
        if let Some(user_id) = filter.user_id {
            query = query.filter(audit_logs::user_id.eq(user_id));
        }

        // Apply entity_id filter
        if let Some(entity_id) = filter.entity_id {
            query = query.filter(audit_logs::entity_id.eq(entity_id));
        }

        // Apply date range filters
        if let Some(start_date) = filter.start_date {
            let start_datetime = start_of_day_tz(start_date);
            query = query.filter(audit_logs::created_at.ge(start_datetime));
        }

        if let Some(end_date) = filter.end_date {
            let end_datetime = end_of_day_tz(end_date);
            query = query.filter(audit_logs::created_at.le(end_datetime));
        }

        // Apply ordering and limit
        let results = query
            .order(audit_logs::created_at.desc())
            .limit(MAX_EXPORT_RECORDS)
            .load::<AuditLog>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(results)
    }

    /// Get user info for a list of user IDs
    pub async fn get_users_info(
        &self,
        user_ids: Vec<Uuid>,
    ) -> Result<Vec<AuditUserInfo>, AppError> {
        if user_ids.is_empty() {
            return Ok(vec![]);
        }

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let results: Vec<(Uuid, String, String, String)> = users::table
            .filter(users::id.eq_any(&user_ids))
            .select((users::id, users::email, users::first_name, users::last_name))
            .load(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(results
            .into_iter()
            .map(|(id, email, first_name, last_name)| AuditUserInfo {
                id,
                email,
                first_name,
                last_name,
            })
            .collect())
    }
}
