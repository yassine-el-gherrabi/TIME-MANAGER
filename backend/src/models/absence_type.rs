use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::absence_types;

/// AbsenceType entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = absence_types)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AbsenceType {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub code: String,
    pub color: Option<String>,
    pub requires_approval: bool,
    pub affects_balance: bool,
    pub is_paid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// NewAbsenceType for creating absence types
#[derive(Debug, Insertable)]
#[diesel(table_name = absence_types)]
pub struct NewAbsenceType {
    pub organization_id: Uuid,
    pub name: String,
    pub code: String,
    pub color: Option<String>,
    pub requires_approval: bool,
    pub affects_balance: bool,
    pub is_paid: bool,
}

/// AbsenceType update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = absence_types)]
pub struct AbsenceTypeUpdate {
    pub name: Option<String>,
    pub code: Option<String>,
    pub color: Option<Option<String>>,
    pub requires_approval: Option<bool>,
    pub affects_balance: Option<bool>,
    pub is_paid: Option<bool>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// AbsenceType response for API
#[derive(Debug, Serialize)]
pub struct AbsenceTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub color: String,
    pub requires_approval: bool,
    pub affects_balance: bool,
    pub is_paid: bool,
    pub created_at: DateTime<Utc>,
}

impl From<AbsenceType> for AbsenceTypeResponse {
    fn from(at: AbsenceType) -> Self {
        Self {
            id: at.id,
            name: at.name,
            code: at.code,
            color: at.color.unwrap_or_else(|| "#3B82F6".to_string()),
            requires_approval: at.requires_approval,
            affects_balance: at.affects_balance,
            is_paid: at.is_paid,
            created_at: at.created_at,
        }
    }
}
