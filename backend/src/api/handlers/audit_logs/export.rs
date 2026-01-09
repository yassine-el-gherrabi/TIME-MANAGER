use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

use crate::config::AppState;
use crate::domain::enums::AuditAction;
use crate::error::AppError;
use crate::extractors::role_guard::{RoleGuard, SuperAdmin};
use crate::models::AuditLogFilter;
use crate::services::AuditService;

#[derive(Debug, Deserialize)]
pub struct ExportAuditLogsQuery {
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

/// GET /api/v1/audit-logs/export
///
/// Export audit logs as CSV.
/// Super Admin access only.
/// Limited to 10,000 records.
pub async fn export_audit_logs(
    State(state): State<AppState>,
    RoleGuard(user, _): RoleGuard<SuperAdmin>,
    Query(query): Query<ExportAuditLogsQuery>,
) -> Result<Response, AppError> {
    let service = AuditService::new(state.db_pool.clone());

    let filter = AuditLogFilter {
        entity_type: query.entity_type,
        action: query.action,
        user_id: query.user_id,
        entity_id: query.entity_id,
        start_date: query.start_date,
        end_date: query.end_date,
    };

    // Super Admin can see all organizations (pass None)
    let csv_content = service.export_csv(None, filter).await?;

    // Log export access
    tracing::info!(
        user_id = %user.0.sub,
        role = ?user.0.role,
        "Super Admin exported audit logs to CSV"
    );

    // Generate filename with current timestamp
    let filename = format!(
        "audit_logs_{}.csv",
        chrono::Utc::now().format("%Y%m%d_%H%M%S")
    );

    // Build response with proper headers
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename))
            .unwrap_or_else(|_| HeaderValue::from_static("attachment; filename=\"audit_logs.csv\"")),
    );

    Ok((StatusCode::OK, headers, csv_content).into_response())
}
