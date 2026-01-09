use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::leave_balances;

/// LeaveBalance entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = leave_balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LeaveBalance {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub absence_type_id: Uuid,
    pub year: i32,
    pub initial_balance: BigDecimal,
    pub used: BigDecimal,
    pub adjustment: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// NewLeaveBalance for creating leave balances
#[derive(Debug, Insertable)]
#[diesel(table_name = leave_balances)]
pub struct NewLeaveBalance {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub absence_type_id: Uuid,
    pub year: i32,
    pub initial_balance: BigDecimal,
}

/// LeaveBalance update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = leave_balances)]
pub struct LeaveBalanceUpdate {
    pub initial_balance: Option<BigDecimal>,
    pub used: Option<BigDecimal>,
    pub adjustment: Option<BigDecimal>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// LeaveBalance response for API with enriched data
#[derive(Debug, Serialize)]
pub struct LeaveBalanceResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub absence_type_id: Uuid,
    pub type_name: String,
    pub type_code: String,
    pub type_color: String,
    pub year: i32,
    pub initial_balance: f64,
    pub used: f64,
    pub adjustment: f64,
    pub remaining: f64,
}

impl LeaveBalanceResponse {
    pub fn from_balance(
        balance: &LeaveBalance,
        type_name: String,
        type_code: String,
        type_color: String,
    ) -> Self {
        use bigdecimal::ToPrimitive;

        let initial = balance.initial_balance.to_f64().unwrap_or(0.0);
        let used = balance.used.to_f64().unwrap_or(0.0);
        let adj = balance.adjustment.to_f64().unwrap_or(0.0);

        Self {
            id: balance.id,
            user_id: balance.user_id,
            absence_type_id: balance.absence_type_id,
            type_name,
            type_code,
            type_color,
            year: balance.year,
            initial_balance: initial,
            used,
            adjustment: adj,
            remaining: initial - used + adj,
        }
    }
}

/// LeaveBalance filter options
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LeaveBalanceFilter {
    pub user_id: Option<Uuid>,
    pub absence_type_id: Option<Uuid>,
    pub year: Option<i32>,
}
