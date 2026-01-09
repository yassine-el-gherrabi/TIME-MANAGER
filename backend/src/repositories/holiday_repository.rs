use chrono::{Datelike, NaiveDate};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Holiday, HolidayFilter, HolidayUpdate, NewHoliday};
use crate::schema::holidays;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Holiday repository for database operations
pub struct HolidayRepository {
    pool: DbPool,
}

impl HolidayRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new holiday
    pub async fn create(&self, new_holiday: NewHoliday) -> Result<Holiday, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(holidays::table)
            .values(&new_holiday)
            .get_result(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Find holiday by ID within organization
    pub async fn find_by_id(
        &self,
        org_id: Uuid,
        holiday_id: Uuid,
    ) -> Result<Holiday, AppError> {
        let mut conn = self.pool.get()?;

        holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .find(holiday_id)
            .first::<Holiday>(&mut conn)
            .map_err(|_| AppError::NotFound("Holiday not found".to_string()))
    }

    /// List holidays with filters
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: &HolidayFilter,
    ) -> Result<Vec<Holiday>, AppError> {
        let mut conn = self.pool.get()?;

        let mut query = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .into_boxed();

        if let Some(start_date) = filter.start_date {
            query = query.filter(holidays::date.ge(start_date));
        }
        if let Some(end_date) = filter.end_date {
            query = query.filter(holidays::date.le(end_date));
        }
        if let Some(is_recurring) = filter.is_recurring {
            query = query.filter(holidays::is_recurring.eq(is_recurring));
        }

        let results = query
            .order(holidays::date.asc())
            .load::<Holiday>(&mut conn)?;

        Ok(results)
    }

    /// List all holidays for a year (including recurring ones)
    pub async fn list_for_year(
        &self,
        org_id: Uuid,
        year: i32,
    ) -> Result<Vec<Holiday>, AppError> {
        let mut conn = self.pool.get()?;

        let start = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();

        // Get non-recurring holidays in this year
        let mut holidays_list: Vec<Holiday> = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .filter(holidays::is_recurring.eq(false))
            .filter(holidays::date.ge(start))
            .filter(holidays::date.le(end))
            .load::<Holiday>(&mut conn)?;

        // Get all recurring holidays
        let recurring: Vec<Holiday> = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .filter(holidays::is_recurring.eq(true))
            .load::<Holiday>(&mut conn)?;

        holidays_list.extend(recurring);
        holidays_list.sort_by_key(|h| h.date);

        Ok(holidays_list)
    }

    /// Get holidays for a date range (for working days calculation)
    pub async fn list_range(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<NaiveDate>, AppError> {
        let mut conn = self.pool.get()?;

        // Get non-recurring holidays in range
        let non_recurring: Vec<NaiveDate> = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .filter(holidays::is_recurring.eq(false))
            .filter(holidays::date.ge(start_date))
            .filter(holidays::date.le(end_date))
            .select(holidays::date)
            .load(&mut conn)?;

        // Get recurring holidays
        let recurring: Vec<Holiday> = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .filter(holidays::is_recurring.eq(true))
            .load::<Holiday>(&mut conn)?;

        // Calculate recurring dates for each year in range
        let mut all_dates: Vec<NaiveDate> = non_recurring;

        for holiday in recurring {
            let mut current_year = start_date.year();
            while current_year <= end_date.year() {
                if let Some(date) = NaiveDate::from_ymd_opt(
                    current_year,
                    holiday.date.month(),
                    holiday.date.day(),
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

    /// Check if a date is a holiday
    pub async fn is_holiday(
        &self,
        org_id: Uuid,
        date: NaiveDate,
    ) -> Result<bool, AppError> {
        let mut conn = self.pool.get()?;

        // Check exact date (non-recurring)
        let exact_count: i64 = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .filter(holidays::is_recurring.eq(false))
            .filter(holidays::date.eq(date))
            .count()
            .get_result(&mut conn)?;

        if exact_count > 0 {
            return Ok(true);
        }

        // Check recurring (month and day match)
        let month = date.month() as i32;
        let day = date.day() as i32;

        let recurring: Vec<Holiday> = holidays::table
            .filter(holidays::organization_id.eq(org_id))
            .filter(holidays::is_recurring.eq(true))
            .load::<Holiday>(&mut conn)?;

        for holiday in recurring {
            if holiday.date.month() as i32 == month && holiday.date.day() as i32 == day {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Update a holiday
    pub async fn update(
        &self,
        org_id: Uuid,
        holiday_id: Uuid,
        update: HolidayUpdate,
    ) -> Result<Holiday, AppError> {
        let mut conn = self.pool.get()?;

        let affected = diesel::update(
            holidays::table
                .filter(holidays::organization_id.eq(org_id))
                .filter(holidays::id.eq(holiday_id)),
        )
        .set(&update)
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Holiday not found".to_string()));
        }

        self.find_by_id(org_id, holiday_id).await
    }

    /// Delete a holiday
    pub async fn delete(&self, org_id: Uuid, holiday_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let deleted = diesel::delete(
            holidays::table
                .filter(holidays::organization_id.eq(org_id))
                .filter(holidays::id.eq(holiday_id)),
        )
        .execute(&mut conn)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Holiday not found".to_string()));
        }

        Ok(())
    }
}
