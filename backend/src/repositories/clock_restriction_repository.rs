use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{ClockOverrideStatus, ClockRestrictionMode};
use crate::error::AppError;
use crate::models::{
    ClockOverrideFilter, ClockOverrideRequest, ClockOverrideRequestUpdate, ClockRestriction,
    ClockRestrictionFilter, ClockRestrictionUpdate, EffectiveRestriction, NewClockOverrideRequest,
    NewClockRestriction, Pagination,
};
use crate::schema::{clock_override_requests, clock_restrictions, team_members};

/// Repository for clock restrictions and override requests
pub struct ClockRestrictionRepository {
    pool: DbPool,
}

impl ClockRestrictionRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Get reference to the pool
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    // =====================
    // Clock Restrictions CRUD
    // =====================

    /// Create a new clock restriction
    pub async fn create_restriction(
        &self,
        restriction: NewClockRestriction,
    ) -> Result<ClockRestriction, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(clock_restrictions::table)
            .values(&restriction)
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict(
                    "A restriction already exists for this scope (org/team/user)".to_string(),
                ),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Get restriction by ID
    pub async fn find_restriction_by_id(
        &self,
        org_id: Uuid,
        restriction_id: Uuid,
    ) -> Result<ClockRestriction, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        clock_restrictions::table
            .filter(clock_restrictions::organization_id.eq(org_id))
            .find(restriction_id)
            .first::<ClockRestriction>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Clock restriction not found".to_string()))
    }

    /// List restrictions for organization with filters
    pub async fn list_restrictions(
        &self,
        org_id: Uuid,
        filter: &ClockRestrictionFilter,
    ) -> Result<Vec<ClockRestriction>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let mut query = clock_restrictions::table
            .filter(clock_restrictions::organization_id.eq(org_id))
            .into_boxed();

        if let Some(team_id) = filter.team_id {
            query = query.filter(clock_restrictions::team_id.eq(team_id));
        }
        if let Some(user_id) = filter.user_id {
            query = query.filter(clock_restrictions::user_id.eq(user_id));
        }
        if let Some(mode) = filter.mode {
            query = query.filter(clock_restrictions::mode.eq(mode));
        }
        if let Some(is_active) = filter.is_active {
            query = query.filter(clock_restrictions::is_active.eq(is_active));
        }

        query
            .order(clock_restrictions::created_at.desc())
            .load::<ClockRestriction>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Update a clock restriction
    pub async fn update_restriction(
        &self,
        org_id: Uuid,
        restriction_id: Uuid,
        mut update: ClockRestrictionUpdate,
    ) -> Result<ClockRestriction, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        update.updated_at = Some(Utc::now());

        diesel::update(
            clock_restrictions::table
                .filter(clock_restrictions::organization_id.eq(org_id))
                .filter(clock_restrictions::id.eq(restriction_id)),
        )
        .set(&update)
        .get_result(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Clock restriction not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Delete a clock restriction
    pub async fn delete_restriction(
        &self,
        org_id: Uuid,
        restriction_id: Uuid,
    ) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            clock_restrictions::table
                .filter(clock_restrictions::organization_id.eq(org_id))
                .filter(clock_restrictions::id.eq(restriction_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound(
                "Clock restriction not found".to_string(),
            ));
        }

        Ok(())
    }

    // =====================
    // Cascade Logic
    // =====================

    /// Get the effective restriction for a user (cascade: User > Team > Org)
    /// Returns None if no restrictions are configured (defaults to unrestricted)
    pub async fn get_effective_restriction(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<EffectiveRestriction>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // 1. Check for user-level restriction
        let user_restriction = clock_restrictions::table
            .filter(clock_restrictions::organization_id.eq(org_id))
            .filter(clock_restrictions::user_id.eq(user_id))
            .filter(clock_restrictions::is_active.eq(true))
            .first::<ClockRestriction>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)?;

        if let Some(restriction) = user_restriction {
            return Ok(Some(EffectiveRestriction {
                restriction,
                source_level: "user".to_string(),
            }));
        }

        // 2. Get user's teams and check for team-level restrictions
        let team_ids: Vec<Uuid> = team_members::table
            .filter(team_members::user_id.eq(user_id))
            .select(team_members::team_id)
            .load(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        if !team_ids.is_empty() {
            // Get most specific team restriction (if user is in multiple teams, take the first active one)
            let team_restriction = clock_restrictions::table
                .filter(clock_restrictions::organization_id.eq(org_id))
                .filter(clock_restrictions::team_id.eq_any(&team_ids))
                .filter(clock_restrictions::user_id.is_null())
                .filter(clock_restrictions::is_active.eq(true))
                .first::<ClockRestriction>(&mut conn)
                .await
                .optional()
                .map_err(AppError::DatabaseError)?;

            if let Some(restriction) = team_restriction {
                return Ok(Some(EffectiveRestriction {
                    restriction,
                    source_level: "team".to_string(),
                }));
            }
        }

        // 3. Check for organization-level restriction
        let org_restriction = clock_restrictions::table
            .filter(clock_restrictions::organization_id.eq(org_id))
            .filter(clock_restrictions::team_id.is_null())
            .filter(clock_restrictions::user_id.is_null())
            .filter(clock_restrictions::is_active.eq(true))
            .first::<ClockRestriction>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)?;

        if let Some(restriction) = org_restriction {
            return Ok(Some(EffectiveRestriction {
                restriction,
                source_level: "organization".to_string(),
            }));
        }

        // No restriction configured - user can clock freely
        Ok(None)
    }

    // =====================
    // Override Requests CRUD
    // =====================

    /// Create a new override request
    pub async fn create_override_request(
        &self,
        request: NewClockOverrideRequest,
    ) -> Result<ClockOverrideRequest, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(clock_override_requests::table)
            .values(&request)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Get override request by ID
    pub async fn find_override_request_by_id(
        &self,
        org_id: Uuid,
        request_id: Uuid,
    ) -> Result<ClockOverrideRequest, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .find(request_id)
            .first::<ClockOverrideRequest>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Override request not found".to_string()))
    }

    /// List override requests with filters and pagination
    pub async fn list_override_requests(
        &self,
        org_id: Uuid,
        filter: &ClockOverrideFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<ClockOverrideRequest>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Count query
        let total: i64 = {
            let mut count_query = clock_override_requests::table
                .filter(clock_override_requests::organization_id.eq(org_id))
                .into_boxed();

            if let Some(user_id) = filter.user_id {
                count_query = count_query.filter(clock_override_requests::user_id.eq(user_id));
            }
            if let Some(status) = filter.status {
                count_query = count_query.filter(clock_override_requests::status.eq(status));
            }
            if let Some(ref action) = filter.requested_action {
                count_query =
                    count_query.filter(clock_override_requests::requested_action.eq(action));
            }

            count_query
                .count()
                .get_result(&mut conn)
                .await
                .map_err(AppError::DatabaseError)?
        };

        // Data query
        let mut query = clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .into_boxed();

        if let Some(user_id) = filter.user_id {
            query = query.filter(clock_override_requests::user_id.eq(user_id));
        }
        if let Some(status) = filter.status {
            query = query.filter(clock_override_requests::status.eq(status));
        }
        if let Some(ref action) = filter.requested_action {
            query = query.filter(clock_override_requests::requested_action.eq(action));
        }

        let offset = (pagination.page - 1) * pagination.per_page;
        let requests = query
            .order(clock_override_requests::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<ClockOverrideRequest>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((requests, total))
    }

    /// List pending override requests for organization
    pub async fn list_pending_override_requests(
        &self,
        org_id: Uuid,
        pagination: &Pagination,
    ) -> Result<(Vec<ClockOverrideRequest>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let total: i64 = clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .filter(clock_override_requests::status.eq(ClockOverrideStatus::Pending))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        let offset = (pagination.page - 1) * pagination.per_page;
        let requests = clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .filter(clock_override_requests::status.eq(ClockOverrideStatus::Pending))
            .order(clock_override_requests::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<ClockOverrideRequest>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((requests, total))
    }

    /// Update an override request (for review)
    pub async fn update_override_request(
        &self,
        org_id: Uuid,
        request_id: Uuid,
        update: ClockOverrideRequestUpdate,
    ) -> Result<ClockOverrideRequest, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::update(
            clock_override_requests::table
                .filter(clock_override_requests::organization_id.eq(org_id))
                .filter(clock_override_requests::id.eq(request_id)),
        )
        .set(&update)
        .get_result(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Override request not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Approve an override request
    pub async fn approve_override_request(
        &self,
        org_id: Uuid,
        request_id: Uuid,
        reviewer_id: Uuid,
        review_notes: Option<String>,
        clock_entry_id: Option<Uuid>,
    ) -> Result<ClockOverrideRequest, AppError> {
        let update = ClockOverrideRequestUpdate {
            clock_entry_id: Some(clock_entry_id),
            status: Some(ClockOverrideStatus::Approved),
            reviewed_by: Some(Some(reviewer_id)),
            reviewed_at: Some(Some(Utc::now())),
            review_notes: Some(review_notes),
        };

        self.update_override_request(org_id, request_id, update)
            .await
    }

    /// Reject an override request
    pub async fn reject_override_request(
        &self,
        org_id: Uuid,
        request_id: Uuid,
        reviewer_id: Uuid,
        review_notes: Option<String>,
    ) -> Result<ClockOverrideRequest, AppError> {
        let update = ClockOverrideRequestUpdate {
            clock_entry_id: None,
            status: Some(ClockOverrideStatus::Rejected),
            reviewed_by: Some(Some(reviewer_id)),
            reviewed_at: Some(Some(Utc::now())),
            review_notes: Some(review_notes),
        };

        self.update_override_request(org_id, request_id, update)
            .await
    }

    /// Auto-approve an override request (for flexible mode without manager approval)
    pub async fn auto_approve_override_request(
        &self,
        org_id: Uuid,
        request_id: Uuid,
        clock_entry_id: Uuid,
    ) -> Result<ClockOverrideRequest, AppError> {
        let update = ClockOverrideRequestUpdate {
            clock_entry_id: Some(Some(clock_entry_id)),
            status: Some(ClockOverrideStatus::AutoApproved),
            reviewed_by: None,
            reviewed_at: Some(Some(Utc::now())),
            review_notes: Some(Some("Auto-approved based on restriction policy".to_string())),
        };

        self.update_override_request(org_id, request_id, update)
            .await
    }

    /// Get user's pending override request for a specific action
    pub async fn find_pending_override_for_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        action: &str,
    ) -> Result<Option<ClockOverrideRequest>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .filter(clock_override_requests::user_id.eq(user_id))
            .filter(clock_override_requests::requested_action.eq(action))
            .filter(clock_override_requests::status.eq(ClockOverrideStatus::Pending))
            .first::<ClockOverrideRequest>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)
    }

    /// Count pending override requests for user
    pub async fn count_pending_overrides_for_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<i64, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .filter(clock_override_requests::user_id.eq(user_id))
            .filter(clock_override_requests::status.eq(ClockOverrideStatus::Pending))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Find a recent approved/auto-approved override for user and action
    /// An override is considered valid if it was approved today and not yet used (clock_entry_id is null)
    pub async fn find_valid_approved_override(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        action: &str,
    ) -> Result<Option<ClockOverrideRequest>, AppError> {
        use chrono::{Duration, Utc};

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Consider overrides approved in the last 24 hours as valid
        let cutoff = Utc::now() - Duration::hours(24);

        clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .filter(clock_override_requests::user_id.eq(user_id))
            .filter(clock_override_requests::requested_action.eq(action))
            .filter(
                clock_override_requests::status
                    .eq(ClockOverrideStatus::Approved)
                    .or(clock_override_requests::status.eq(ClockOverrideStatus::AutoApproved)),
            )
            .filter(clock_override_requests::clock_entry_id.is_null()) // Not yet used
            .filter(clock_override_requests::created_at.gt(cutoff))
            .order(clock_override_requests::created_at.desc())
            .first::<ClockOverrideRequest>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)
    }

    /// Mark an override as used by linking it to a clock entry
    pub async fn mark_override_as_used(
        &self,
        org_id: Uuid,
        override_id: Uuid,
        clock_entry_id: Uuid,
    ) -> Result<ClockOverrideRequest, AppError> {
        let update = ClockOverrideRequestUpdate {
            clock_entry_id: Some(Some(clock_entry_id)),
            status: None,
            reviewed_by: None,
            reviewed_at: None,
            review_notes: None,
        };

        self.update_override_request(org_id, override_id, update)
            .await
    }

    /// Batch fetch override requests by clock entry IDs
    /// Returns a map of clock_entry_id -> ClockOverrideRequest
    pub async fn find_overrides_by_clock_entry_ids(
        &self,
        org_id: Uuid,
        clock_entry_ids: &[Uuid],
    ) -> Result<std::collections::HashMap<Uuid, ClockOverrideRequest>, AppError> {
        use std::collections::HashMap;

        if clock_entry_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let overrides: Vec<ClockOverrideRequest> = clock_override_requests::table
            .filter(clock_override_requests::organization_id.eq(org_id))
            .filter(clock_override_requests::clock_entry_id.eq_any(clock_entry_ids))
            .load::<ClockOverrideRequest>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        // Build map from clock_entry_id to override
        let mut map = HashMap::new();
        for override_req in overrides {
            if let Some(entry_id) = override_req.clock_entry_id {
                map.insert(entry_id, override_req);
            }
        }

        Ok(map)
    }

    /// Get the default restriction (to be used when no restriction is configured)
    pub fn get_default_restriction(&self, org_id: Uuid) -> ClockRestriction {
        ClockRestriction {
            id: Uuid::nil(),
            organization_id: org_id,
            team_id: None,
            user_id: None,
            mode: ClockRestrictionMode::Unrestricted,
            clock_in_earliest: None,
            clock_in_latest: None,
            clock_out_earliest: None,
            clock_out_latest: None,
            enforce_schedule: false,
            require_manager_approval: false,
            is_active: true,
            max_daily_clock_events: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
