use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::holidays;

/// Holiday entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = holidays)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Holiday {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}

/// NewHoliday for creating holidays
#[derive(Debug, Insertable)]
#[diesel(table_name = holidays)]
pub struct NewHoliday {
    pub organization_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
}

/// Holiday update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = holidays)]
pub struct HolidayUpdate {
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}

/// Holiday response for API
#[derive(Debug, Serialize)]
pub struct HolidayResponse {
    pub id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Holiday> for HolidayResponse {
    fn from(h: Holiday) -> Self {
        Self {
            id: h.id,
            name: h.name,
            date: h.date,
            is_recurring: h.is_recurring,
            created_at: h.created_at,
        }
    }
}

/// Holiday filter options
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HolidayFilter {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}
