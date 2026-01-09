use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{AbsenceType, AbsenceTypeResponse, AbsenceTypeUpdate, NewAbsenceType};
use crate::repositories::AbsenceTypeRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Request to create an absence type
#[derive(Debug, Deserialize)]
pub struct CreateAbsenceTypeRequest {
    pub name: String,
    pub code: String,
    pub color: Option<String>,
    pub requires_approval: Option<bool>,
    pub affects_balance: Option<bool>,
    pub is_paid: Option<bool>,
}

/// Request to update an absence type
#[derive(Debug, Deserialize)]
pub struct UpdateAbsenceTypeRequest {
    pub name: Option<String>,
    pub code: Option<String>,
    pub color: Option<String>,
    pub requires_approval: Option<bool>,
    pub affects_balance: Option<bool>,
    pub is_paid: Option<bool>,
}

/// Service for absence type operations
pub struct AbsenceTypeService {
    absence_type_repo: AbsenceTypeRepository,
}

impl AbsenceTypeService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            absence_type_repo: AbsenceTypeRepository::new(pool),
        }
    }

    /// Create a new absence type
    pub async fn create(
        &self,
        org_id: Uuid,
        request: CreateAbsenceTypeRequest,
    ) -> Result<AbsenceTypeResponse, AppError> {
        // Validate code format (uppercase, no spaces)
        let code = request.code.trim().to_uppercase();
        if code.is_empty() || code.len() > 20 {
            return Err(AppError::ValidationError(
                "Code must be 1-20 characters".to_string(),
            ));
        }

        // Check for duplicate code
        if let Some(_) = self.absence_type_repo.find_by_code(org_id, &code).await? {
            return Err(AppError::Conflict(
                "An absence type with this code already exists".to_string(),
            ));
        }

        let new_type = NewAbsenceType {
            organization_id: org_id,
            name: request.name.trim().to_string(),
            code,
            color: request.color,
            requires_approval: request.requires_approval.unwrap_or(true),
            affects_balance: request.affects_balance.unwrap_or(true),
            is_paid: request.is_paid.unwrap_or(true),
        };

        let absence_type = self.absence_type_repo.create(new_type).await?;
        Ok(AbsenceTypeResponse::from(absence_type))
    }

    /// Get an absence type by ID
    pub async fn get(
        &self,
        org_id: Uuid,
        type_id: Uuid,
    ) -> Result<AbsenceTypeResponse, AppError> {
        let absence_type = self.absence_type_repo.find_by_id(org_id, type_id).await?;
        Ok(AbsenceTypeResponse::from(absence_type))
    }

    /// List all absence types for an organization
    pub async fn list(&self, org_id: Uuid) -> Result<Vec<AbsenceTypeResponse>, AppError> {
        let types = self.absence_type_repo.list(org_id).await?;
        Ok(types.into_iter().map(AbsenceTypeResponse::from).collect())
    }

    /// List absence types that affect balance
    pub async fn list_balance_affecting(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<AbsenceTypeResponse>, AppError> {
        let types = self.absence_type_repo.list_balance_affecting(org_id).await?;
        Ok(types.into_iter().map(AbsenceTypeResponse::from).collect())
    }

    /// Update an absence type
    pub async fn update(
        &self,
        org_id: Uuid,
        type_id: Uuid,
        request: UpdateAbsenceTypeRequest,
    ) -> Result<AbsenceTypeResponse, AppError> {
        // If code is being updated, validate and check for duplicates
        let code = if let Some(ref c) = request.code {
            let normalized = c.trim().to_uppercase();
            if normalized.is_empty() || normalized.len() > 20 {
                return Err(AppError::ValidationError(
                    "Code must be 1-20 characters".to_string(),
                ));
            }

            if let Some(existing) = self.absence_type_repo.find_by_code(org_id, &normalized).await? {
                if existing.id != type_id {
                    return Err(AppError::Conflict(
                        "An absence type with this code already exists".to_string(),
                    ));
                }
            }

            Some(normalized)
        } else {
            None
        };

        let update = AbsenceTypeUpdate {
            name: request.name.map(|n| n.trim().to_string()),
            code,
            color: request.color.map(Some),
            requires_approval: request.requires_approval,
            affects_balance: request.affects_balance,
            is_paid: request.is_paid,
            updated_at: None,
        };

        let absence_type = self.absence_type_repo.update(org_id, type_id, update).await?;
        Ok(AbsenceTypeResponse::from(absence_type))
    }

    /// Delete an absence type
    pub async fn delete(&self, org_id: Uuid, type_id: Uuid) -> Result<(), AppError> {
        self.absence_type_repo.delete(org_id, type_id).await
    }

    /// Get an absence type by ID (internal use, returns full model)
    pub async fn get_type(&self, org_id: Uuid, type_id: Uuid) -> Result<AbsenceType, AppError> {
        self.absence_type_repo.find_by_id(org_id, type_id).await
    }

    /// Seed default French absence types for an organization
    pub async fn seed_default_types(&self, org_id: Uuid) -> Result<(), AppError> {
        let default_types = vec![
            ("Congés payés", "CP", "#22C55E", true, true, true),
            ("RTT", "RTT", "#3B82F6", true, true, true),
            ("Maladie", "MALADIE", "#EF4444", false, false, true),
            ("Sans solde", "SANS_SOLDE", "#6B7280", true, false, false),
            ("Congé maternité", "MATERNITE", "#EC4899", false, false, true),
            ("Congé paternité", "PATERNITE", "#8B5CF6", false, false, true),
        ];

        for (name, code, color, requires_approval, affects_balance, is_paid) in default_types {
            // Skip if already exists
            if self.absence_type_repo.find_by_code(org_id, code).await?.is_some() {
                continue;
            }

            let new_type = NewAbsenceType {
                organization_id: org_id,
                name: name.to_string(),
                code: code.to_string(),
                color: Some(color.to_string()),
                requires_approval,
                affects_balance,
                is_paid,
            };

            // Ignore errors (type might already exist from another process)
            let _ = self.absence_type_repo.create(new_type).await;
        }

        Ok(())
    }
}
