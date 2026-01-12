use chrono::{Datelike, NaiveDate};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{ClosedDay, ClosedDayFilter, ClosedDayUpdate, NewClosedDay};
use crate::schema::closed_days;
use crate::utils::{end_of_year, start_of_year};

/// ClosedDay repository for database operations
pub struct ClosedDayRepository {
    pool: DbPool,
}

impl ClosedDayRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new closed day
    pub async fn create(&self, new_closed_day: NewClosedDay) -> Result<ClosedDay, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(closed_days::table)
            .values(&new_closed_day)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Find closed day by ID within organization
    pub async fn find_by_id(
        &self,
        org_id: Uuid,
        closed_day_id: Uuid,
    ) -> Result<ClosedDay, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .find(closed_day_id)
            .first::<ClosedDay>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Closed day not found".to_string()))
    }

    /// List closed days with filters
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: &ClosedDayFilter,
    ) -> Result<Vec<ClosedDay>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let mut query = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .into_boxed();

        if let Some(start_date) = filter.start_date {
            query = query.filter(closed_days::date.ge(start_date));
        }
        if let Some(end_date) = filter.end_date {
            query = query.filter(closed_days::date.le(end_date));
        }
        if let Some(is_recurring) = filter.is_recurring {
            query = query.filter(closed_days::is_recurring.eq(is_recurring));
        }

        let results = query
            .order(closed_days::date.asc())
            .load::<ClosedDay>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(results)
    }

    /// List all closed days for a year (including recurring ones)
    pub async fn list_for_year(&self, org_id: Uuid, year: i32) -> Result<Vec<ClosedDay>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let start = start_of_year(year)?;
        let end = end_of_year(year)?;

        // Get non-recurring closed days in this year
        let mut closed_days_list: Vec<ClosedDay> = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .filter(closed_days::is_recurring.eq(false))
            .filter(closed_days::date.ge(start))
            .filter(closed_days::date.le(end))
            .load::<ClosedDay>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        // Get all recurring closed days
        let recurring: Vec<ClosedDay> = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .filter(closed_days::is_recurring.eq(true))
            .load::<ClosedDay>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        closed_days_list.extend(recurring);
        closed_days_list.sort_by_key(|cd| cd.date);

        Ok(closed_days_list)
    }

    /// Get closed days for a date range (for working days calculation)
    pub async fn list_range(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<NaiveDate>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Get non-recurring closed days in range
        let non_recurring: Vec<NaiveDate> = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .filter(closed_days::is_recurring.eq(false))
            .filter(closed_days::date.ge(start_date))
            .filter(closed_days::date.le(end_date))
            .select(closed_days::date)
            .load(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        // Get recurring closed days
        let recurring: Vec<ClosedDay> = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .filter(closed_days::is_recurring.eq(true))
            .load::<ClosedDay>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        // Calculate recurring dates for each year in range
        let mut all_dates: Vec<NaiveDate> = non_recurring;

        for closed_day in recurring {
            let mut current_year = start_date.year();
            while current_year <= end_date.year() {
                if let Some(date) = NaiveDate::from_ymd_opt(
                    current_year,
                    closed_day.date.month(),
                    closed_day.date.day(),
                ) {
                    if date >= start_date && date <= end_date {
                        all_dates.push(date);
                    }
                }
                current_year += 1;
            }
        }

        all_dates.sort();
        all_dates.dedup();

        Ok(all_dates)
    }

    /// Check if a date is a closed day
    pub async fn is_closed_day(&self, org_id: Uuid, date: NaiveDate) -> Result<bool, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Check exact date (non-recurring)
        let exact_count: i64 = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .filter(closed_days::is_recurring.eq(false))
            .filter(closed_days::date.eq(date))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        if exact_count > 0 {
            return Ok(true);
        }

        // Check recurring (month and day match)
        let month = date.month() as i32;
        let day = date.day() as i32;

        let recurring: Vec<ClosedDay> = closed_days::table
            .filter(closed_days::organization_id.eq(org_id))
            .filter(closed_days::is_recurring.eq(true))
            .load::<ClosedDay>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        for closed_day in recurring {
            if closed_day.date.month() as i32 == month && closed_day.date.day() as i32 == day {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Update a closed day
    pub async fn update(
        &self,
        org_id: Uuid,
        closed_day_id: Uuid,
        update: ClosedDayUpdate,
    ) -> Result<ClosedDay, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let affected = diesel::update(
            closed_days::table
                .filter(closed_days::organization_id.eq(org_id))
                .filter(closed_days::id.eq(closed_day_id)),
        )
        .set(&update)
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if affected == 0 {
            return Err(AppError::NotFound("Closed day not found".to_string()));
        }

        self.find_by_id(org_id, closed_day_id).await
    }

    /// Delete a closed day
    pub async fn delete(&self, org_id: Uuid, closed_day_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            closed_days::table
                .filter(closed_days::organization_id.eq(org_id))
                .filter(closed_days::id.eq(closed_day_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Closed day not found".to_string()));
        }

        Ok(())
    }
}
