use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::ClockEntryStatus;
use crate::schema::clock_entries;

/// ClockEntry entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = clock_entries)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClockEntry {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub clock_in: DateTime<Utc>,
    pub clock_out: Option<DateTime<Utc>>,
    pub status: ClockEntryStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// NewClockEntry for creating clock entries (clock in)
#[derive(Debug, Insertable)]
#[diesel(table_name = clock_entries)]
pub struct NewClockEntry {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub clock_in: DateTime<Utc>,
    pub notes: Option<String>,
}

/// ClockEntry update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = clock_entries)]
pub struct ClockEntryUpdate {
    pub clock_out: Option<Option<DateTime<Utc>>>,
    pub status: Option<ClockEntryStatus>,
    pub approved_by: Option<Option<Uuid>>,
    pub approved_at: Option<Option<DateTime<Utc>>>,
    pub notes: Option<Option<String>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// ClockEntry response with user info
#[derive(Debug, Serialize)]
pub struct ClockEntryResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub clock_in: DateTime<Utc>,
    pub clock_out: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i64>,
    pub status: ClockEntryStatus,
    pub approved_by: Option<Uuid>,
    pub approver_name: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl ClockEntryResponse {
    pub fn from_entry(
        entry: &ClockEntry,
        user_name: String,
        user_email: String,
        approver_name: Option<String>,
    ) -> Self {
        let duration_minutes = entry
            .clock_out
            .map(|out| (out - entry.clock_in).num_minutes());

        Self {
            id: entry.id,
            user_id: entry.user_id,
            user_name,
            user_email,
            clock_in: entry.clock_in,
            clock_out: entry.clock_out,
            duration_minutes,
            status: entry.status,
            approved_by: entry.approved_by,
            approver_name,
            approved_at: entry.approved_at,
            notes: entry.notes.clone(),
            created_at: entry.created_at,
        }
    }
}

/// Current clock status for a user
#[derive(Debug, Serialize)]
pub struct ClockStatus {
    pub is_clocked_in: bool,
    pub current_entry: Option<ClockEntry>,
    pub elapsed_minutes: Option<i64>,
}

/// Clock filter options
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ClockFilter {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<ClockEntryStatus>,
    pub user_id: Option<Uuid>,
}

/// Paginated clock entries response
#[derive(Debug, Serialize)]
pub struct PaginatedClockEntries {
    pub data: Vec<ClockEntryResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
