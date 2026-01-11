use bigdecimal::BigDecimal;
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::AbsenceStatus;
use crate::error::AppError;
use crate::models::{Absence, AbsenceFilter, AbsenceUpdate, NewAbsence, Pagination};
use crate::schema::absences;

/// Absence repository for database operations
pub struct AbsenceRepository {
    pub(crate) pool: DbPool,
}

impl AbsenceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Get a reference to the pool (for services that need direct access)
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// Create a new absence request
    pub async fn create(&self, new_absence: NewAbsence) -> Result<Absence, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(absences::table)
            .values(&new_absence)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Find absence by ID within organization
    pub async fn find_by_id(
        &self,
        org_id: Uuid,
        absence_id: Uuid,
    ) -> Result<Absence, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        absences::table
            .filter(absences::organization_id.eq(org_id))
            .find(absence_id)
            .first::<Absence>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Absence not found".to_string()))
    }

    /// List absences with filters and pagination
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: &AbsenceFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<Absence>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let mut query = absences::table
            .filter(absences::organization_id.eq(org_id))
            .into_boxed();

        if let Some(user_id) = filter.user_id {
            query = query.filter(absences::user_id.eq(user_id));
        }
        if let Some(type_id) = filter.type_id {
            query = query.filter(absences::type_id.eq(type_id));
        }
        if let Some(status) = filter.status {
            query = query.filter(absences::status.eq(status));
        }
        if let Some(start_date) = filter.start_date {
            query = query.filter(absences::end_date.ge(start_date));
        }
        if let Some(end_date) = filter.end_date {
            query = query.filter(absences::start_date.le(end_date));
        }

        // Count total
        let count_query = absences::table
            .filter(absences::organization_id.eq(org_id))
            .into_boxed();

        // Re-apply filters for count
        let mut count_q = count_query;
        if let Some(user_id) = filter.user_id {
            count_q = count_q.filter(absences::user_id.eq(user_id));
        }
        if let Some(type_id) = filter.type_id {
            count_q = count_q.filter(absences::type_id.eq(type_id));
        }
        if let Some(status) = filter.status {
            count_q = count_q.filter(absences::status.eq(status));
        }
        if let Some(start_date) = filter.start_date {
            count_q = count_q.filter(absences::end_date.ge(start_date));
        }
        if let Some(end_date) = filter.end_date {
            count_q = count_q.filter(absences::start_date.le(end_date));
        }

        let total: i64 = count_q
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        // Apply pagination
        let offset = (pagination.page - 1) * pagination.per_page;
        let absences = query
            .order(absences::start_date.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<Absence>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((absences, total))
    }

    /// List pending absences for approval (for managers)
    pub async fn list_pending(
        &self,
        org_id: Uuid,
        user_ids: Option<Vec<Uuid>>,
        pagination: &Pagination,
    ) -> Result<(Vec<Absence>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let mut query = absences::table
            .filter(absences::organization_id.eq(org_id))
            .filter(absences::status.eq(AbsenceStatus::Pending))
            .into_boxed();

        if let Some(ids) = &user_ids {
            query = query.filter(absences::user_id.eq_any(ids));
        }

        // Count
        let mut count_q = absences::table
            .filter(absences::organization_id.eq(org_id))
            .filter(absences::status.eq(AbsenceStatus::Pending))
            .into_boxed();

        if let Some(ids) = &user_ids {
            count_q = count_q.filter(absences::user_id.eq_any(ids));
        }

        let total: i64 = count_q
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        let offset = (pagination.page - 1) * pagination.per_page;
        let absences = query
            .order(absences::created_at.asc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<Absence>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((absences, total))
    }

    /// Update an absence
    pub async fn update(
        &self,
        org_id: Uuid,
        absence_id: Uuid,
        mut update: AbsenceUpdate,
    ) -> Result<Absence, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        update.updated_at = Some(Utc::now());

        let affected = diesel::update(
            absences::table
                .filter(absences::organization_id.eq(org_id))
                .filter(absences::id.eq(absence_id)),
        )
        .set(&update)
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if affected == 0 {
            return Err(AppError::NotFound("Absence not found".to_string()));
        }

        self.find_by_id(org_id, absence_id).await
    }

    /// Check for overlapping absences
    pub async fn check_overlap(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        exclude_id: Option<Uuid>,
    ) -> Result<bool, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let mut query = absences::table
            .filter(absences::organization_id.eq(org_id))
            .filter(absences::user_id.eq(user_id))
            .filter(absences::status.ne(AbsenceStatus::Rejected))
            .filter(absences::status.ne(AbsenceStatus::Cancelled))
            .filter(absences::start_date.le(end_date))
            .filter(absences::end_date.ge(start_date))
            .into_boxed();

        if let Some(id) = exclude_id {
            query = query.filter(absences::id.ne(id));
        }

        let count: i64 = query
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(count > 0)
    }

    /// Get absences for a date range (for calendar view)
    pub async fn get_for_date_range(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        user_ids: Option<Vec<Uuid>>,
    ) -> Result<Vec<Absence>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let mut query = absences::table
            .filter(absences::organization_id.eq(org_id))
            .filter(absences::status.eq(AbsenceStatus::Approved))
            .filter(absences::start_date.le(end_date))
            .filter(absences::end_date.ge(start_date))
            .into_boxed();

        if let Some(ids) = user_ids {
            query = query.filter(absences::user_id.eq_any(ids));
        }

        let absences = query
            .order(absences::start_date.asc())
            .load::<Absence>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(absences)
    }

    /// Calculate total days used for a type in a year
    pub async fn get_total_used(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        type_id: Uuid,
        year: i32,
    ) -> Result<BigDecimal, AppError> {
        use bigdecimal::Zero;
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        let total: Option<BigDecimal> = absences::table
            .filter(absences::organization_id.eq(org_id))
            .filter(absences::user_id.eq(user_id))
            .filter(absences::type_id.eq(type_id))
            .filter(absences::status.eq(AbsenceStatus::Approved))
            .filter(absences::start_date.ge(start))
            .filter(absences::start_date.le(end))
            .select(diesel::dsl::sum(absences::days_count))
            .first(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(total.unwrap_or_else(BigDecimal::zero))
    }

    /// Delete an absence (admin only)
    pub async fn delete(&self, org_id: Uuid, absence_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            absences::table
                .filter(absences::organization_id.eq(org_id))
                .filter(absences::id.eq(absence_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Absence not found".to_string()));
        }

        Ok(())
    }
}
