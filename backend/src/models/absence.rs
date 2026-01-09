use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::AbsenceStatus;
use crate::schema::absences;

/// Absence entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = absences)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Absence {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_count: BigDecimal,
    pub status: AbsenceStatus,
    pub reason: Option<String>,
    pub rejection_reason: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// NewAbsence for creating absences
#[derive(Debug, Insertable)]
#[diesel(table_name = absences)]
pub struct NewAbsence {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_count: BigDecimal,
    pub status: AbsenceStatus,
    pub reason: Option<String>,
}

/// Absence update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = absences)]
pub struct AbsenceUpdate {
    pub status: Option<AbsenceStatus>,
    pub rejection_reason: Option<Option<String>>,
    pub approved_by: Option<Option<Uuid>>,
    pub approved_at: Option<Option<DateTime<Utc>>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Absence response for API with enriched data
#[derive(Debug, Serialize)]
pub struct AbsenceResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub user_email: String,
    pub type_id: Uuid,
    pub type_name: String,
    pub type_code: String,
    pub type_color: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_count: f64,
    pub status: AbsenceStatus,
    pub reason: Option<String>,
    pub rejection_reason: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approver_name: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Absence filter options
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AbsenceFilter {
    pub user_id: Option<Uuid>,
    pub type_id: Option<Uuid>,
    pub status: Option<AbsenceStatus>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

/// Paginated absences response
#[derive(Debug, Serialize)]
pub struct PaginatedAbsences {
    pub data: Vec<AbsenceResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}
