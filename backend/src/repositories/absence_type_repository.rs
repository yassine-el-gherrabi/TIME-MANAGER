use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{AbsenceType, AbsenceTypeUpdate, NewAbsenceType};
use crate::schema::absence_types;

/// Absence type repository for database operations
pub struct AbsenceTypeRepository {
    pool: DbPool,
}

impl AbsenceTypeRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new absence type
    pub async fn create(&self, new_type: NewAbsenceType) -> Result<AbsenceType, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(absence_types::table)
            .values(&new_type)
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => {
                    AppError::Conflict("An absence type with this code already exists".to_string())
                }
                _ => AppError::DatabaseError(e),
            })
    }

    /// Find absence type by ID within organization
    pub async fn find_by_id(&self, org_id: Uuid, type_id: Uuid) -> Result<AbsenceType, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        absence_types::table
            .filter(absence_types::organization_id.eq(org_id))
            .find(type_id)
            .first::<AbsenceType>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Absence type not found".to_string()))
    }

    /// Find absence type by code within organization
    pub async fn find_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<AbsenceType>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let result = absence_types::table
            .filter(absence_types::organization_id.eq(org_id))
            .filter(absence_types::code.eq(code))
            .first::<AbsenceType>(&mut conn)
            .await
            .optional()
            .map_err(AppError::DatabaseError)?;

        Ok(result)
    }

    /// List all absence types for organization
    pub async fn list(&self, org_id: Uuid) -> Result<Vec<AbsenceType>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let types = absence_types::table
            .filter(absence_types::organization_id.eq(org_id))
            .order(absence_types::name.asc())
            .load::<AbsenceType>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(types)
    }

    /// Update an absence type
    pub async fn update(
        &self,
        org_id: Uuid,
        type_id: Uuid,
        mut update: AbsenceTypeUpdate,
    ) -> Result<AbsenceType, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        update.updated_at = Some(Utc::now());

        let affected = diesel::update(
            absence_types::table
                .filter(absence_types::organization_id.eq(org_id))
                .filter(absence_types::id.eq(type_id)),
        )
        .set(&update)
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if affected == 0 {
            return Err(AppError::NotFound("Absence type not found".to_string()));
        }

        self.find_by_id(org_id, type_id).await
    }

    /// Delete an absence type
    pub async fn delete(&self, org_id: Uuid, type_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            absence_types::table
                .filter(absence_types::organization_id.eq(org_id))
                .filter(absence_types::id.eq(type_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Absence type not found".to_string()));
        }

        Ok(())
    }

    /// Get absence types that affect balance
    pub async fn list_balance_affecting(&self, org_id: Uuid) -> Result<Vec<AbsenceType>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let types = absence_types::table
            .filter(absence_types::organization_id.eq(org_id))
            .filter(absence_types::affects_balance.eq(true))
            .order(absence_types::name.asc())
            .load::<AbsenceType>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(types)
    }
}
