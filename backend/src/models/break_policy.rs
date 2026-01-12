use chrono::{DateTime, NaiveTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::BreakTrackingMode;
use crate::schema::{break_entries, break_policies, break_windows};

// ============================================================================
// Break Policy (Cascade: User > Team > Org)
// ============================================================================

/// Break policy entity from database
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = break_policies)]
pub struct BreakPolicy {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub tracking_mode: BreakTrackingMode,
    pub notify_missing_break: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// New break policy for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = break_policies)]
pub struct NewBreakPolicy {
    pub organization_id: Uuid,
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub tracking_mode: BreakTrackingMode,
    pub notify_missing_break: bool,
    pub is_active: bool,
}

/// Break policy update payload
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = break_policies)]
pub struct BreakPolicyUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub tracking_mode: Option<BreakTrackingMode>,
    pub notify_missing_break: Option<bool>,
    pub is_active: Option<bool>,
}

/// Break policy response with context (org/team/user names)
#[derive(Debug, Serialize)]
pub struct BreakPolicyResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub team_id: Option<Uuid>,
    pub team_name: Option<String>,
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub tracking_mode: BreakTrackingMode,
    pub notify_missing_break: bool,
    pub is_active: bool,
    pub windows: Vec<BreakWindowResponse>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Effective policy result (from cascade resolution)
#[derive(Debug, Serialize)]
pub struct EffectiveBreakPolicy {
    pub policy: Option<BreakPolicyResponse>,
    pub source_level: String, // "user", "team", "organization", "default"
}

// ============================================================================
// Break Window (Break times per day of week)
// ============================================================================

/// Break window entity from database
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = break_windows)]
pub struct BreakWindow {
    pub id: Uuid,
    pub break_policy_id: Uuid,
    pub day_of_week: i16,
    pub window_start: NaiveTime,
    pub window_end: NaiveTime,
    pub min_duration_minutes: i32,
    pub max_duration_minutes: i32,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
}

/// New break window for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = break_windows)]
pub struct NewBreakWindow {
    pub break_policy_id: Uuid,
    pub day_of_week: i16,
    pub window_start: NaiveTime,
    pub window_end: NaiveTime,
    pub min_duration_minutes: i32,
    pub max_duration_minutes: i32,
    pub is_mandatory: bool,
}

/// Break window response
#[derive(Debug, Serialize, Clone)]
pub struct BreakWindowResponse {
    pub id: Uuid,
    pub day_of_week: i16,
    pub window_start: String,
    pub window_end: String,
    pub min_duration_minutes: i32,
    pub max_duration_minutes: i32,
    pub is_mandatory: bool,
}

// ============================================================================
// Break Entry (Actual break records for explicit tracking mode)
// ============================================================================

/// Break entry entity from database
#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = break_entries)]
pub struct BreakEntry {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub clock_entry_id: Uuid,
    pub break_start: DateTime<Utc>,
    pub break_end: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// New break entry for insert
#[derive(Debug, Insertable)]
#[diesel(table_name = break_entries)]
pub struct NewBreakEntry {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub clock_entry_id: Uuid,
    pub break_start: DateTime<Utc>,
    pub notes: Option<String>,
}

/// Break entry update payload
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = break_entries)]
pub struct BreakEntryUpdate {
    pub break_end: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<Option<String>>,
}

/// Break entry response with context
#[derive(Debug, Serialize)]
pub struct BreakEntryResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub clock_entry_id: Uuid,
    pub break_start: DateTime<Utc>,
    pub break_end: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// API Request/Response Types
// ============================================================================

/// Create break policy request
#[derive(Debug, Deserialize)]
pub struct CreateBreakPolicyRequest {
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub tracking_mode: BreakTrackingMode,
    #[serde(default)]
    pub notify_missing_break: bool,
    pub windows: Option<Vec<CreateBreakWindowRequest>>,
}

/// Update break policy request
#[derive(Debug, Deserialize)]
pub struct UpdateBreakPolicyRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub tracking_mode: Option<BreakTrackingMode>,
    pub notify_missing_break: Option<bool>,
    pub is_active: Option<bool>,
}

/// Create break window request
#[derive(Debug, Deserialize, Clone)]
pub struct CreateBreakWindowRequest {
    pub day_of_week: i16,
    pub window_start: String,
    pub window_end: String,
    pub min_duration_minutes: i32,
    pub max_duration_minutes: i32,
    #[serde(default = "default_true")]
    pub is_mandatory: bool,
}

fn default_true() -> bool {
    true
}

/// Start break request (for explicit tracking)
#[derive(Debug, Deserialize)]
pub struct StartBreakRequest {
    pub notes: Option<String>,
}

/// End break request (for explicit tracking)
#[derive(Debug, Deserialize)]
pub struct EndBreakRequest {
    pub notes: Option<String>,
}

/// Break policy filter parameters
#[derive(Debug, Deserialize, Default)]
pub struct BreakPolicyFilter {
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub tracking_mode: Option<BreakTrackingMode>,
    pub is_active: Option<bool>,
}

/// Break entry filter parameters
#[derive(Debug, Deserialize, Default)]
pub struct BreakEntryFilter {
    pub clock_entry_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// Paginated break policies response
#[derive(Debug, Serialize)]
pub struct PaginatedBreakPolicies {
    pub data: Vec<BreakPolicyResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Paginated break entries response
#[derive(Debug, Serialize)]
pub struct PaginatedBreakEntries {
    pub data: Vec<BreakEntryResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// Break status (for checking if user is on break)
#[derive(Debug, Serialize)]
pub struct BreakStatus {
    pub is_on_break: bool,
    pub current_break: Option<BreakEntryResponse>,
    pub elapsed_minutes: Option<i32>,
    pub policy: Option<EffectiveBreakPolicy>,
}

/// Break deduction calculation result
#[derive(Debug, Serialize)]
pub struct BreakDeduction {
    pub total_minutes: i32,
    pub source: String, // "auto_deduct" or "tracked"
    pub entries: Vec<BreakEntryResponse>,
}
