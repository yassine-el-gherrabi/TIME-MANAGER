use chrono::NaiveDate;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{HolidayFilter, HolidayResponse, HolidayUpdate, NewHoliday};
use crate::repositories::HolidayRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Request to create a holiday
#[derive(Debug, Deserialize)]
pub struct CreateHolidayRequest {
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: Option<bool>,
}

/// Request to update a holiday
#[derive(Debug, Deserialize)]
pub struct UpdateHolidayRequest {
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub is_recurring: Option<bool>,
}

/// Service for holiday operations
pub struct HolidayService {
    holiday_repo: HolidayRepository,
}

impl HolidayService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            holiday_repo: HolidayRepository::new(pool),
        }
    }

    /// Create a new holiday
    pub async fn create(
        &self,
        org_id: Uuid,
        request: CreateHolidayRequest,
    ) -> Result<HolidayResponse, AppError> {
        if request.name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "Holiday name cannot be empty".to_string(),
            ));
        }

        let new_holiday = NewHoliday {
            organization_id: org_id,
            name: request.name.trim().to_string(),
            date: request.date,
            is_recurring: request.is_recurring.unwrap_or(false),
        };

        let holiday = self.holiday_repo.create(new_holiday).await?;
        Ok(HolidayResponse::from(holiday))
    }

    /// Get a holiday by ID
    pub async fn get(
        &self,
        org_id: Uuid,
        holiday_id: Uuid,
    ) -> Result<HolidayResponse, AppError> {
        let holiday = self.holiday_repo.find_by_id(org_id, holiday_id).await?;
        Ok(HolidayResponse::from(holiday))
    }

    /// List holidays with filters
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: HolidayFilter,
    ) -> Result<Vec<HolidayResponse>, AppError> {
        let holidays = self.holiday_repo.list(org_id, &filter).await?;
        Ok(holidays.into_iter().map(HolidayResponse::from).collect())
    }

    /// List all holidays for a year
    pub async fn list_for_year(
        &self,
        org_id: Uuid,
        year: i32,
    ) -> Result<Vec<HolidayResponse>, AppError> {
        let holidays = self.holiday_repo.list_for_year(org_id, year).await?;
        Ok(holidays.into_iter().map(HolidayResponse::from).collect())
    }

    /// Update a holiday
    pub async fn update(
        &self,
        org_id: Uuid,
        holiday_id: Uuid,
        request: UpdateHolidayRequest,
    ) -> Result<HolidayResponse, AppError> {
        if let Some(ref name) = request.name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "Holiday name cannot be empty".to_string(),
                ));
            }
        }

        let update = HolidayUpdate {
            name: request.name.map(|n| n.trim().to_string()),
            date: request.date,
            is_recurring: request.is_recurring,
        };

        let holiday = self.holiday_repo.update(org_id, holiday_id, update).await?;
        Ok(HolidayResponse::from(holiday))
    }

    /// Delete a holiday
    pub async fn delete(&self, org_id: Uuid, holiday_id: Uuid) -> Result<(), AppError> {
        self.holiday_repo.delete(org_id, holiday_id).await
    }

    /// Check if a date is a holiday
    pub async fn is_holiday(&self, org_id: Uuid, date: NaiveDate) -> Result<bool, AppError> {
        self.holiday_repo.is_holiday(org_id, date).await
    }

    /// Get holidays in a date range (for calendar view)
    pub async fn get_range(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<NaiveDate>, AppError> {
        self.holiday_repo.list_range(org_id, start_date, end_date).await
    }

    /// Seed default French holidays for an organization
    pub async fn seed_french_holidays(&self, org_id: Uuid) -> Result<(), AppError> {
        let french_holidays = vec![
            ("Jour de l'An", 1, 1, true),
            ("Fête du Travail", 5, 1, true),
            ("Victoire 1945", 5, 8, true),
            ("Fête Nationale", 7, 14, true),
            ("Assomption", 8, 15, true),
            ("Toussaint", 11, 1, true),
            ("Armistice", 11, 11, true),
            ("Noël", 12, 25, true),
        ];

        for (name, month, day, is_recurring) in french_holidays {
            let date = NaiveDate::from_ymd_opt(2024, month, day).unwrap();

            let new_holiday = NewHoliday {
                organization_id: org_id,
                name: name.to_string(),
                date,
                is_recurring,
            };

            // Ignore errors (holidays might already exist)
            let _ = self.holiday_repo.create(new_holiday).await;
        }

        Ok(())
    }
}
