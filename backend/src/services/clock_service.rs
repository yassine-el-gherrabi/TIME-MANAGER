use chrono::Utc;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{NotificationType, UserRole};
use crate::error::AppError;
use crate::services::NotificationService;
use crate::models::{
    ClockEntry, ClockEntryResponse, ClockFilter, ClockStatus, PaginatedClockEntries, Pagination,
};
use crate::repositories::{ClockRepository, TeamRepository};

/// Service for clock in/out operations
pub struct ClockService {
    clock_repo: ClockRepository,
    team_repo: TeamRepository,
}

impl ClockService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            clock_repo: ClockRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool),
        }
    }

    /// Clock in - creates a new clock entry
    pub async fn clock_in(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        notes: Option<String>,
    ) -> Result<ClockEntry, AppError> {
        // Check if user already has an open clock entry
        if self.clock_repo.find_open_entry(org_id, user_id).await?.is_some() {
            return Err(AppError::ValidationError(
                "You are already clocked in. Please clock out first.".to_string(),
            ));
        }

        self.clock_repo.clock_in(org_id, user_id, notes).await
    }

    /// Clock out - closes the current open entry
    pub async fn clock_out(&self, org_id: Uuid, user_id: Uuid) -> Result<ClockEntry, AppError> {
        // Find the open entry
        let entry = self
            .clock_repo
            .find_open_entry(org_id, user_id)
            .await?
            .ok_or_else(|| AppError::ValidationError("You are not clocked in".to_string()))?;

        self.clock_repo.clock_out(org_id, entry.id).await
    }

    /// Get current clock status for a user
    pub async fn get_current_status(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<ClockStatus, AppError> {
        let entry = self.clock_repo.find_open_entry(org_id, user_id).await?;

        let (is_clocked_in, elapsed_minutes) = match &entry {
            Some(e) => {
                let elapsed = (Utc::now() - e.clock_in).num_minutes();
                (true, Some(elapsed))
            }
            None => (false, None),
        };

        Ok(ClockStatus {
            is_clocked_in,
            current_entry: entry,
            elapsed_minutes,
        })
    }

    /// Get clock history for a user with pagination
    pub async fn get_history(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        filter: ClockFilter,
        pagination: Pagination,
    ) -> Result<PaginatedClockEntries, AppError> {
        let (entries, total) = self
            .clock_repo
            .list_by_user(org_id, user_id, &filter, &pagination)
            .await?;

        let mut responses = Vec::with_capacity(entries.len());
        for entry in &entries {
            let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
            let approver_name = if let Some(approver_id) = entry.approved_by {
                let (name, _) = self.clock_repo.get_user_info(approver_id).await?;
                Some(name)
            } else {
                None
            };
            responses.push(ClockEntryResponse::from_entry(
                entry,
                user_name,
                user_email,
                approver_name,
            ));
        }

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedClockEntries {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    /// Get a single clock entry
    pub async fn get_entry(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
    ) -> Result<ClockEntryResponse, AppError> {
        let entry = self.clock_repo.find_by_id(org_id, entry_id).await?;
        let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
        let approver_name = if let Some(approver_id) = entry.approved_by {
            let (name, _) = self.clock_repo.get_user_info(approver_id).await?;
            Some(name)
        } else {
            None
        };

        Ok(ClockEntryResponse::from_entry(
            &entry,
            user_name,
            user_email,
            approver_name,
        ))
    }

    /// Approve a clock entry (Manager+ only)
    pub async fn approve_entry(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
    ) -> Result<ClockEntry, AppError> {
        // Verify approver has permission
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers can approve clock entries".to_string(),
            ));
        }

        // Get the entry to check it's pending
        let entry = self.clock_repo.find_by_id(org_id, entry_id).await?;
        if entry.clock_out.is_none() {
            return Err(AppError::ValidationError(
                "Cannot approve an open clock entry".to_string(),
            ));
        }

        // For managers, verify they manage a team the user belongs to
        if approver_role == UserRole::Manager {
            let managed_teams = self.team_repo.get_managed_teams(org_id, approver_id).await?;
            let mut can_approve = false;

            for team in managed_teams {
                if self.team_repo.is_member(team.id, entry.user_id).await? {
                    can_approve = true;
                    break;
                }
            }

            if !can_approve {
                return Err(AppError::Forbidden(
                    "You can only approve entries for members of your team".to_string(),
                ));
            }
        }

        let approved = self.clock_repo.approve(org_id, entry_id, approver_id).await?;

        // Create notification for the employee
        let notification_service = NotificationService::new(self.clock_repo.pool().clone());
        let clock_in_str = entry.clock_in.format("%Y-%m-%d %H:%M").to_string();
        let _ = notification_service
            .create_notification(
                org_id,
                entry.user_id,
                NotificationType::ClockApproved,
                "Clock Entry Approved".to_string(),
                format!("Your clock entry from {} has been approved.", clock_in_str),
                None,
            )
            .await;

        Ok(approved)
    }

    /// Reject a clock entry (Manager+ only)
    pub async fn reject_entry(
        &self,
        org_id: Uuid,
        entry_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
        reason: Option<String>,
    ) -> Result<ClockEntry, AppError> {
        // Verify approver has permission
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers can reject clock entries".to_string(),
            ));
        }

        // Get the entry to check it's pending
        let entry = self.clock_repo.find_by_id(org_id, entry_id).await?;
        if entry.clock_out.is_none() {
            return Err(AppError::ValidationError(
                "Cannot reject an open clock entry".to_string(),
            ));
        }

        // For managers, verify they manage a team the user belongs to
        if approver_role == UserRole::Manager {
            let managed_teams = self.team_repo.get_managed_teams(org_id, approver_id).await?;
            let mut can_reject = false;

            for team in managed_teams {
                if self.team_repo.is_member(team.id, entry.user_id).await? {
                    can_reject = true;
                    break;
                }
            }

            if !can_reject {
                return Err(AppError::Forbidden(
                    "You can only reject entries for members of your team".to_string(),
                ));
            }
        }

        // Keep reason for notification before moving
        let reason_text = reason.as_deref().unwrap_or("Not specified").to_string();

        let rejected = self
            .clock_repo
            .reject(org_id, entry_id, approver_id, reason)
            .await?;

        // Create notification for the employee
        let notification_service = NotificationService::new(self.clock_repo.pool().clone());
        let clock_in_str = entry.clock_in.format("%Y-%m-%d %H:%M").to_string();
        let _ = notification_service
            .create_notification(
                org_id,
                entry.user_id,
                NotificationType::ClockRejected,
                "Clock Entry Rejected".to_string(),
                format!(
                    "Your clock entry from {} has been rejected. Reason: {}",
                    clock_in_str, reason_text
                ),
                None,
            )
            .await;

        Ok(rejected)
    }

    /// List pending entries (for approval)
    pub async fn list_pending(
        &self,
        org_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
        pagination: Pagination,
    ) -> Result<PaginatedClockEntries, AppError> {
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers can view pending entries".to_string(),
            ));
        }

        let (entries, total) = self.clock_repo.list_pending(org_id, &pagination).await?;

        // For managers, filter to only their team members
        let filtered_entries = if approver_role == UserRole::Manager {
            let managed_teams = self.team_repo.get_managed_teams(org_id, approver_id).await?;
            let mut filtered = Vec::new();

            for entry in entries {
                for team in &managed_teams {
                    if self.team_repo.is_member(team.id, entry.user_id).await? {
                        filtered.push(entry);
                        break;
                    }
                }
            }
            filtered
        } else {
            entries
        };

        let mut responses = Vec::with_capacity(filtered_entries.len());
        for entry in &filtered_entries {
            let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
            responses.push(ClockEntryResponse::from_entry(entry, user_name, user_email, None));
        }

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedClockEntries {
            data: responses,
            total: filtered_entries.len() as i64,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }
}
