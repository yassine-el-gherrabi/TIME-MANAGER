use chrono::{DateTime, Datelike, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    NewWorkSchedule, NewWorkScheduleDay, WorkSchedule, WorkScheduleDay, WorkScheduleDayUpdate,
    WorkScheduleUpdate,
};
use crate::schema::{users, work_schedule_days, work_schedules};

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Work schedule repository for database operations
pub struct WorkScheduleRepository {
    pool: DbPool,
}

impl WorkScheduleRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new work schedule
    pub async fn create(&self, new_schedule: NewWorkSchedule) -> Result<WorkSchedule, AppError> {
        let mut conn = self.pool.get()?;

        // If this schedule is default, unset any existing default first
        if new_schedule.is_default {
            diesel::update(
                work_schedules::table
                    .filter(work_schedules::organization_id.eq(new_schedule.organization_id))
                    .filter(work_schedules::is_default.eq(true)),
            )
            .set(work_schedules::is_default.eq(false))
            .execute(&mut conn)?;
        }

        diesel::insert_into(work_schedules::table)
            .values(&new_schedule)
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict(
                    "A work schedule with this name already exists".to_string(),
                ),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Find work schedule by ID within organization
    pub async fn find_by_id(
        &self,
        org_id: Uuid,
        schedule_id: Uuid,
    ) -> Result<WorkSchedule, AppError> {
        let mut conn = self.pool.get()?;

        work_schedules::table
            .filter(work_schedules::organization_id.eq(org_id))
            .find(schedule_id)
            .first::<WorkSchedule>(&mut conn)
            .map_err(|_| AppError::NotFound("Work schedule not found".to_string()))
    }

    /// List all work schedules for organization
    pub async fn list(&self, org_id: Uuid) -> Result<Vec<WorkSchedule>, AppError> {
        let mut conn = self.pool.get()?;

        let schedules = work_schedules::table
            .filter(work_schedules::organization_id.eq(org_id))
            .order(work_schedules::name.asc())
            .load::<WorkSchedule>(&mut conn)?;

        Ok(schedules)
    }

    /// Update a work schedule
    pub async fn update(
        &self,
        org_id: Uuid,
        schedule_id: Uuid,
        mut update: WorkScheduleUpdate,
    ) -> Result<WorkSchedule, AppError> {
        let mut conn = self.pool.get()?;

        // If setting as default, unset any existing default first
        if update.is_default == Some(true) {
            diesel::update(
                work_schedules::table
                    .filter(work_schedules::organization_id.eq(org_id))
                    .filter(work_schedules::is_default.eq(true))
                    .filter(work_schedules::id.ne(schedule_id)),
            )
            .set(work_schedules::is_default.eq(false))
            .execute(&mut conn)?;
        }

        update.updated_at = Some(Utc::now());

        let affected = diesel::update(
            work_schedules::table
                .filter(work_schedules::organization_id.eq(org_id))
                .filter(work_schedules::id.eq(schedule_id)),
        )
        .set(&update)
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Work schedule not found".to_string()));
        }

        self.find_by_id(org_id, schedule_id).await
    }

    /// Delete a work schedule
    pub async fn delete(&self, org_id: Uuid, schedule_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let deleted = diesel::delete(
            work_schedules::table
                .filter(work_schedules::organization_id.eq(org_id))
                .filter(work_schedules::id.eq(schedule_id)),
        )
        .execute(&mut conn)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Work schedule not found".to_string()));
        }

        Ok(())
    }

    /// Get default schedule for organization
    pub async fn get_default(&self, org_id: Uuid) -> Result<Option<WorkSchedule>, AppError> {
        let mut conn = self.pool.get()?;

        let schedule = work_schedules::table
            .filter(work_schedules::organization_id.eq(org_id))
            .filter(work_schedules::is_default.eq(true))
            .first::<WorkSchedule>(&mut conn)
            .optional()?;

        Ok(schedule)
    }

    /// Set a schedule as default
    pub async fn set_default(
        &self,
        org_id: Uuid,
        schedule_id: Uuid,
    ) -> Result<WorkSchedule, AppError> {
        let mut conn = self.pool.get()?;

        // Unset any existing default
        diesel::update(
            work_schedules::table
                .filter(work_schedules::organization_id.eq(org_id))
                .filter(work_schedules::is_default.eq(true)),
        )
        .set(work_schedules::is_default.eq(false))
        .execute(&mut conn)?;

        // Set new default
        let affected = diesel::update(
            work_schedules::table
                .filter(work_schedules::organization_id.eq(org_id))
                .filter(work_schedules::id.eq(schedule_id)),
        )
        .set((
            work_schedules::is_default.eq(true),
            work_schedules::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Work schedule not found".to_string()));
        }

        self.find_by_id(org_id, schedule_id).await
    }

    /// Add a day to a schedule
    pub async fn add_day(&self, new_day: NewWorkScheduleDay) -> Result<WorkScheduleDay, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(work_schedule_days::table)
            .values(&new_day)
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict("Schedule already has this day configured".to_string()),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Update a schedule day
    pub async fn update_day(
        &self,
        day_id: Uuid,
        update: WorkScheduleDayUpdate,
    ) -> Result<WorkScheduleDay, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(work_schedule_days::table.find(day_id))
            .set(&update)
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => {
                    AppError::NotFound("Schedule day not found".to_string())
                }
                _ => AppError::DatabaseError(e),
            })
    }

    /// Remove a day from a schedule
    pub async fn remove_day(&self, day_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let deleted = diesel::delete(work_schedule_days::table.find(day_id)).execute(&mut conn)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Schedule day not found".to_string()));
        }

        Ok(())
    }

    /// Get all days for a schedule
    pub async fn get_days(&self, schedule_id: Uuid) -> Result<Vec<WorkScheduleDay>, AppError> {
        let mut conn = self.pool.get()?;

        let days = work_schedule_days::table
            .filter(work_schedule_days::work_schedule_id.eq(schedule_id))
            .order(work_schedule_days::day_of_week.asc())
            .load::<WorkScheduleDay>(&mut conn)?;

        Ok(days)
    }

    /// Assign schedule to user
    pub async fn assign_to_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        schedule_id: Uuid,
    ) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        // Verify schedule belongs to organization
        let _ = self.find_by_id(org_id, schedule_id).await?;

        diesel::update(
            users::table
                .filter(users::id.eq(user_id))
                .filter(users::organization_id.eq(org_id)),
        )
        .set(users::work_schedule_id.eq(Some(schedule_id)))
        .execute(&mut conn)?;

        Ok(())
    }

    /// Unassign schedule from user
    pub async fn unassign_from_user(&self, org_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let affected = diesel::update(
            users::table
                .filter(users::id.eq(user_id))
                .filter(users::organization_id.eq(org_id)),
        )
        .set(users::work_schedule_id.eq(None::<Uuid>))
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    /// Get user's assigned schedule
    pub async fn get_user_schedule(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<WorkSchedule>, AppError> {
        let mut conn = self.pool.get()?;

        let schedule_id: Option<Uuid> = users::table
            .filter(users::id.eq(user_id))
            .filter(users::organization_id.eq(org_id))
            .select(users::work_schedule_id)
            .first(&mut conn)?;

        match schedule_id {
            Some(id) => {
                let schedule = self.find_by_id(org_id, id).await?;
                Ok(Some(schedule))
            }
            None => Ok(None),
        }
    }

    /// Calculate theoretical hours for a user in a date range
    pub async fn get_theoretical_hours(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<f64, AppError> {
        // Get user's schedule (or default)
        let schedule = match self.get_user_schedule(org_id, user_id).await? {
            Some(s) => s,
            None => match self.get_default(org_id).await? {
                Some(s) => s,
                None => return Ok(0.0),
            },
        };

        let days = self.get_days(schedule.id).await?;
        if days.is_empty() {
            return Ok(0.0);
        }

        // Calculate total hours
        let mut total_minutes: i64 = 0;
        let mut current = start.date_naive();
        let end_date = end.date_naive();

        while current <= end_date {
            let weekday = current.weekday().num_days_from_monday() as i16;
            if let Some(day) = days.iter().find(|d| d.day_of_week == weekday) {
                let work_minutes =
                    (day.end_time - day.start_time).num_minutes() - day.break_minutes as i64;
                if work_minutes > 0 {
                    total_minutes += work_minutes;
                }
            }
            current = current.succ_opt().unwrap_or(current);
        }

        Ok(total_minutes as f64 / 60.0)
    }
}
