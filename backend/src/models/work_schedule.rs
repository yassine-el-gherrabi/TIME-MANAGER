use chrono::{DateTime, NaiveTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{work_schedule_days, work_schedules};

/// WorkSchedule entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = work_schedules)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkSchedule {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// NewWorkSchedule for creating schedules
#[derive(Debug, Insertable)]
#[diesel(table_name = work_schedules)]
pub struct NewWorkSchedule {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
}

/// WorkSchedule update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = work_schedules)]
pub struct WorkScheduleUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub is_default: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// WorkScheduleDay entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = work_schedule_days)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkScheduleDay {
    pub id: Uuid,
    pub work_schedule_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub break_minutes: i32,
}

/// NewWorkScheduleDay for creating schedule days
#[derive(Debug, Insertable)]
#[diesel(table_name = work_schedule_days)]
pub struct NewWorkScheduleDay {
    pub work_schedule_id: Uuid,
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub break_minutes: i32,
}

/// WorkScheduleDay update struct
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = work_schedule_days)]
pub struct WorkScheduleDayUpdate {
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub break_minutes: Option<i32>,
}

/// WorkSchedule with its days for complete view
#[derive(Debug, Clone, Serialize)]
pub struct WorkScheduleWithDays {
    #[serde(flatten)]
    pub schedule: WorkSchedule,
    pub days: Vec<WorkScheduleDay>,
}

/// Day configuration for creating/updating schedules
#[derive(Debug, Clone, Deserialize)]
pub struct DayConfig {
    pub day_of_week: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub break_minutes: i32,
}
