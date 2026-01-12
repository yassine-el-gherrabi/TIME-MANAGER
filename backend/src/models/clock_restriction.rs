use chrono::{DateTime, NaiveTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::{ClockOverrideStatus, ClockRestrictionMode};
use crate::schema::{clock_override_requests, clock_restrictions};

/// ClockRestriction entity from database
/// Defines when users can clock in/out with cascade logic: User > Team > Org
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = clock_restrictions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClockRestriction {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub mode: ClockRestrictionMode,
    pub clock_in_earliest: Option<NaiveTime>,
    pub clock_in_latest: Option<NaiveTime>,
    pub clock_out_earliest: Option<NaiveTime>,
    pub clock_out_latest: Option<NaiveTime>,
    pub enforce_schedule: bool,
    pub require_manager_approval: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub max_daily_clock_events: Option<i32>,
}

/// NewClockRestriction for creating clock restrictions
#[derive(Debug, Insertable)]
#[diesel(table_name = clock_restrictions)]
pub struct NewClockRestriction {
    pub organization_id: Uuid,
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub mode: ClockRestrictionMode,
    pub clock_in_earliest: Option<NaiveTime>,
    pub clock_in_latest: Option<NaiveTime>,
    pub clock_out_earliest: Option<NaiveTime>,
    pub clock_out_latest: Option<NaiveTime>,
    pub enforce_schedule: bool,
    pub require_manager_approval: bool,
    pub is_active: bool,
    pub max_daily_clock_events: Option<i32>,
}

/// ClockRestriction update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = clock_restrictions)]
pub struct ClockRestrictionUpdate {
    pub mode: Option<ClockRestrictionMode>,
    pub clock_in_earliest: Option<Option<NaiveTime>>,
    pub clock_in_latest: Option<Option<NaiveTime>>,
    pub clock_out_earliest: Option<Option<NaiveTime>>,
    pub clock_out_latest: Option<Option<NaiveTime>>,
    pub enforce_schedule: Option<bool>,
    pub require_manager_approval: Option<bool>,
    pub is_active: Option<bool>,
    pub max_daily_clock_events: Option<Option<i32>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// ClockRestriction response with additional context
#[derive(Debug, Serialize)]
pub struct ClockRestrictionResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub team_id: Option<Uuid>,
    pub team_name: Option<String>,
    pub user_id: Option<Uuid>,
    pub user_name: Option<String>,
    pub mode: ClockRestrictionMode,
    pub clock_in_earliest: Option<NaiveTime>,
    pub clock_in_latest: Option<NaiveTime>,
    pub clock_out_earliest: Option<NaiveTime>,
    pub clock_out_latest: Option<NaiveTime>,
    pub enforce_schedule: bool,
    pub require_manager_approval: bool,
    pub is_active: bool,
    pub max_daily_clock_events: Option<i32>,
    pub scope_level: String, // "organization", "team", or "user"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ClockRestrictionResponse {
    pub fn from_restriction(
        restriction: &ClockRestriction,
        organization_name: String,
        team_name: Option<String>,
        user_name: Option<String>,
    ) -> Self {
        let scope_level = if restriction.user_id.is_some() {
            "user".to_string()
        } else if restriction.team_id.is_some() {
            "team".to_string()
        } else {
            "organization".to_string()
        };

        Self {
            id: restriction.id,
            organization_id: restriction.organization_id,
            organization_name,
            team_id: restriction.team_id,
            team_name,
            user_id: restriction.user_id,
            user_name,
            mode: restriction.mode,
            clock_in_earliest: restriction.clock_in_earliest,
            clock_in_latest: restriction.clock_in_latest,
            clock_out_earliest: restriction.clock_out_earliest,
            clock_out_latest: restriction.clock_out_latest,
            enforce_schedule: restriction.enforce_schedule,
            require_manager_approval: restriction.require_manager_approval,
            is_active: restriction.is_active,
            max_daily_clock_events: restriction.max_daily_clock_events,
            scope_level,
            created_at: restriction.created_at,
            updated_at: restriction.updated_at,
        }
    }
}

/// Effective restriction for a user (resolved from cascade)
#[derive(Debug, Serialize)]
pub struct EffectiveRestriction {
    pub restriction: ClockRestriction,
    pub source_level: String, // "user", "team", "organization", or "default"
}

/// ClockOverrideRequest entity from database
/// For flexible mode: users can request override with justification
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = clock_override_requests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ClockOverrideRequest {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub clock_entry_id: Option<Uuid>,
    pub requested_action: String,
    pub requested_at: DateTime<Utc>,
    pub reason: String,
    pub status: ClockOverrideStatus,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// NewClockOverrideRequest for creating override requests
#[derive(Debug, Insertable)]
#[diesel(table_name = clock_override_requests)]
pub struct NewClockOverrideRequest {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub clock_entry_id: Option<Uuid>,
    pub requested_action: String,
    pub requested_at: DateTime<Utc>,
    pub reason: String,
    pub status: ClockOverrideStatus,
}

/// ClockOverrideRequest update struct for review
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = clock_override_requests)]
pub struct ClockOverrideRequestUpdate {
    pub clock_entry_id: Option<Option<Uuid>>,
    pub status: Option<ClockOverrideStatus>,
    pub reviewed_by: Option<Option<Uuid>>,
    pub reviewed_at: Option<Option<DateTime<Utc>>>,
    pub review_notes: Option<Option<String>>,
}

