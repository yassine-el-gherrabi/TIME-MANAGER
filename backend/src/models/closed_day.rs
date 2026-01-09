use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::closed_days;

/// ClosedDay entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = closed_days)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClosedDay {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}

/// NewClosedDay for creating closed days
#[derive(Debug, Insertable)]
#[diesel(table_name = closed_days)]
pub struct NewClosedDay {
    pub organization_id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
}

/// ClosedDay update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = closed_days)]
pub struct ClosedDayUpdate {
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}

/// ClosedDay response for API
#[derive(Debug, Serialize)]
pub struct ClosedDayResponse {
    pub id: Uuid,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub created_at: DateTime<Utc>,
}

impl From<ClosedDay> for ClosedDayResponse {
    fn from(cd: ClosedDay) -> Self {
        Self {
            id: cd.id,
            name: cd.name,
            date: cd.date,
            is_recurring: cd.is_recurring,
            created_at: cd.created_at,
        }
    }
}

/// ClosedDay filter options
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ClosedDayFilter {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}
