use chrono::{DateTime, NaiveTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{
    DayConfig, NewWorkSchedule, NewWorkScheduleDay, WorkScheduleDay, WorkScheduleDayUpdate,
    WorkScheduleUpdate, WorkScheduleWithDays,
};
use crate::repositories::WorkScheduleRepository;

/// Request to create a work schedule
#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub days: Vec<DayConfig>,
}

/// Request to update a work schedule
#[derive(Debug, Deserialize)]
pub struct UpdateScheduleRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_default: Option<bool>,
}

/// Request to add a day to schedule
#[derive(Debug, Deserialize)]
pub struct AddDayRequest {
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub break_minutes: i32,
}

/// Request to update a schedule day
#[derive(Debug, Deserialize)]
pub struct UpdateDayRequest {
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub break_minutes: Option<i32>,
}

/// Service for work schedule operations
pub struct WorkScheduleService {
    schedule_repo: WorkScheduleRepository,
}

impl WorkScheduleService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            schedule_repo: WorkScheduleRepository::new(pool),
        }
    }

    /// Create a new work schedule with days
    pub async fn create_schedule(
        &self,
        org_id: Uuid,
        request: CreateScheduleRequest,
    ) -> Result<WorkScheduleWithDays, AppError> {
        // Validate days
        for day in &request.days {
            if day.day_of_week < 0 || day.day_of_week > 6 {
                return Err(AppError::ValidationError(
                    "day_of_week must be between 0 (Monday) and 6 (Sunday)".to_string(),
                ));
            }
            if day.start_time >= day.end_time {
                return Err(AppError::ValidationError(
                    "start_time must be before end_time".to_string(),
                ));
            }
            if day.break_minutes < 0 {
                return Err(AppError::ValidationError(
                    "break_minutes cannot be negative".to_string(),
                ));
            }
        }

        // Create the schedule
        let new_schedule = NewWorkSchedule {
            organization_id: org_id,
            name: request.name,
            description: request.description,
            is_default: request.is_default,
        };

        let schedule = self.schedule_repo.create(new_schedule).await?;

        // Add days
        let mut days = Vec::with_capacity(request.days.len());
        for day_config in request.days {
            let new_day = NewWorkScheduleDay {
                work_schedule_id: schedule.id,
                day_of_week: day_config.day_of_week,
                start_time: day_config.start_time,
                end_time: day_config.end_time,
                break_minutes: day_config.break_minutes,
            };
            let day = self.schedule_repo.add_day(new_day).await?;
            days.push(day);
        }

        Ok(WorkScheduleWithDays { schedule, days })
    }

    /// Get a schedule by ID with its days
    pub async fn get_schedule(
        &self,
        org_id: Uuid,
        schedule_id: Uuid,
    ) -> Result<WorkScheduleWithDays, AppError> {
        let schedule = self.schedule_repo.find_by_id(org_id, schedule_id).await?;
        let days = self.schedule_repo.get_days(schedule_id).await?;

        Ok(WorkScheduleWithDays { schedule, days })
    }

    /// List all schedules for organization
    pub async fn list_schedules(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<WorkScheduleWithDays>, AppError> {
        let schedules = self.schedule_repo.list(org_id).await?;

        let mut results = Vec::with_capacity(schedules.len());
        for schedule in schedules {
            let days = self.schedule_repo.get_days(schedule.id).await?;
            results.push(WorkScheduleWithDays { schedule, days });
        }

        Ok(results)
    }

    /// Update a schedule
    pub async fn update_schedule(
        &self,
        org_id: Uuid,
        schedule_id: Uuid,
        request: UpdateScheduleRequest,
    ) -> Result<WorkScheduleWithDays, AppError> {
        let update = WorkScheduleUpdate {
            name: request.name,
            description: request.description,
            is_default: request.is_default,
            updated_at: Some(Utc::now()),
        };

        let schedule = self
            .schedule_repo
            .update(org_id, schedule_id, update)
            .await?;
        let days = self.schedule_repo.get_days(schedule_id).await?;

        Ok(WorkScheduleWithDays { schedule, days })
    }

    /// Delete a schedule
    pub async fn delete_schedule(&self, org_id: Uuid, schedule_id: Uuid) -> Result<(), AppError> {
        self.schedule_repo.delete(org_id, schedule_id).await
    }

    /// Add a day to a schedule
    pub async fn add_day(
        &self,
        org_id: Uuid,
        schedule_id: Uuid,
        request: AddDayRequest,
    ) -> Result<WorkScheduleDay, AppError> {
        // Verify schedule exists
        let _ = self.schedule_repo.find_by_id(org_id, schedule_id).await?;

        // Validate
        if request.day_of_week < 0 || request.day_of_week > 6 {
            return Err(AppError::ValidationError(
                "day_of_week must be between 0 (Monday) and 6 (Sunday)".to_string(),
            ));
        }
        if request.start_time >= request.end_time {
            return Err(AppError::ValidationError(
                "start_time must be before end_time".to_string(),
            ));
        }

        let new_day = NewWorkScheduleDay {
            work_schedule_id: schedule_id,
            day_of_week: request.day_of_week,
            start_time: request.start_time,
            end_time: request.end_time,
            break_minutes: request.break_minutes,
        };

        self.schedule_repo.add_day(new_day).await
    }

    /// Update a schedule day
    pub async fn update_day(
        &self,
        day_id: Uuid,
        request: UpdateDayRequest,
    ) -> Result<WorkScheduleDay, AppError> {
        let update = WorkScheduleDayUpdate {
            start_time: request.start_time,
            end_time: request.end_time,
            break_minutes: request.break_minutes,
        };

        self.schedule_repo.update_day(day_id, update).await
    }

    /// Remove a day from a schedule
    pub async fn remove_day(&self, day_id: Uuid) -> Result<(), AppError> {
        self.schedule_repo.remove_day(day_id).await
    }

    /// Assign a schedule to a user
    pub async fn assign_to_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        schedule_id: Uuid,
    ) -> Result<(), AppError> {
        self.schedule_repo
            .assign_to_user(org_id, user_id, schedule_id)
            .await
    }

    /// Unassign schedule from a user
    pub async fn unassign_from_user(&self, org_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.schedule_repo.unassign_from_user(org_id, user_id).await
    }

    /// Get user's schedule
    pub async fn get_user_schedule(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<WorkScheduleWithDays>, AppError> {
        let schedule = self
            .schedule_repo
            .get_user_schedule(org_id, user_id)
            .await?;

        match schedule {
            Some(s) => {
                let days = self.schedule_repo.get_days(s.id).await?;
                Ok(Some(WorkScheduleWithDays { schedule: s, days }))
            }
            None => Ok(None),
        }
    }

    /// Get default schedule for organization
    pub async fn get_default_schedule(
        &self,
        org_id: Uuid,
    ) -> Result<Option<WorkScheduleWithDays>, AppError> {
        let schedule = self.schedule_repo.get_default(org_id).await?;

        match schedule {
            Some(s) => {
                let days = self.schedule_repo.get_days(s.id).await?;
                Ok(Some(WorkScheduleWithDays { schedule: s, days }))
            }
            None => Ok(None),
        }
    }

    /// Calculate theoretical hours for a user in a period
    pub async fn get_theoretical_hours(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<f64, AppError> {
        self.schedule_repo
            .get_theoretical_hours(org_id, user_id, start, end)
            .await
    }
}
