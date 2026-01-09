use chrono::NaiveDate;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{ClosedDayFilter, ClosedDayResponse, ClosedDayUpdate, NewClosedDay};
use crate::repositories::ClosedDayRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Request to create a closed day
#[derive(Debug, Deserialize)]
pub struct CreateClosedDayRequest {
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: Option<bool>,
}

/// Request to update a closed day
#[derive(Debug, Deserialize)]
pub struct UpdateClosedDayRequest {
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}

/// Service for closed day operations
pub struct ClosedDayService {
    closed_day_repo: ClosedDayRepository,
}

impl ClosedDayService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            closed_day_repo: ClosedDayRepository::new(pool),
        }
    }

    /// Create a new closed day
    pub async fn create(
        &self,
        org_id: Uuid,
        request: CreateClosedDayRequest,
    ) -> Result<ClosedDayResponse, AppError> {
        if request.name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Closed day name cannot be empty".to_string(),
            ));
        }

        let new_closed_day = NewClosedDay {
            organization_id: org_id,
            name: request.name.trim().to_string(),
            date: request.date,
            is_recurring: request.is_recurring.unwrap_or(false),
        };

        let closed_day = self.closed_day_repo.create(new_closed_day).await?;
        Ok(ClosedDayResponse::from(closed_day))
    }

    /// Get a closed day by ID
    pub async fn get(
        &self,
        org_id: Uuid,
        closed_day_id: Uuid,
    ) -> Result<ClosedDayResponse, AppError> {
        let closed_day = self.closed_day_repo.find_by_id(org_id, closed_day_id).await?;
        Ok(ClosedDayResponse::from(closed_day))
    }

    /// List closed days with filters
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: ClosedDayFilter,
    ) -> Result<Vec<ClosedDayResponse>, AppError> {
        let closed_days = self.closed_day_repo.list(org_id, &filter).await?;
        Ok(closed_days.into_iter().map(ClosedDayResponse::from).collect())
    }

    /// List all closed days for a year
    pub async fn list_for_year(
        &self,
        org_id: Uuid,
        year: i32,
    ) -> Result<Vec<ClosedDayResponse>, AppError> {
        let closed_days = self.closed_day_repo.list_for_year(org_id, year).await?;
        Ok(closed_days.into_iter().map(ClosedDayResponse::from).collect())
    }

    /// Update a closed day
    pub async fn update(
        &self,
        org_id: Uuid,
        closed_day_id: Uuid,
        request: UpdateClosedDayRequest,
    ) -> Result<ClosedDayResponse, AppError> {
        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Closed day name cannot be empty".to_string(),
                ));
            }
        }

        let update = ClosedDayUpdate {
            name: request.name.map(|n| n.trim().to_string()),
            date: request.date,
            is_recurring: request.is_recurring,
        };

        let closed_day = self.closed_day_repo.update(org_id, closed_day_id, update).await?;
        Ok(ClosedDayResponse::from(closed_day))
    }

    /// Delete a closed day
    pub async fn delete(&self, org_id: Uuid, closed_day_id: Uuid) -> Result<(), AppError> {
        self.closed_day_repo.delete(org_id, closed_day_id).await
    }

    /// Check if a date is a closed day
    pub async fn is_closed_day(&self, org_id: Uuid, date: NaiveDate) -> Result<bool, AppError> {
        self.closed_day_repo.is_closed_day(org_id, date).await
    }

    /// Get closed days in a date range (for calendar view)
    pub async fn get_range(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<NaiveDate>, AppError> {
        self.closed_day_repo.list_range(org_id, start_date, end_date).await
    }
}
