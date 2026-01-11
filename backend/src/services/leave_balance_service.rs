use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{
    LeaveBalanceFilter, LeaveBalanceResponse, LeaveBalanceUpdate, NewLeaveBalance,
};
use crate::repositories::{AbsenceTypeRepository, LeaveBalanceRepository};

/// Request to create or update a leave balance
#[derive(Debug, Deserialize)]
pub struct SetBalanceRequest {
    pub absence_type_id: Uuid,
    pub year: i32,
    pub initial_balance: f64,
}

/// Request to adjust a balance
#[derive(Debug, Deserialize)]
pub struct AdjustBalanceRequest {
    pub adjustment: f64,
    pub reason: Option<String>,
}

/// Service for leave balance operations
pub struct LeaveBalanceService {
    balance_repo: LeaveBalanceRepository,
    absence_type_repo: AbsenceTypeRepository,
}

impl LeaveBalanceService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            balance_repo: LeaveBalanceRepository::new(pool.clone()),
            absence_type_repo: AbsenceTypeRepository::new(pool),
        }
    }

    /// Get balances for current user
    pub async fn get_my_balances(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Vec<LeaveBalanceResponse>, AppError> {
        let year = Utc::now().year();
        self.get_user_balances(org_id, user_id, year).await
    }

    /// Get balances for a specific user and year
    pub async fn get_user_balances(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        year: i32,
    ) -> Result<Vec<LeaveBalanceResponse>, AppError> {
        let balances = self.balance_repo.get_user_balances(org_id, user_id, year).await?;

        let mut responses = Vec::with_capacity(balances.len());
        for balance in balances {
            let absence_type = self
                .absence_type_repo
                .find_by_id(org_id, balance.absence_type_id)
                .await?;

            responses.push(LeaveBalanceResponse::from_balance(
                &balance,
                absence_type.name,
                absence_type.code,
                absence_type.color.unwrap_or_else(|| "#3B82F6".to_string()),
            ));
        }

        Ok(responses)
    }

    /// Set initial balance for a user
    pub async fn set_balance(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        request: SetBalanceRequest,
    ) -> Result<LeaveBalanceResponse, AppError> {
        // Validate year
        let current_year = Utc::now().year();
        if request.year < current_year - 1 || request.year > current_year + 1 {
            return Err(AppError::ValidationError(
                "Year must be within one year of current year".to_string(),
            ));
        }

        if request.initial_balance < 0.0 {
            return Err(AppError::ValidationError(
                "Initial balance cannot be negative".to_string(),
            ));
        }

        // Verify absence type exists and affects balance
        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, request.absence_type_id)
            .await?;

        if !absence_type.affects_balance {
            return Err(AppError::ValidationError(
                "This absence type does not use balance tracking".to_string(),
            ));
        }

        // Check if balance already exists
        let existing = self
            .balance_repo
            .find_by_user_type_year(org_id, user_id, request.absence_type_id, request.year)
            .await?;

        let balance = if let Some(b) = existing {
            // Update existing
            let update = LeaveBalanceUpdate {
                initial_balance: Some(BigDecimal::try_from(request.initial_balance).unwrap_or_default()),
                ..Default::default()
            };
            self.balance_repo.update(org_id, b.id, update).await?
        } else {
            // Create new
            let new_balance = NewLeaveBalance {
                organization_id: org_id,
                user_id,
                absence_type_id: request.absence_type_id,
                year: request.year,
                initial_balance: BigDecimal::try_from(request.initial_balance).unwrap_or_default(),
            };
            self.balance_repo.create(new_balance).await?
        };

        Ok(LeaveBalanceResponse::from_balance(
            &balance,
            absence_type.name,
            absence_type.code,
            absence_type.color.unwrap_or_else(|| "#3B82F6".to_string()),
        ))
    }

    /// Adjust balance (add or subtract from adjustment field)
    pub async fn adjust_balance(
        &self,
        org_id: Uuid,
        balance_id: Uuid,
        request: AdjustBalanceRequest,
    ) -> Result<LeaveBalanceResponse, AppError> {
        // Get current balance
        let balance = self.balance_repo.find_by_id(org_id, balance_id).await?;

        // Calculate new adjustment
        let current_adjustment = balance.adjustment.to_f64().unwrap_or(0.0);
        let new_adjustment = current_adjustment + request.adjustment;

        let update = LeaveBalanceUpdate {
            adjustment: Some(BigDecimal::try_from(new_adjustment).unwrap_or_default()),
            ..Default::default()
        };

        let updated = self.balance_repo.update(org_id, balance_id, update).await?;

        // Get absence type for response
        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, updated.absence_type_id)
            .await?;

        Ok(LeaveBalanceResponse::from_balance(
            &updated,
            absence_type.name,
            absence_type.code,
            absence_type.color.unwrap_or_else(|| "#3B82F6".to_string()),
        ))
    }

    /// Get balance by ID
    pub async fn get_balance(
        &self,
        org_id: Uuid,
        balance_id: Uuid,
    ) -> Result<LeaveBalanceResponse, AppError> {
        let balance = self.balance_repo.find_by_id(org_id, balance_id).await?;

        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, balance.absence_type_id)
            .await?;

        Ok(LeaveBalanceResponse::from_balance(
            &balance,
            absence_type.name,
            absence_type.code,
            absence_type.color.unwrap_or_else(|| "#3B82F6".to_string()),
        ))
    }

    /// List balances with filters (Admin only)
    pub async fn list_balances(
        &self,
        org_id: Uuid,
        filter: LeaveBalanceFilter,
    ) -> Result<Vec<LeaveBalanceResponse>, AppError> {
        let balances = self.balance_repo.list(org_id, &filter).await?;

        let mut responses = Vec::with_capacity(balances.len());
        for balance in balances {
            let absence_type = self
                .absence_type_repo
                .find_by_id(org_id, balance.absence_type_id)
                .await?;

            responses.push(LeaveBalanceResponse::from_balance(
                &balance,
                absence_type.name,
                absence_type.code,
                absence_type.color.unwrap_or_else(|| "#3B82F6".to_string()),
            ));
        }

        Ok(responses)
    }

    /// Initialize balances for a new user based on absence types
    pub async fn initialize_user_balances(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        default_balances: &[(Uuid, f64)],
    ) -> Result<(), AppError> {
        let year = Utc::now().year();

        for (type_id, initial) in default_balances {
            let new_balance = NewLeaveBalance {
                organization_id: org_id,
                user_id,
                absence_type_id: *type_id,
                year,
                initial_balance: BigDecimal::try_from(*initial).unwrap_or_default(),
            };

            // Ignore conflict errors (balance might already exist)
            let _ = self.balance_repo.create(new_balance).await;
        }

        Ok(())
    }
}

// Trait to get current year from chrono DateTime
trait Year {
    fn year(&self) -> i32;
}

impl Year for chrono::DateTime<Utc> {
    fn year(&self) -> i32 {
        chrono::Datelike::year(&self.date_naive())
    }
}
