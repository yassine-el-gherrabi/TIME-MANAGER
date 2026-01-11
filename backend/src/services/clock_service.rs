use chrono::Utc;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{NotificationType, UserRole};
use crate::error::AppError;
use crate::models::{
    ClockEntry, ClockEntryResponse, ClockFilter, ClockStatus, PaginatedClockEntries, Pagination,
    PendingClockFilter,
};
use crate::repositories::{ClockRepository, OrganizationRepository, TeamRepository};
use crate::services::NotificationService;

/// Service for clock in/out operations
pub struct ClockService {
    clock_repo: ClockRepository,
    team_repo: TeamRepository,
    org_repo: OrganizationRepository,
}

impl ClockService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            clock_repo: ClockRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool.clone()),
            org_repo: OrganizationRepository::new(pool),
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
        if self
            .clock_repo
            .find_open_entry(org_id, user_id)
            .await?
            .is_some()
        {
            return Err(AppError::ValidationError(
                "You are already clocked in. Please clock out first.".to_string(),
            ));
        }

        self.clock_repo.clock_in(org_id, user_id, notes).await
    }

    /// Clock out - closes the current open entry with optional notes
    pub async fn clock_out(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        notes: Option<String>,
    ) -> Result<ClockEntry, AppError> {
        // Find the open entry
        let entry = self
            .clock_repo
            .find_open_entry(org_id, user_id)
            .await?
            .ok_or_else(|| AppError::ValidationError("You are not clocked in".to_string()))?;

        self.clock_repo.clock_out(org_id, entry.id, notes).await
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

        // Fetch organization name
        let organization = self.org_repo.find_by_id(org_id).await?;
        let org_name = organization.name;

        let mut responses = Vec::with_capacity(entries.len());
        for entry in &entries {
            let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
            let teams = self.team_repo.get_user_teams(org_id, entry.user_id).await.unwrap_or_default();
            let team_id = teams.first().map(|t| t.id);
            let team_name = teams.first().map(|t| t.name.clone());
            let approver_name = if let Some(approver_id) = entry.approved_by {
                let (name, _) = self.clock_repo.get_user_info(approver_id).await?;
                Some(name)
            } else {
                None
            };
            responses.push(ClockEntryResponse::from_entry(
                entry,
                org_name.clone(),
                user_name,
                user_email,
                team_id,
                team_name,
                approver_name,
                None, // theoretical_hours not needed for history view
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
        let organization = self.org_repo.find_by_id(org_id).await?;
        let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
        let teams = self.team_repo.get_user_teams(org_id, entry.user_id).await.unwrap_or_default();
        let team_id = teams.first().map(|t| t.id);
        let team_name = teams.first().map(|t| t.name.clone());
        let approver_name = if let Some(approver_id) = entry.approved_by {
            let (name, _) = self.clock_repo.get_user_info(approver_id).await?;
            Some(name)
        } else {
            None
        };

        Ok(ClockEntryResponse::from_entry(
            &entry,
            organization.name,
            user_name,
            user_email,
            team_id,
            team_name,
            approver_name,
            None, // theoretical_hours not needed for single entry view
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
            let managed_teams = self
                .team_repo
                .get_managed_teams(org_id, approver_id)
                .await?;
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

        let approved = self
            .clock_repo
            .approve(org_id, entry_id, approver_id)
            .await?;

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
            let managed_teams = self
                .team_repo
                .get_managed_teams(org_id, approver_id)
                .await?;
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
    ///
    /// - SuperAdmin: Can filter by organization_id (defaults to their org), can filter by team_id
    /// - Admin: Uses their org, can filter by team_id
    /// - Manager: Uses their org, filters by managed teams, can further filter by team_id
    pub async fn list_pending(
        &self,
        user_org_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
        filter: PendingClockFilter,
        pagination: Pagination,
    ) -> Result<PaginatedClockEntries, AppError> {
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers can view pending entries".to_string(),
            ));
        }

        // Determine which organization to query
        // SuperAdmin can specify a different org, others use their own
        let org_id = if approver_role == UserRole::SuperAdmin {
            filter.organization_id.unwrap_or(user_org_id)
        } else {
            user_org_id
        };

        let (entries, total) = self.clock_repo.list_pending(org_id, &pagination).await?;

        // Filter entries based on role and team filter
        let filtered_entries = match approver_role {
            UserRole::Manager => {
                // Managers see only their managed teams
                let managed_teams = self
                    .team_repo
                    .get_managed_teams(org_id, approver_id)
                    .await?;
                let mut filtered = Vec::new();

                for entry in entries {
                    for team in &managed_teams {
                        // If team_id filter is set, only include that team
                        if let Some(filter_team_id) = filter.team_id {
                            if team.id != filter_team_id {
                                continue;
                            }
                        }
                        if self.team_repo.is_member(team.id, entry.user_id).await? {
                            filtered.push(entry);
                            break;
                        }
                    }
                }
                filtered
            }
            _ => {
                // Admin and SuperAdmin - apply team filter if specified
                if let Some(filter_team_id) = filter.team_id {
                    let mut filtered = Vec::new();
                    for entry in entries {
                        if self.team_repo.is_member(filter_team_id, entry.user_id).await? {
                            filtered.push(entry);
                        }
                    }
                    filtered
                } else {
                    entries
                }
            }
        };

        // Fetch organization name
        let organization = self.org_repo.find_by_id(org_id).await?;
        let org_name = organization.name;

        let mut responses = Vec::with_capacity(filtered_entries.len());
        for entry in &filtered_entries {
            let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
            let teams = self.team_repo.get_user_teams(org_id, entry.user_id).await.unwrap_or_default();
            let team_id = teams.first().map(|t| t.id);
            let team_name = teams.first().map(|t| t.name.clone());
            // TODO: Calculate theoretical_hours from user's schedule when WorkScheduleRepository is added
            responses.push(ClockEntryResponse::from_entry(
                entry, org_name.clone(), user_name, user_email, team_id, team_name, None, None,
            ));
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
