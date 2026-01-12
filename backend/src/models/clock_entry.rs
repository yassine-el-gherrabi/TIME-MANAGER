use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::{ClockEntryStatus, ClockOverrideStatus};
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
    pub organization_id: Uuid,
    pub organization_name: String,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub team_id: Option<Uuid>,
    pub team_name: Option<String>,
    pub clock_in: DateTime<Utc>,
    pub clock_out: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i64>,
    /// Expected hours for the day based on user's schedule (None if no schedule)
    pub theoretical_hours: Option<f64>,
    pub status: ClockEntryStatus,
    pub approved_by: Option<Uuid>,
    pub approver_name: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    /// Override information (if entry was made via override)
    pub override_id: Option<Uuid>,
    pub override_reason: Option<String>,
    pub override_status: Option<ClockOverrideStatus>,
}

impl ClockEntryResponse {
    pub fn from_entry(
        entry: &ClockEntry,
        organization_name: String,
        user_name: String,
        user_email: String,
        team_id: Option<Uuid>,
        team_name: Option<String>,
        approver_name: Option<String>,
        theoretical_hours: Option<f64>,
        override_info: Option<(Uuid, String, ClockOverrideStatus)>,
    ) -> Self {
        let duration_minutes = entry
            .clock_out
            .map(|out| (out - entry.clock_in).num_minutes());

        let (override_id, override_reason, override_status) = match override_info {
            Some((id, reason, status)) => (Some(id), Some(reason), Some(status)),
            None => (None, None, None),
        };

        Self {
            id: entry.id,
            organization_id: entry.organization_id,
            organization_name,
            user_id: entry.user_id,
            user_name,
            user_email,
            team_id,
            team_name,
            clock_in: entry.clock_in,
            clock_out: entry.clock_out,
            duration_minutes,
            theoretical_hours,
            status: entry.status,
            approved_by: entry.approved_by,
            approver_name,
            approved_at: entry.approved_at,
            notes: entry.notes.clone(),
            created_at: entry.created_at,
            override_id,
            override_reason,
            override_status,
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

/// Pending clock entries filter (for approval pages)
#[derive(Debug, Clone, Default)]
pub struct PendingClockFilter {
    /// Filter by organization (SuperAdmin only)
    pub organization_id: Option<Uuid>,
    /// Filter by team
    pub team_id: Option<Uuid>,
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
