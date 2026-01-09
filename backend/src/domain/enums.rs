use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::schema::sql_types::AbsenceStatus as AbsenceStatusSqlType;
use crate::schema::sql_types::ClockEntryStatus as ClockEntryStatusSqlType;
use crate::schema::sql_types::NotificationType as NotificationTypeSqlType;
use crate::schema::sql_types::UserRole as UserRoleSqlType;

/// User role enumeration matching the database user_role ENUM
/// Hierarchy: SuperAdmin > Admin > Manager > Employee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = UserRoleSqlType)]
#[derive(Default)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Manager,
    #[default]
    Employee,
}

impl UserRole {
    /// Returns the numeric level of the role for comparison
    /// Higher value means more privileges
    fn level(&self) -> u8 {
        match self {
            UserRole::Employee => 0,
            UserRole::Manager => 1,
            UserRole::Admin => 2,
            UserRole::SuperAdmin => 3,
        }
    }
}

impl PartialOrd for UserRole {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UserRole {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level().cmp(&other.level())
    }
}

impl ToSql<UserRoleSqlType, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let role_str = match self {
            UserRole::SuperAdmin => "super_admin",
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
            UserRole::Employee => "employee",
        };
        out.write_all(role_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<UserRoleSqlType, Pg> for UserRole {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let role_str = std::str::from_utf8(bytes.as_bytes())?;
        match role_str {
            "super_admin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "manager" => Ok(UserRole::Manager),
            "employee" => Ok(UserRole::Employee),
            _ => Err(format!("Unrecognized user role: {}", role_str).into()),
        }
    }
}

/// Clock entry status enumeration matching the database clock_entry_status ENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = ClockEntryStatusSqlType)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ClockEntryStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
}

impl ToSql<ClockEntryStatusSqlType, Pg> for ClockEntryStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let status_str = match self {
            ClockEntryStatus::Pending => "pending",
            ClockEntryStatus::Approved => "approved",
            ClockEntryStatus::Rejected => "rejected",
        };
        out.write_all(status_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<ClockEntryStatusSqlType, Pg> for ClockEntryStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let status_str = std::str::from_utf8(bytes.as_bytes())?;
        match status_str {
            "pending" => Ok(ClockEntryStatus::Pending),
            "approved" => Ok(ClockEntryStatus::Approved),
            "rejected" => Ok(ClockEntryStatus::Rejected),
            _ => Err(format!("Unrecognized clock entry status: {}", status_str).into()),
        }
    }
}

/// Absence status enumeration matching the database absence_status ENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = AbsenceStatusSqlType)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum AbsenceStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

impl ToSql<AbsenceStatusSqlType, Pg> for AbsenceStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let status_str = match self {
            AbsenceStatus::Pending => "pending",
            AbsenceStatus::Approved => "approved",
            AbsenceStatus::Rejected => "rejected",
            AbsenceStatus::Cancelled => "cancelled",
        };
        out.write_all(status_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<AbsenceStatusSqlType, Pg> for AbsenceStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let status_str = std::str::from_utf8(bytes.as_bytes())?;
        match status_str {
            "pending" => Ok(AbsenceStatus::Pending),
            "approved" => Ok(AbsenceStatus::Approved),
            "rejected" => Ok(AbsenceStatus::Rejected),
            "cancelled" => Ok(AbsenceStatus::Cancelled),
            _ => Err(format!("Unrecognized absence status: {}", status_str).into()),
        }
    }
}

/// Notification type enumeration matching the database notification_type ENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = NotificationTypeSqlType)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    AbsenceApproved,
    AbsenceRejected,
    AbsencePending,
    ClockCorrection,
    ClockApproved,
    ClockRejected,
}

impl ToSql<NotificationTypeSqlType, Pg> for NotificationType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let type_str = match self {
            NotificationType::AbsenceApproved => "absence_approved",
            NotificationType::AbsenceRejected => "absence_rejected",
            NotificationType::AbsencePending => "absence_pending",
            NotificationType::ClockCorrection => "clock_correction",
            NotificationType::ClockApproved => "clock_approved",
            NotificationType::ClockRejected => "clock_rejected",
        };
        out.write_all(type_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<NotificationTypeSqlType, Pg> for NotificationType {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let type_str = std::str::from_utf8(bytes.as_bytes())?;
        match type_str {
            "absence_approved" => Ok(NotificationType::AbsenceApproved),
            "absence_rejected" => Ok(NotificationType::AbsenceRejected),
            "absence_pending" => Ok(NotificationType::AbsencePending),
            "clock_correction" => Ok(NotificationType::ClockCorrection),
            "clock_approved" => Ok(NotificationType::ClockApproved),
            "clock_rejected" => Ok(NotificationType::ClockRejected),
            _ => Err(format!("Unrecognized notification type: {}", type_str).into()),
        }
    }
}
