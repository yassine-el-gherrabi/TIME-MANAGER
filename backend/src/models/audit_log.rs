use chrono::{DateTime, NaiveDate, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::enums::AuditAction;
use crate::schema::audit_logs;

/// AuditLog entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = audit_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuditLog {
    pub id: Uuid,
    pub organization_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// NewAuditLog for creating audit log entries
#[derive(Debug, Insertable)]
#[diesel(table_name = audit_logs)]
pub struct NewAuditLog {
    pub organization_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Filter options for audit log queries
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AuditLogFilter {
    /// Filter by entity type (e.g., "users", "teams", "absences")
    pub entity_type: Option<String>,
    /// Filter by action (create, update, delete)
    pub action: Option<AuditAction>,
    /// Filter by user who performed the action
    pub user_id: Option<Uuid>,
    /// Filter by specific entity
    pub entity_id: Option<Uuid>,
    /// Filter from date (inclusive)
    pub start_date: Option<NaiveDate>,
    /// Filter to date (inclusive)
    pub end_date: Option<NaiveDate>,
}

/// User info for audit log response
#[derive(Debug, Clone, Serialize)]
pub struct AuditUserInfo {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

/// AuditLog response for API
#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub action: AuditAction,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    /// User who performed the action (if available)
    pub user: Option<AuditUserInfo>,
}

impl AuditLogResponse {
    /// Create response from AuditLog with optional user info
    pub fn from_audit_log(log: AuditLog, user: Option<AuditUserInfo>) -> Self {
        Self {
            id: log.id,
            action: log.action,
            entity_type: log.entity_type,
            entity_id: log.entity_id,
            old_values: log.old_values,
            new_values: log.new_values,
            ip_address: log.ip_address,
            user_agent: log.user_agent,
            created_at: log.created_at,
            user,
        }
    }
}

/// Paginated audit logs response
#[derive(Debug, Serialize)]
pub struct PaginatedAuditLogs {
    pub data: Vec<AuditLogResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// Context for creating audit logs (captured from request)
#[derive(Debug, Clone, Default)]
pub struct AuditContext {
    pub user_id: Option<Uuid>,
    pub organization_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl AuditContext {
    /// Create a new audit context
    pub fn new(
        user_id: Option<Uuid>,
        organization_id: Option<Uuid>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            user_id,
            organization_id,
            ip_address,
            user_agent,
        }
    }
}
