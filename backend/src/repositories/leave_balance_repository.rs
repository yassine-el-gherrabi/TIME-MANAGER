use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{LeaveBalance, LeaveBalanceFilter, LeaveBalanceUpdate, NewLeaveBalance};
use crate::schema::leave_balances;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Leave balance repository for database operations
pub struct LeaveBalanceRepository {
    pool: DbPool,
}

impl LeaveBalanceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new leave balance
    pub async fn create(&self, new_balance: NewLeaveBalance) -> Result<LeaveBalance, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(leave_balances::table)
            .values(&new_balance)
            .get_result(&mut conn)
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict(
                    "Leave balance for this type and year already exists".to_string(),
                ),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Find balance by ID
    pub async fn find_by_id(
        &self,
        org_id: Uuid,
        balance_id: Uuid,
    ) -> Result<LeaveBalance, AppError> {
        let mut conn = self.pool.get()?;

        leave_balances::table
            .filter(leave_balances::organization_id.eq(org_id))
            .find(balance_id)
            .first::<LeaveBalance>(&mut conn)
            .map_err(|_| AppError::NotFound("Leave balance not found".to_string()))
    }

    /// Find balance by user, type, and year
    pub async fn find_by_user_type_year(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        absence_type_id: Uuid,
        year: i32,
    ) -> Result<Option<LeaveBalance>, AppError> {
        let mut conn = self.pool.get()?;

        let result = leave_balances::table
            .filter(leave_balances::organization_id.eq(org_id))
            .filter(leave_balances::user_id.eq(user_id))
            .filter(leave_balances::absence_type_id.eq(absence_type_id))
            .filter(leave_balances::year.eq(year))
            .first::<LeaveBalance>(&mut conn)
            .optional()?;

        Ok(result)
    }

    /// Get or create balance for user, type, and year
    pub async fn get_or_create(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        absence_type_id: Uuid,
        year: i32,
        default_initial: BigDecimal,
    ) -> Result<LeaveBalance, AppError> {
        if let Some(balance) = self.find_by_user_type_year(org_id, user_id, absence_type_id, year).await? {
            return Ok(balance);
        }

        let new_balance = NewLeaveBalance {
            organization_id: org_id,
            user_id,
            absence_type_id,
            year,
            initial_balance: default_initial,
        };

        self.create(new_balance).await
    }

    /// List balances with filters
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: &LeaveBalanceFilter,
    ) -> Result<Vec<LeaveBalance>, AppError> {
        let mut conn = self.pool.get()?;

        let mut query = leave_balances::table
            .filter(leave_balances::organization_id.eq(org_id))
            .into_boxed();

        if let Some(user_id) = filter.user_id {
            query = query.filter(leave_balances::user_id.eq(user_id));
        }
        if let Some(absence_type_id) = filter.absence_type_id {
            query = query.filter(leave_balances::absence_type_id.eq(absence_type_id));
        }
        if let Some(year) = filter.year {
            query = query.filter(leave_balances::year.eq(year));
        }

        let balances = query
            .order((leave_balances::year.desc(), leave_balances::absence_type_id.asc()))
            .load::<LeaveBalance>(&mut conn)?;

        Ok(balances)
    }

    /// Get all balances for a user for a specific year
    pub async fn get_user_balances(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        year: i32,
    ) -> Result<Vec<LeaveBalance>, AppError> {
        let mut conn = self.pool.get()?;

        let balances = leave_balances::table
            .filter(leave_balances::organization_id.eq(org_id))
            .filter(leave_balances::user_id.eq(user_id))
            .filter(leave_balances::year.eq(year))
            .order(leave_balances::absence_type_id.asc())
            .load::<LeaveBalance>(&mut conn)?;

        Ok(balances)
    }

    /// Update a leave balance
    pub async fn update(
        &self,
        org_id: Uuid,
        balance_id: Uuid,
        mut update: LeaveBalanceUpdate,
    ) -> Result<LeaveBalance, AppError> {
        let mut conn = self.pool.get()?;

        update.updated_at = Some(Utc::now());

        let affected = diesel::update(
            leave_balances::table
                .filter(leave_balances::organization_id.eq(org_id))
                .filter(leave_balances::id.eq(balance_id)),
        )
        .set(&update)
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Leave balance not found".to_string()));
        }

        self.find_by_id(org_id, balance_id).await
    }

    /// Increment used amount
    pub async fn increment_used(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        absence_type_id: Uuid,
        year: i32,
        amount: BigDecimal,
    ) -> Result<LeaveBalance, AppError> {
        let mut conn = self.pool.get()?;

        let affected = diesel::update(
            leave_balances::table
                .filter(leave_balances::organization_id.eq(org_id))
                .filter(leave_balances::user_id.eq(user_id))
                .filter(leave_balances::absence_type_id.eq(absence_type_id))
                .filter(leave_balances::year.eq(year)),
        )
        .set((
            leave_balances::used.eq(leave_balances::used + &amount),
            leave_balances::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Leave balance not found".to_string()));
        }

        self.find_by_user_type_year(org_id, user_id, absence_type_id, year)
            .await?
            .ok_or_else(|| AppError::NotFound("Leave balance not found".to_string()))
    }

    /// Decrement used amount (for cancellations)
    pub async fn decrement_used(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        absence_type_id: Uuid,
        year: i32,
        amount: BigDecimal,
    ) -> Result<LeaveBalance, AppError> {
        let mut conn = self.pool.get()?;

        let affected = diesel::update(
            leave_balances::table
                .filter(leave_balances::organization_id.eq(org_id))
                .filter(leave_balances::user_id.eq(user_id))
                .filter(leave_balances::absence_type_id.eq(absence_type_id))
                .filter(leave_balances::year.eq(year)),
        )
        .set((
            leave_balances::used.eq(leave_balances::used - &amount),
            leave_balances::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Leave balance not found".to_string()));
        }

        self.find_by_user_type_year(org_id, user_id, absence_type_id, year)
            .await?
            .ok_or_else(|| AppError::NotFound("Leave balance not found".to_string()))
    }

    /// Delete a leave balance
    pub async fn delete(&self, org_id: Uuid, balance_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let deleted = diesel::delete(
            leave_balances::table
                .filter(leave_balances::organization_id.eq(org_id))
                .filter(leave_balances::id.eq(balance_id)),
        )
        .execute(&mut conn)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Leave balance not found".to_string()));
        }

        Ok(())
    }
}
