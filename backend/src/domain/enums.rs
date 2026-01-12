use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::schema::sql_types::AbsenceStatus as AbsenceStatusSqlType;
use crate::schema::sql_types::AuditAction as AuditActionSqlType;
use crate::schema::sql_types::BreakTrackingMode as BreakTrackingModeSqlType;
use crate::schema::sql_types::ClockEntryStatus as ClockEntryStatusSqlType;
use crate::schema::sql_types::ClockOverrideStatus as ClockOverrideStatusSqlType;
use crate::schema::sql_types::ClockRestrictionMode as ClockRestrictionModeSqlType;
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

/// Audit action enumeration matching the database audit_action ENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = AuditActionSqlType)]
#[serde(rename_all = "lowercase")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
}

impl ToSql<AuditActionSqlType, Pg> for AuditAction {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let action_str = match self {
            AuditAction::Create => "create",
            AuditAction::Update => "update",
            AuditAction::Delete => "delete",
        };
        out.write_all(action_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<AuditActionSqlType, Pg> for AuditAction {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let action_str = std::str::from_utf8(bytes.as_bytes())?;
        match action_str {
            "create" => Ok(AuditAction::Create),
            "update" => Ok(AuditAction::Update),
            "delete" => Ok(AuditAction::Delete),
            _ => Err(format!("Unrecognized audit action: {}", action_str).into()),
        }
    }
}

/// Clock restriction mode enumeration matching the database clock_restriction_mode ENUM
/// - Strict: No override possible, must be within time window
/// - Flexible: Override with justification (auto-approved or pending manager approval)
/// - Unrestricted: No time restrictions enforced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = ClockRestrictionModeSqlType)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ClockRestrictionMode {
    Strict,
    #[default]
    Flexible,
    Unrestricted,
}

impl ToSql<ClockRestrictionModeSqlType, Pg> for ClockRestrictionMode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let mode_str = match self {
            ClockRestrictionMode::Strict => "strict",
            ClockRestrictionMode::Flexible => "flexible",
            ClockRestrictionMode::Unrestricted => "unrestricted",
        };
        out.write_all(mode_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<ClockRestrictionModeSqlType, Pg> for ClockRestrictionMode {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let mode_str = std::str::from_utf8(bytes.as_bytes())?;
        match mode_str {
            "strict" => Ok(ClockRestrictionMode::Strict),
            "flexible" => Ok(ClockRestrictionMode::Flexible),
            "unrestricted" => Ok(ClockRestrictionMode::Unrestricted),
            _ => Err(format!("Unrecognized clock restriction mode: {}", mode_str).into()),
        }
    }
}

/// Clock override request status enumeration matching the database clock_override_status ENUM
/// - Pending: Waiting for manager review
/// - Approved: Manager approved the override request
/// - Rejected: Manager rejected the override request
/// - AutoApproved: Automatically approved (flexible mode without require_manager_approval)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = ClockOverrideStatusSqlType)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum ClockOverrideStatus {
    #[default]
    Pending,
    Approved,
    Rejected,
    AutoApproved,
}

impl ToSql<ClockOverrideStatusSqlType, Pg> for ClockOverrideStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let status_str = match self {
            ClockOverrideStatus::Pending => "pending",
            ClockOverrideStatus::Approved => "approved",
            ClockOverrideStatus::Rejected => "rejected",
            ClockOverrideStatus::AutoApproved => "auto_approved",
        };
        out.write_all(status_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<ClockOverrideStatusSqlType, Pg> for ClockOverrideStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let status_str = std::str::from_utf8(bytes.as_bytes())?;
        match status_str {
            "pending" => Ok(ClockOverrideStatus::Pending),
            "approved" => Ok(ClockOverrideStatus::Approved),
            "rejected" => Ok(ClockOverrideStatus::Rejected),
            "auto_approved" => Ok(ClockOverrideStatus::AutoApproved),
            _ => Err(format!("Unrecognized clock override status: {}", status_str).into()),
        }
    }
}

/// Break tracking mode enumeration matching the database break_tracking_mode ENUM
/// - AutoDeduct: Breaks are automatically deducted from worked hours based on policy
/// - ExplicitTracking: Users must explicitly start/end breaks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = BreakTrackingModeSqlType)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum BreakTrackingMode {
    #[default]
    AutoDeduct,
    ExplicitTracking,
}

impl ToSql<BreakTrackingModeSqlType, Pg> for BreakTrackingMode {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let mode_str = match self {
            BreakTrackingMode::AutoDeduct => "auto_deduct",
            BreakTrackingMode::ExplicitTracking => "explicit_tracking",
        };
        out.write_all(mode_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<BreakTrackingModeSqlType, Pg> for BreakTrackingMode {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let mode_str = std::str::from_utf8(bytes.as_bytes())?;
        match mode_str {
            "auto_deduct" => Ok(BreakTrackingMode::AutoDeduct),
            "explicit_tracking" => Ok(BreakTrackingMode::ExplicitTracking),
            _ => Err(format!("Unrecognized break tracking mode: {}", mode_str).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_type_serialization() {
        // Test JSON serialization uses snake_case
        let json = serde_json::to_string(&NotificationType::AbsenceApproved).unwrap();
        assert_eq!(json, "\"absence_approved\"");

        let json = serde_json::to_string(&NotificationType::ClockCorrection).unwrap();
        assert_eq!(json, "\"clock_correction\"");
    }

    #[test]
    fn test_notification_type_deserialization() {
        let notif: NotificationType = serde_json::from_str("\"absence_rejected\"").unwrap();
        assert_eq!(notif, NotificationType::AbsenceRejected);

        let notif: NotificationType = serde_json::from_str("\"clock_approved\"").unwrap();
        assert_eq!(notif, NotificationType::ClockApproved);
    }

    #[test]
    fn test_user_role_hierarchy() {
        // Test that role comparisons work correctly
        assert!(UserRole::SuperAdmin > UserRole::Admin);
        assert!(UserRole::Admin > UserRole::Manager);
        assert!(UserRole::Manager > UserRole::Employee);
    }

    #[test]
    fn test_user_role_serialization() {
        // UserRole uses PascalCase for JSON (no rename_all attribute)
        let json = serde_json::to_string(&UserRole::SuperAdmin).unwrap();
        assert_eq!(json, "\"SuperAdmin\"");

        let json = serde_json::to_string(&UserRole::Employee).unwrap();
        assert_eq!(json, "\"Employee\"");
    }

    #[test]
    fn test_absence_status_serialization() {
        let json = serde_json::to_string(&AbsenceStatus::Pending).unwrap();
        assert_eq!(json, "\"pending\"");

        let json = serde_json::to_string(&AbsenceStatus::Approved).unwrap();
        assert_eq!(json, "\"approved\"");
    }

    #[test]
    fn test_clock_entry_status_serialization() {
        let json = serde_json::to_string(&ClockEntryStatus::Pending).unwrap();
        assert_eq!(json, "\"pending\"");

        let json = serde_json::to_string(&ClockEntryStatus::Approved).unwrap();
        assert_eq!(json, "\"approved\"");
    }

    #[test]
    fn test_audit_action_serialization() {
        let json = serde_json::to_string(&AuditAction::Create).unwrap();
        assert_eq!(json, "\"create\"");

        let json = serde_json::to_string(&AuditAction::Update).unwrap();
        assert_eq!(json, "\"update\"");
    }
}
