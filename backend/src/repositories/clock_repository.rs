use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::domain::enums::ClockEntryStatus;
use crate::error::AppError;
use crate::models::{ClockEntry, ClockEntryUpdate, ClockFilter, NewClockEntry, Pagination};
use crate::schema::{clock_entries, team_members, users};

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Clock repository for database operations
pub struct ClockRepository {
    pool: DbPool,
}

impl ClockRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Get reference to the pool
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    /// Create a new clock entry (clock in)
    pub async fn clock_in(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        notes: Option<String>,
    ) -> Result<ClockEntry, AppError> {
        let mut conn = self.pool.get()?;

        let new_entry = NewClockEntry {
            organization_id: org_id,
            user_id,
            clock_in: Utc::now(),
            notes,
        };

        diesel::insert_into(clock_entries::table)
            .values(&new_entry)
            .get_result(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Clock out an entry
    pub async fn clock_out(&self, org_id: Uuid, entry_id: Uuid) -> Result<ClockEntry, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(
            clock_entries::table
                .filter(clock_entries::organization_id.eq(org_id))
                .filter(clock_entries::id.eq(entry_id)),
        )
        .set((
            clock_entries::clock_out.eq(Some(Utc::now())),
            clock_entries::updated_at.eq(Utc::now()),
        ))
        .get_result(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Clock entry not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Find open clock entry for user
    pub async fn find_open_entry(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<ClockEntry>, AppError> {
        let mut conn = self.pool.get()?;

        let entry = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::user_id.eq(user_id))
            .filter(clock_entries::clock_out.is_null())
            .first::<ClockEntry>(&mut conn)
            .optional()?;

        Ok(entry)
    }

    /// Find clock entry by ID
    pub async fn find_by_id(&self, org_id: Uuid, entry_id: Uuid) -> Result<ClockEntry, AppError> {
        let mut conn = self.pool.get()?;

        clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .find(entry_id)
            .first::<ClockEntry>(&mut conn)
            .map_err(|_| AppError::NotFound("Clock entry not found".to_string()))
    }

    /// List clock entries for a user
    pub async fn list_by_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        filter: &ClockFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<ClockEntry>, i64), AppError> {
        let mut conn = self.pool.get()?;

        // Get total count
        let total: i64 = {
            let mut count_query = clock_entries::table
                .filter(clock_entries::organization_id.eq(org_id))
                .filter(clock_entries::user_id.eq(user_id))
                .into_boxed();

            if let Some(start) = filter.start_date {
                count_query = count_query.filter(clock_entries::clock_in.ge(start));
            }
            if let Some(end) = filter.end_date {
                count_query = count_query.filter(clock_entries::clock_in.le(end));
            }
            if let Some(status) = filter.status {
                count_query = count_query.filter(clock_entries::status.eq(status));
            }

            count_query.count().get_result(&mut conn)?
        };

        // Build data query
        let mut query = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::user_id.eq(user_id))
            .into_boxed();

        if let Some(start) = filter.start_date {
            query = query.filter(clock_entries::clock_in.ge(start));
        }
        if let Some(end) = filter.end_date {
            query = query.filter(clock_entries::clock_in.le(end));
        }
        if let Some(status) = filter.status {
            query = query.filter(clock_entries::status.eq(status));
        }

        let offset = (pagination.page - 1) * pagination.per_page;
        let entries = query
            .order(clock_entries::clock_in.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<ClockEntry>(&mut conn)?;

        Ok((entries, total))
    }

    /// List clock entries for a team
    pub async fn list_by_team(
        &self,
        org_id: Uuid,
        team_id: Uuid,
        filter: &ClockFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<ClockEntry>, i64), AppError> {
        let mut conn = self.pool.get()?;

        // Get team member user IDs
        let member_ids: Vec<Uuid> = team_members::table
            .filter(team_members::team_id.eq(team_id))
            .select(team_members::user_id)
            .load(&mut conn)?;

        if member_ids.is_empty() {
            return Ok((vec![], 0));
        }

        // Get total count
        let total: i64 = {
            let mut count_query = clock_entries::table
                .filter(clock_entries::organization_id.eq(org_id))
                .filter(clock_entries::user_id.eq_any(&member_ids))
                .into_boxed();

            if let Some(start) = filter.start_date {
                count_query = count_query.filter(clock_entries::clock_in.ge(start));
            }
            if let Some(end) = filter.end_date {
                count_query = count_query.filter(clock_entries::clock_in.le(end));
            }
            if let Some(status) = filter.status {
                count_query = count_query.filter(clock_entries::status.eq(status));
            }

            count_query.count().get_result(&mut conn)?
        };

        // Build data query
        let mut query = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::user_id.eq_any(&member_ids))
            .into_boxed();

        if let Some(start) = filter.start_date {
            query = query.filter(clock_entries::clock_in.ge(start));
        }
        if let Some(end) = filter.end_date {
            query = query.filter(clock_entries::clock_in.le(end));
        }
        if let Some(status) = filter.status {
            query = query.filter(clock_entries::status.eq(status));
        }

        let offset = (pagination.page - 1) * pagination.per_page;
        let entries = query
            .order(clock_entries::clock_in.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<ClockEntry>(&mut conn)?;

        Ok((entries, total))
    }

    /// List pending clock entries for organization
    pub async fn list_pending(
        &self,
        org_id: Uuid,
        pagination: &Pagination,
    ) -> Result<(Vec<ClockEntry>, i64), AppError> {
        let mut conn = self.pool.get()?;

        let total: i64 = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::status.eq(ClockEntryStatus::Pending))
            .filter(clock_entries::clock_out.is_not_null())
            .count()
            .get_result(&mut conn)?;

        let offset = (pagination.page - 1) * pagination.per_page;
        let entries = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::status.eq(ClockEntryStatus::Pending))
            .filter(clock_entries::clock_out.is_not_null())
            .order(clock_entries::clock_in.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<ClockEntry>(&mut conn)?;

        Ok((entries, total))
    }

    /// Approve a clock entry
    pub async fn approve(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
        approver_id: Uuid,
    ) -> Result<ClockEntry, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(
            clock_entries::table
                .filter(clock_entries::organization_id.eq(org_id))
                .filter(clock_entries::id.eq(entry_id)),
        )
        .set((
            clock_entries::status.eq(ClockEntryStatus::Approved),
            clock_entries::approved_by.eq(Some(approver_id)),
            clock_entries::approved_at.eq(Some(Utc::now())),
            clock_entries::updated_at.eq(Utc::now()),
        ))
        .get_result(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Clock entry not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Reject a clock entry
    pub async fn reject(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
        approver_id: Uuid,
        reason: Option<String>,
    ) -> Result<ClockEntry, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(
            clock_entries::table
                .filter(clock_entries::organization_id.eq(org_id))
                .filter(clock_entries::id.eq(entry_id)),
        )
        .set((
            clock_entries::status.eq(ClockEntryStatus::Rejected),
            clock_entries::approved_by.eq(Some(approver_id)),
            clock_entries::approved_at.eq(Some(Utc::now())),
            clock_entries::notes.eq(reason),
            clock_entries::updated_at.eq(Utc::now()),
        ))
        .get_result(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Clock entry not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Get entries for a period (for KPI calculations)
    pub async fn get_entries_for_period(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<ClockEntry>, AppError> {
        let mut conn = self.pool.get()?;

        let entries = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::user_id.eq(user_id))
            .filter(clock_entries::clock_in.ge(start))
            .filter(clock_entries::clock_in.le(end))
            .filter(clock_entries::clock_out.is_not_null())
            .order(clock_entries::clock_in.asc())
            .load::<ClockEntry>(&mut conn)?;

        Ok(entries)
    }

    /// Get all currently clocked in users for organization
    pub async fn get_currently_clocked_in(&self, org_id: Uuid) -> Result<Vec<ClockEntry>, AppError> {
        let mut conn = self.pool.get()?;

        let entries = clock_entries::table
            .filter(clock_entries::organization_id.eq(org_id))
            .filter(clock_entries::clock_out.is_null())
            .load::<ClockEntry>(&mut conn)?;

        Ok(entries)
    }

    /// Update a clock entry
    pub async fn update(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
        update: ClockEntryUpdate,
    ) -> Result<ClockEntry, AppError> {
        let mut conn = self.pool.get()?;

        diesel::update(
            clock_entries::table
                .filter(clock_entries::organization_id.eq(org_id))
                .filter(clock_entries::id.eq(entry_id)),
        )
        .set(&update)
        .get_result(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => {
                AppError::NotFound("Clock entry not found".to_string())
            }
            _ => AppError::DatabaseError(e),
        })
    }

    /// Get user info for clock entries
    pub async fn get_user_info(
        &self,
        user_id: Uuid,
    ) -> Result<(String, String), AppError> {
        let mut conn = self.pool.get()?;

        let (first_name, last_name, email): (String, String, String) = users::table
            .filter(users::id.eq(user_id))
            .select((users::first_name, users::last_name, users::email))
            .first(&mut conn)?;

        Ok((format!("{} {}", first_name, last_name), email))
    }
}
