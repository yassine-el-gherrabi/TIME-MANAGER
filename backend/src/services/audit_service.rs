use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::enums::AuditAction;
use crate::error::AppError;
use crate::models::{
    AuditContext, AuditLogFilter, AuditLogResponse, AuditUserInfo, NewAuditLog,
    PaginatedAuditLogs, Pagination,
};
use crate::repositories::AuditRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Audit service for logging and querying audit trail
pub struct AuditService {
    audit_repo: AuditRepository,
}

impl AuditService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            audit_repo: AuditRepository::new(pool),
        }
    }

    /// Log a CREATE action
    pub async fn log_create<T: Serialize>(
        &self,
        ctx: &AuditContext,
        entity_type: &str,
        entity_id: Uuid,
        new_entity: &T,
    ) -> Result<(), AppError> {
        let new_values = serde_json::to_value(new_entity).ok();

        let new_audit_log = NewAuditLog {
            organization_id: ctx.organization_id,
            user_id: ctx.user_id,
            action: AuditAction::Create,
            entity_type: entity_type.to_string(),
            entity_id,
            old_values: None,
            new_values,
            ip_address: ctx.ip_address.clone(),
            user_agent: ctx.user_agent.clone(),
        };

        // Fire and forget - log errors but don't fail the operation
        if let Err(e) = self.audit_repo.create(new_audit_log).await {
            tracing::error!("Failed to create audit log: {:?}", e);
        }

        Ok(())
    }

    /// Log an UPDATE action
    pub async fn log_update<T: Serialize>(
        &self,
        ctx: &AuditContext,
        entity_type: &str,
        entity_id: Uuid,
        old_entity: &T,
        new_entity: &T,
    ) -> Result<(), AppError> {
        let old_values = serde_json::to_value(old_entity).ok();
        let new_values = serde_json::to_value(new_entity).ok();

        let new_audit_log = NewAuditLog {
            organization_id: ctx.organization_id,
            user_id: ctx.user_id,
            action: AuditAction::Update,
            entity_type: entity_type.to_string(),
            entity_id,
            old_values,
            new_values,
            ip_address: ctx.ip_address.clone(),
            user_agent: ctx.user_agent.clone(),
        };

        // Fire and forget - log errors but don't fail the operation
        if let Err(e) = self.audit_repo.create(new_audit_log).await {
            tracing::error!("Failed to create audit log: {:?}", e);
        }

        Ok(())
    }

    /// Log a DELETE action
    pub async fn log_delete<T: Serialize>(
        &self,
        ctx: &AuditContext,
        entity_type: &str,
        entity_id: Uuid,
        old_entity: &T,
    ) -> Result<(), AppError> {
        let old_values = serde_json::to_value(old_entity).ok();

        let new_audit_log = NewAuditLog {
            organization_id: ctx.organization_id,
            user_id: ctx.user_id,
            action: AuditAction::Delete,
            entity_type: entity_type.to_string(),
            entity_id,
            old_values,
            new_values: None,
            ip_address: ctx.ip_address.clone(),
            user_agent: ctx.user_agent.clone(),
        };

        // Fire and forget - log errors but don't fail the operation
        if let Err(e) = self.audit_repo.create(new_audit_log).await {
            tracing::error!("Failed to create audit log: {:?}", e);
        }

        Ok(())
    }

    /// List audit logs with pagination and filters
    /// org_id is None for Super Admin (can see all orgs), Some for scoped queries
    pub async fn list(
        &self,
        org_id: Option<Uuid>,
        filter: AuditLogFilter,
        pagination: Pagination,
    ) -> Result<PaginatedAuditLogs, AppError> {
        let (audit_logs, total) = self.audit_repo.list(org_id, &filter, &pagination).await?;

        // Get unique user IDs to fetch user info
        let user_ids: Vec<Uuid> = audit_logs
            .iter()
            .filter_map(|log| log.user_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // Fetch user info
        let users_info = self.audit_repo.get_users_info(user_ids).await?;
        let users_map: HashMap<Uuid, AuditUserInfo> = users_info
            .into_iter()
            .map(|u| (u.id, u))
            .collect();

        // Build response with user info
        let data: Vec<AuditLogResponse> = audit_logs
            .into_iter()
            .map(|log| {
                let user = log.user_id.and_then(|id| users_map.get(&id).cloned());
                AuditLogResponse::from_audit_log(log, user)
            })
            .collect();

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedAuditLogs {
            data,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    /// Export audit logs to CSV format
    pub async fn export_csv(
        &self,
        org_id: Option<Uuid>,
        filter: AuditLogFilter,
    ) -> Result<String, AppError> {
        let audit_logs = self.audit_repo.list_for_export(org_id, &filter).await?;

        // Get unique user IDs to fetch user info
        let user_ids: Vec<Uuid> = audit_logs
            .iter()
            .filter_map(|log| log.user_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // Fetch user info
        let users_info = self.audit_repo.get_users_info(user_ids).await?;
        let users_map: HashMap<Uuid, AuditUserInfo> = users_info
            .into_iter()
            .map(|u| (u.id, u))
            .collect();

        // Build CSV
        let mut csv = String::from(
            "Timestamp,Action,Entity Type,Entity ID,User Email,User Name,IP Address,Old Values,New Values\n",
        );

        for log in audit_logs {
            let user = log.user_id.and_then(|id| users_map.get(&id));
            let user_email = user.map(|u| u.email.as_str()).unwrap_or("-");
            let user_name = user
                .map(|u| format!("{} {}", u.first_name, u.last_name))
                .unwrap_or_else(|| "-".to_string());

            let old_values = log
                .old_values
                .map(|v| escape_csv(&v.to_string()))
                .unwrap_or_default();
            let new_values = log
                .new_values
                .map(|v| escape_csv(&v.to_string()))
                .unwrap_or_default();

            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                log.created_at.format("%Y-%m-%d %H:%M:%S UTC"),
                format!("{:?}", log.action).to_lowercase(),
                log.entity_type,
                log.entity_id,
                escape_csv(user_email),
                escape_csv(&user_name),
                log.ip_address.as_deref().unwrap_or("-"),
                old_values,
                new_values,
            ));
        }

        Ok(csv)
    }
}

/// Escape CSV field value
fn escape_csv(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