/// ClockOverrideRequest response with additional context
#[derive(Debug, Serialize)]
pub struct ClockOverrideRequestResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub clock_entry_id: Option<Uuid>,
    pub requested_action: String,
    pub requested_at: DateTime<Utc>,
    pub reason: String,
    pub status: ClockOverrideStatus,
    pub reviewed_by: Option<Uuid>,
    pub reviewer_name: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub review_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl ClockOverrideRequestResponse {
    pub fn from_request(
        request: &ClockOverrideRequest,
        user_name: String,
        user_email: String,
        reviewer_name: Option<String>,
    ) -> Self {
        Self {
            id: request.id,
            organization_id: request.organization_id,
            user_id: request.user_id,
            user_name,
            user_email,
            clock_entry_id: request.clock_entry_id,
            requested_action: request.requested_action.clone(),
            requested_at: request.requested_at,
            reason: request.reason.clone(),
            status: request.status,
            reviewed_by: request.reviewed_by,
            reviewer_name,
            reviewed_at: request.reviewed_at,
            review_notes: request.review_notes.clone(),
            created_at: request.created_at,
        }
    }
}

/// Filter for clock restrictions
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ClockRestrictionFilter {
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub mode: Option<ClockRestrictionMode>,
    pub is_active: Option<bool>,
}

/// Filter for override requests
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ClockOverrideFilter {
    pub user_id: Option<Uuid>,
    pub status: Option<ClockOverrideStatus>,
    pub requested_action: Option<String>,
}

/// Paginated clock override requests response
#[derive(Debug, Serialize)]
pub struct PaginatedClockOverrideRequests {
    pub data: Vec<ClockOverrideRequestResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// Validation result for clock action
#[derive(Debug, Serialize)]
pub struct ClockValidationResult {
    pub allowed: bool,
    pub message: Option<String>,
    pub can_request_override: bool,
    pub effective_restriction: Option<EffectiveRestriction>,
}

/// Create clock restriction request
#[derive(Debug, Deserialize)]
pub struct CreateClockRestrictionRequest {
    pub team_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub mode: ClockRestrictionMode,
    pub clock_in_earliest: Option<NaiveTime>,
    pub clock_in_latest: Option<NaiveTime>,
    pub clock_out_earliest: Option<NaiveTime>,
    pub clock_out_latest: Option<NaiveTime>,
    pub enforce_schedule: Option<bool>,
    pub require_manager_approval: Option<bool>,
    pub is_active: Option<bool>,
    pub max_daily_clock_events: Option<i32>,
}

/// Update clock restriction request
#[derive(Debug, Deserialize)]
pub struct UpdateClockRestrictionRequest {
    pub mode: Option<ClockRestrictionMode>,
    pub clock_in_earliest: Option<Option<NaiveTime>>,
    pub clock_in_latest: Option<Option<NaiveTime>>,
    pub clock_out_earliest: Option<Option<NaiveTime>>,
    pub clock_out_latest: Option<Option<NaiveTime>>,
    pub enforce_schedule: Option<bool>,
    pub require_manager_approval: Option<bool>,
    pub is_active: Option<bool>,
    pub max_daily_clock_events: Option<Option<i32>>,
}

/// Create override request
#[derive(Debug, Deserialize)]
pub struct CreateOverrideRequest {
    pub requested_action: String,
    pub reason: String,
}

/// Review override request
#[derive(Debug, Deserialize)]
pub struct ReviewOverrideRequest {
    pub approved: bool,
    pub review_notes: Option<String>,
}
