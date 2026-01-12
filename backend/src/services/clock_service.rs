use chrono::Utc;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{ClockRestrictionMode, NotificationType, UserRole};
use crate::error::AppError;
use crate::models::{
    ClockEntry, ClockEntryResponse, ClockFilter, ClockStatus, ClockValidationResult,
    PaginatedClockEntries, Pagination, PendingClockFilter,
};
use crate::repositories::{ClockRepository, ClockRestrictionRepository, OrganizationRepository, TeamRepository, WorkScheduleRepository};
use crate::services::NotificationService;

/// Service for clock in/out operations
pub struct ClockService {
    clock_repo: ClockRepository,
    restriction_repo: ClockRestrictionRepository,
    team_repo: TeamRepository,
    org_repo: OrganizationRepository,
    work_schedule_repo: WorkScheduleRepository,
}

impl ClockService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            clock_repo: ClockRepository::new(pool.clone()),
            restriction_repo: ClockRestrictionRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool.clone()),
            org_repo: OrganizationRepository::new(pool.clone()),
            work_schedule_repo: WorkScheduleRepository::new(pool),
        }
    }

    /// Clock in - creates a new clock entry
    /// Returns validation result if restrictions block the action
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

        // Check daily clock limit if configured
        let effective = self
            .restriction_repo
            .get_effective_restriction(org_id, user_id)
            .await?;
        if let Some(ref eff) = effective {
            if let Some(max_daily) = eff.restriction.max_daily_clock_events {
                let today = Utc::now().date_naive();
                let daily_count = self
                    .clock_repo
                    .count_daily_entries(org_id, user_id, today)
                    .await?;
                if daily_count >= max_daily as i64 {
                    return Err(AppError::ValidationError(format!(
                        "Daily clock limit reached. Maximum {} clock entries per day allowed.",
                        max_daily
                    )));
                }
            }
        }

        // Check for valid approved override before validation
        let valid_override = self
            .restriction_repo
            .find_valid_approved_override(org_id, user_id, "clock_in")
            .await?;

        // Validate clock restrictions
        let validation = self.validate_clock_action(org_id, user_id, "clock_in").await?;
        if !validation.allowed {
            let message = validation.message.unwrap_or_else(|| "Clock in is not allowed at this time".to_string());
            if validation.can_request_override {
                return Err(AppError::ValidationError(format!(
                    "{}. You can request an override with justification.",
                    message
                )));
            }
            return Err(AppError::ValidationError(message));
        }

        let entry = self.clock_repo.clock_in(org_id, user_id, notes).await?;

        // Mark the override as used if one was found
        if let Some(override_req) = valid_override {
            let _ = self
                .restriction_repo
                .mark_override_as_used(org_id, override_req.id, entry.id)
                .await;
        }

        Ok(entry)
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

        // Check for valid approved override before validation
        let valid_override = self
            .restriction_repo
            .find_valid_approved_override(org_id, user_id, "clock_out")
            .await?;

        // Validate clock restrictions
        let validation = self.validate_clock_action(org_id, user_id, "clock_out").await?;
        if !validation.allowed {
            let message = validation.message.unwrap_or_else(|| "Clock out is not allowed at this time".to_string());
            if validation.can_request_override {
                return Err(AppError::ValidationError(format!(
                    "{}. You can request an override with justification.",
                    message
                )));
            }
            return Err(AppError::ValidationError(message));
        }

        let result = self.clock_repo.clock_out(org_id, entry.id, notes).await?;

        // Mark the override as used if one was found
        if let Some(override_req) = valid_override {
            let _ = self
                .restriction_repo
                .mark_override_as_used(org_id, override_req.id, result.id)
                .await;
        }

        Ok(result)
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
                None, // override_info not needed for history view
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
            None, // override_info not needed for single entry view
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
        if let Err(e) = notification_service
            .create_notification(
                org_id,
                entry.user_id,
                NotificationType::ClockApproved,
                "Clock Entry Approved".to_string(),
                format!("Your clock entry from {} has been approved.", clock_in_str),
                None,
            )
            .await
        {
            tracing::warn!(
                user_id = %entry.user_id,
                entry_id = %entry_id,
                error = %e,
                "Failed to create clock approval notification"
            );
        }

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
        if let Err(e) = notification_service
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
            .await
        {
            tracing::warn!(
                user_id = %entry.user_id,
                entry_id = %entry_id,
                error = %e,
                "Failed to create clock rejection notification"
            );
        }

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

        // Batch fetch override info for all entries
        let entry_ids: Vec<Uuid> = filtered_entries.iter().map(|e| e.id).collect();
        let overrides_map = self
            .restriction_repo
            .find_overrides_by_clock_entry_ids(org_id, &entry_ids)
            .await?;

        let mut responses = Vec::with_capacity(filtered_entries.len());
        for entry in &filtered_entries {
            let (user_name, user_email) = self.clock_repo.get_user_info(entry.user_id).await?;
            let teams = self.team_repo.get_user_teams(org_id, entry.user_id).await.unwrap_or_default();
            let team_id = teams.first().map(|t| t.id);
            let team_name = teams.first().map(|t| t.name.clone());

            // Get override info if this entry was made via override
            let override_info = overrides_map.get(&entry.id).map(|o| {
                (o.id, o.reason.clone(), o.status)
            });

            // Calculate theoretical hours for this entry's day from user's schedule
            let theoretical_hours = self
                .work_schedule_repo
                .get_theoretical_hours(
                    org_id,
                    entry.user_id,
                    entry.clock_in.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc(),
                    entry.clock_in.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc(),
                )
                .await
                .ok()
                .filter(|h| *h > 0.0);

            responses.push(ClockEntryResponse::from_entry(
                entry, org_name.clone(), user_name, user_email, team_id, team_name, None, theoretical_hours, override_info,
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

    // =====================
    // Clock Restriction Validation
    // =====================

    /// Validate if a clock action is allowed based on restrictions
    /// Checks for approved overrides before blocking
    pub async fn validate_clock_action(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        action: &str,
    ) -> Result<ClockValidationResult, AppError> {
        use chrono::{NaiveTime, Timelike};

        // Get effective restriction for the user
        let effective = self
            .restriction_repo
            .get_effective_restriction(org_id, user_id)
            .await?;

        // If no restriction, user can clock freely
        let Some(effective_restriction) = effective else {
            return Ok(ClockValidationResult {
                allowed: true,
                message: None,
                can_request_override: false,
                effective_restriction: None,
            });
        };

        let restriction = &effective_restriction.restriction;

        // Unrestricted mode - always allowed
        if restriction.mode == ClockRestrictionMode::Unrestricted {
            return Ok(ClockValidationResult {
                allowed: true,
                message: None,
                can_request_override: false,
                effective_restriction: Some(effective_restriction),
            });
        }

        // Get current time
        let now = Utc::now();
        let current_time = NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second())
            .unwrap_or_default();

        // Check time window based on action
        let (earliest, latest) = if action == "clock_in" {
            (restriction.clock_in_earliest, restriction.clock_in_latest)
        } else {
            (restriction.clock_out_earliest, restriction.clock_out_latest)
        };

        let within_window = self.check_time_window(current_time, earliest, latest);

        if within_window {
            return Ok(ClockValidationResult {
                allowed: true,
                message: None,
                can_request_override: false,
                effective_restriction: Some(effective_restriction),
            });
        }

        // Outside allowed window - check if there's a valid approved override
        let valid_override = self
            .restriction_repo
            .find_valid_approved_override(org_id, user_id, action)
            .await?;

        if valid_override.is_some() {
            // User has an approved override - allow the action
            return Ok(ClockValidationResult {
                allowed: true,
                message: None,
                can_request_override: false,
                effective_restriction: Some(effective_restriction),
            });
        }

        // No valid override - build restriction message
        let message = self.build_restriction_message(action, earliest, latest);

        match restriction.mode {
            ClockRestrictionMode::Strict => Ok(ClockValidationResult {
                allowed: false,
                message: Some(message),
                can_request_override: false,
                effective_restriction: Some(effective_restriction),
            }),
            ClockRestrictionMode::Flexible => Ok(ClockValidationResult {
                allowed: false,
                message: Some(message),
                can_request_override: true,
                effective_restriction: Some(effective_restriction),
            }),
            ClockRestrictionMode::Unrestricted => Ok(ClockValidationResult {
                allowed: true,
                message: None,
                can_request_override: false,
                effective_restriction: Some(effective_restriction),
            }),
        }
    }

    /// Get the current clock restrictions status for a user (for UI display)
    pub async fn get_restriction_status(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<ClockValidationResult>, AppError> {
        // Return both clock_in and clock_out validation status
        // For simplicity, we return the clock_in validation
        let validation = self.validate_clock_action(org_id, user_id, "clock_in").await?;
        Ok(Some(validation))
    }

    fn check_time_window(
        &self,
        current: chrono::NaiveTime,
        earliest: Option<chrono::NaiveTime>,
        latest: Option<chrono::NaiveTime>,
    ) -> bool {
        let after_earliest = earliest.is_none_or(|e| current >= e);
        let before_latest = latest.is_none_or(|l| current <= l);
        after_earliest && before_latest
    }

    fn build_restriction_message(
        &self,
        action: &str,
        earliest: Option<chrono::NaiveTime>,
        latest: Option<chrono::NaiveTime>,
    ) -> String {
        let action_name = if action == "clock_in" {
            "Clock in"
        } else {
            "Clock out"
        };

        match (earliest, latest) {
            (Some(e), Some(l)) => {
                format!(
                    "{} is only allowed between {} and {}",
                    action_name,
                    e.format("%H:%M"),
                    l.format("%H:%M")
                )
            }
            (Some(e), None) => {
                format!("{} is not allowed before {}", action_name, e.format("%H:%M"))
            }
            (None, Some(l)) => {
                format!("{} is not allowed after {}", action_name, l.format("%H:%M"))
            }
            (None, None) => format!("{} is currently restricted", action_name),
        }
    }
}
