use chrono::{DateTime, NaiveTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::BreakTrackingMode;
use crate::error::AppError;
use crate::models::{
    BreakEntry, BreakEntryFilter, BreakEntryUpdate, BreakPolicy, BreakPolicyFilter,
    BreakPolicyUpdate, BreakWindow, NewBreakEntry, NewBreakPolicy, NewBreakWindow, Pagination,
};
use crate::schema::{break_entries, break_policies, break_windows, team_members};
use crate::utils::{end_of_day, start_of_day};

/// Repository for break policies, windows, and entries
pub struct BreakRepository {
    pool: DbPool,
}

impl BreakRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Get reference to the pool
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    // =====================
    // Break Policies CRUD
    // =====================

    /// Create a new break policy
    pub async fn create_policy(&self, policy: NewBreakPolicy) -> Result<BreakPolicy, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(break_policies::table)
            .values(&policy)
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict(
                    "A break policy already exists for this scope (org/team/user)".to_string(),
                ),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Get policy by ID
    pub async fn find_policy_by_id(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
    ) -> Result<BreakPolicy, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_policies::table
            .filter(break_policies::organization_id.eq(org_id))
            .find(policy_id)
            .first::<BreakPolicy>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Break policy not found".to_string()))
    }

    /// List policies for organization with filters and pagination
    pub async fn list_policies(
        &self,
        org_id: Uuid,
        filter: &BreakPolicyFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<BreakPolicy>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Count query
        let total: i64 = {
            let mut count_query = break_policies::table
                .filter(break_policies::organization_id.eq(org_id))
                .into_boxed();

            if let Some(team_id) = filter.team_id {
                count_query = count_query.filter(break_policies::team_id.eq(team_id));
            }
            if let Some(user_id) = filter.user_id {
                count_query = count_query.filter(break_policies::user_id.eq(user_id));
            }
            if let Some(tracking_mode) = filter.tracking_mode {
                count_query = count_query.filter(break_policies::tracking_mode.eq(tracking_mode));
            }
            if let Some(is_active) = filter.is_active {
                count_query = count_query.filter(break_policies::is_active.eq(is_active));
            }

            count_query
                .count()
                .get_result(&mut conn)
                .await
                .map_err(AppError::DatabaseError)?
        };

        // Data query
        let mut query = break_policies::table
            .filter(break_policies::organization_id.eq(org_id))
            .into_boxed();

        if let Some(team_id) = filter.team_id {
            query = query.filter(break_policies::team_id.eq(team_id));
        }
        if let Some(user_id) = filter.user_id {
            query = query.filter(break_policies::user_id.eq(user_id));
        }
        if let Some(tracking_mode) = filter.tracking_mode {
            query = query.filter(break_policies::tracking_mode.eq(tracking_mode));
        }
        if let Some(is_active) = filter.is_active {
            query = query.filter(break_policies::is_active.eq(is_active));
        }

        let offset = (pagination.page - 1) * pagination.per_page;
        let policies = query
            .order(break_policies::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<BreakPolicy>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((policies, total))
    }

    /// Update a break policy
    pub async fn update_policy(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
        update: BreakPolicyUpdate,
    ) -> Result<BreakPolicy, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::update(
            break_policies::table
                .filter(break_policies::organization_id.eq(org_id))
                .filter(break_policies::id.eq(policy_id)),
        )
        .set(&update)
        .get_result(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Break policy not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Delete a break policy (cascades to windows)
    pub async fn delete_policy(&self, org_id: Uuid, policy_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            break_policies::table
                .filter(break_policies::organization_id.eq(org_id))
                .filter(break_policies::id.eq(policy_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Break policy not found".to_string()));
        }

        Ok(())
    }

    // =====================
    // Cascade Logic
    // =====================

    /// Get the effective break policy for a user (cascade: User > Team > Org)
    /// Returns None if no policies are configured
    pub async fn get_effective_policy(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<(BreakPolicy, String)>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // 1. Check for user-level policy
        let user_policy = break_policies::table
            .filter(break_policies::organization_id.eq(org_id))
            .filter(break_policies::user_id.eq(user_id))
            .filter(break_policies::is_active.eq(true))
            .first::<BreakPolicy>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)?;

        if let Some(policy) = user_policy {
            return Ok(Some((policy, "user".to_string())));
        }

        // 2. Get user's teams and check for team-level policies
        let team_ids: Vec<Uuid> = team_members::table
            .filter(team_members::user_id.eq(user_id))
            .select(team_members::team_id)
            .load(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        if !team_ids.is_empty() {
            let team_policy = break_policies::table
                .filter(break_policies::organization_id.eq(org_id))
                .filter(break_policies::team_id.eq_any(&team_ids))
                .filter(break_policies::user_id.is_null())
                .filter(break_policies::is_active.eq(true))
                .first::<BreakPolicy>(&mut conn)
                .await
                .optional()
                .map_err(AppError::DatabaseError)?;

            if let Some(policy) = team_policy {
                return Ok(Some((policy, "team".to_string())));
            }
        }

        // 3. Check for organization-level policy
        let org_policy = break_policies::table
            .filter(break_policies::organization_id.eq(org_id))
            .filter(break_policies::team_id.is_null())
            .filter(break_policies::user_id.is_null())
            .filter(break_policies::is_active.eq(true))
            .first::<BreakPolicy>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)?;

        if let Some(policy) = org_policy {
            return Ok(Some((policy, "organization".to_string())));
        }

        // No policy configured
        Ok(None)
    }

    // =====================
    // Break Windows CRUD
    // =====================

    /// Create a new break window
    pub async fn create_window(&self, window: NewBreakWindow) -> Result<BreakWindow, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(break_windows::table)
            .values(&window)
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict(
                    "A break window already exists for this day of week".to_string(),
                ),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Get windows for a policy
    pub async fn get_windows_for_policy(
        &self,
        policy_id: Uuid,
    ) -> Result<Vec<BreakWindow>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_windows::table
            .filter(break_windows::break_policy_id.eq(policy_id))
            .order(break_windows::day_of_week.asc())
            .load::<BreakWindow>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Get window by ID
    pub async fn find_window_by_id(&self, window_id: Uuid) -> Result<BreakWindow, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_windows::table
            .find(window_id)
            .first::<BreakWindow>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Break window not found".to_string()))
    }

    /// Delete a break window
    pub async fn delete_window(&self, window_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(break_windows::table.find(window_id))
            .execute(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Break window not found".to_string()));
        }

        Ok(())
    }

    /// Delete all windows for a policy
    pub async fn delete_windows_for_policy(&self, policy_id: Uuid) -> Result<usize, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::delete(break_windows::table.filter(break_windows::break_policy_id.eq(policy_id)))
            .execute(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Get window for a specific day of week for a policy
    pub async fn get_window_for_day(
        &self,
        policy_id: Uuid,
        day_of_week: i16,
    ) -> Result<Option<BreakWindow>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_windows::table
            .filter(break_windows::break_policy_id.eq(policy_id))
            .filter(break_windows::day_of_week.eq(day_of_week))
            .first::<BreakWindow>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)
    }

    // =====================
    // Break Entries CRUD
    // =====================

    /// Create a new break entry (start a break)
    pub async fn create_entry(&self, entry: NewBreakEntry) -> Result<BreakEntry, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(break_entries::table)
            .values(&entry)
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Get entry by ID
    pub async fn find_entry_by_id(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
    ) -> Result<BreakEntry, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_entries::table
            .filter(break_entries::organization_id.eq(org_id))
            .find(entry_id)
            .first::<BreakEntry>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Break entry not found".to_string()))
    }

    /// List break entries with filters and pagination
    pub async fn list_entries(
        &self,
        org_id: Uuid,
        filter: &BreakEntryFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<BreakEntry>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        // Count query
        let total: i64 = {
            let mut count_query = break_entries::table
                .filter(break_entries::organization_id.eq(org_id))
                .into_boxed();

            if let Some(user_id) = filter.user_id {
                count_query = count_query.filter(break_entries::user_id.eq(user_id));
            }
            if let Some(clock_entry_id) = filter.clock_entry_id {
                count_query = count_query.filter(break_entries::clock_entry_id.eq(clock_entry_id));
            }
            if let Some(ref start_date) = filter.start_date {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
                    count_query = count_query.filter(break_entries::break_start.ge(start_of_day(date)));
                }
            }
            if let Some(ref end_date) = filter.end_date {
                if let Ok(date) = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
                    count_query = count_query.filter(break_entries::break_start.le(end_of_day(date)));
                }
            }

            count_query
                .count()
                .get_result(&mut conn)
                .await
                .map_err(AppError::DatabaseError)?
        };

        // Data query
        let mut query = break_entries::table
            .filter(break_entries::organization_id.eq(org_id))
            .into_boxed();

        if let Some(user_id) = filter.user_id {
            query = query.filter(break_entries::user_id.eq(user_id));
        }
        if let Some(clock_entry_id) = filter.clock_entry_id {
            query = query.filter(break_entries::clock_entry_id.eq(clock_entry_id));
        }
        if let Some(ref start_date) = filter.start_date {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d") {
                query = query.filter(break_entries::break_start.ge(start_of_day(date)));
            }
        }
        if let Some(ref end_date) = filter.end_date {
            if let Ok(date) = chrono::NaiveDate::parse_from_str(end_date, "%Y-%m-%d") {
                query = query.filter(break_entries::break_start.le(end_of_day(date)));
            }
        }

        let offset = (pagination.page - 1) * pagination.per_page;
        let entries = query
            .order(break_entries::break_start.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<BreakEntry>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((entries, total))
    }

    /// Update a break entry (end a break)
    pub async fn update_entry(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
        update: BreakEntryUpdate,
    ) -> Result<BreakEntry, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::update(
            break_entries::table
                .filter(break_entries::organization_id.eq(org_id))
                .filter(break_entries::id.eq(entry_id)),
        )
        .set(&update)
        .get_result(&mut conn)
        .await
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Break entry not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Get current active break for user (break_end is NULL)
    pub async fn get_active_break_for_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<BreakEntry>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_entries::table
            .filter(break_entries::organization_id.eq(org_id))
            .filter(break_entries::user_id.eq(user_id))
            .filter(break_entries::break_end.is_null())
            .first::<BreakEntry>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)
    }

    /// Get all break entries for a clock entry
    pub async fn get_entries_for_clock_entry(
        &self,
        clock_entry_id: Uuid,
    ) -> Result<Vec<BreakEntry>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_entries::table
            .filter(break_entries::clock_entry_id.eq(clock_entry_id))
            .order(break_entries::break_start.asc())
            .load::<BreakEntry>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    /// Calculate total break minutes for a clock entry
    pub async fn calculate_total_break_minutes(
        &self,
        clock_entry_id: Uuid,
    ) -> Result<i32, AppError> {
        let entries = self.get_entries_for_clock_entry(clock_entry_id).await?;

        let total = entries.iter().filter_map(|e| e.duration_minutes).sum();

        Ok(total)
    }

    /// Get break entries for a user within a date range
    pub async fn get_entries_for_user_in_range(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<BreakEntry>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        break_entries::table
            .filter(break_entries::organization_id.eq(org_id))
            .filter(break_entries::user_id.eq(user_id))
            .filter(break_entries::break_start.ge(start))
            .filter(break_entries::break_start.le(end))
            .order(break_entries::break_start.asc())
            .load::<BreakEntry>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)
    }

    // =====================
    // Helpers
    // =====================

    /// Parse time string to NaiveTime
    pub fn parse_time(time_str: &str) -> Result<NaiveTime, AppError> {
        NaiveTime::parse_from_str(time_str, "%H:%M")
            .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M:%S"))
            .map_err(|_| AppError::ValidationError(format!("Invalid time format: {}", time_str)))
    }

    /// Get the default break policy for auto_deduct mode
    pub fn get_default_policy(&self, org_id: Uuid) -> BreakPolicy {
        BreakPolicy {
            id: Uuid::nil(),
            organization_id: org_id,
            team_id: None,
            user_id: None,
            name: "Default (No Policy)".to_string(),
            description: None,
            tracking_mode: BreakTrackingMode::AutoDeduct,
            notify_missing_break: false,
            is_active: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
