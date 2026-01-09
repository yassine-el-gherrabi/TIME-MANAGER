use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::AuditAction;
use crate::error::AppError;
use crate::extractors::role_guard::{RoleGuard, SuperAdmin};
use crate::models::{AuditLogFilter, Pagination};
use crate::services::AuditService;

#[derive(Debug, Deserialize)]
pub struct ListAuditLogsQuery {
    /// Page number (1-indexed)
    pub page: Option<i64>,
    /// Items per page (max 100)
    pub per_page: Option<i64>,
    /// Filter by entity type (e.g., "users", "teams", "absences")
    pub entity_type: Option<String>,
    /// Filter by action (create, update, delete)
    pub action: Option<AuditAction>,
    /// Filter by user who performed the action
    pub user_id: Option<Uuid>,
    /// Filter by specific entity
    pub entity_id: Option<Uuid>,
    /// Filter from date (YYYY-MM-DD)
    pub start_date: Option<NaiveDate>,
    /// Filter to date (YYYY-MM-DD)
    pub end_date: Option<NaiveDate>,
}

/// GET /api/v1/audit-logs
///
/// List audit logs with filters and pagination.
/// Super Admin access only.
pub async fn list_audit_logs(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<SuperAdmin>,
    Query(query): Query<ListAuditLogsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = AuditService::new(state.db_pool.clone());

    let pagination = Pagination {
        page: query.page.unwrap_or(1).max(1),
        per_page: query.per_page.unwrap_or(20).min(100).max(1),
    };

    let filter = AuditLogFilter {
        entity_type: query.entity_type,
        action: query.action,
        user_id: query.user_id,
        entity_id: query.entity_id,
        start_date: query.start_date,
        end_date: query.end_date,
    };

    // Super Admin can see all organizations (pass None)
    let audit_logs = service.list(None, filter, pagination).await?;

    // Log access for audit trail (optional - could be omitted for audit log reads)
    tracing::info!(
        user_id = %user.0.sub,
        role = ?user.0.role,
        "Super Admin accessed audit logs"
    );

    Ok((StatusCode::OK, Json(audit_logs)))
}
