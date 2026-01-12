use chrono::{NaiveTime, Timelike, Utc};
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{ClockOverrideStatus, ClockRestrictionMode, NotificationType, UserRole};
use crate::error::AppError;
use crate::models::{
    ClockOverrideFilter, ClockOverrideRequestResponse, ClockRestriction, ClockRestrictionFilter,
    ClockRestrictionResponse, ClockRestrictionUpdate, ClockValidationResult,
    CreateClockRestrictionRequest, CreateOverrideRequest, EffectiveRestriction,
    NewClockOverrideRequest, NewClockRestriction, PaginatedClockOverrideRequests, Pagination,
    ReviewOverrideRequest, UpdateClockRestrictionRequest,
};
use crate::repositories::{ClockRestrictionRepository, OrganizationRepository, TeamRepository};
use crate::services::NotificationService;

/// Service for clock restrictions and override requests
pub struct ClockRestrictionService {
    restriction_repo: ClockRestrictionRepository,
    team_repo: TeamRepository,
    org_repo: OrganizationRepository,
}

impl ClockRestrictionService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            restriction_repo: ClockRestrictionRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool.clone()),
            org_repo: OrganizationRepository::new(pool),
        }
    }

    /// Get the pool reference
    pub fn pool(&self) -> &DbPool {
        self.restriction_repo.pool()
    }

    // =====================
    // Clock Restriction Management
    // =====================

    /// Create a clock restriction (Admin+ only)
    pub async fn create_restriction(
        &self,
        org_id: Uuid,
        request: CreateClockRestrictionRequest,
        creator_role: UserRole,
    ) -> Result<ClockRestrictionResponse, AppError> {
        // Only Admin+ can create restrictions
        if creator_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage clock restrictions".to_string(),
            ));
        }

        // Validate team_id if provided
        if let Some(team_id) = request.team_id {
            let team = self.team_repo.find_by_id(org_id, team_id).await?;
            if team.organization_id != org_id {
                return Err(AppError::ValidationError(
                    "Team does not belong to this organization".to_string(),
                ));
            }
        }

        let new_restriction = NewClockRestriction {
            organization_id: org_id,
            team_id: request.team_id,
            user_id: request.user_id,
            mode: request.mode,
            clock_in_earliest: request.clock_in_earliest,
            clock_in_latest: request.clock_in_latest,
            clock_out_earliest: request.clock_out_earliest,
            clock_out_latest: request.clock_out_latest,
            enforce_schedule: request.enforce_schedule.unwrap_or(true),
            require_manager_approval: request.require_manager_approval.unwrap_or(false),
            is_active: request.is_active.unwrap_or(true),
            max_daily_clock_events: request.max_daily_clock_events,
        };

        let restriction = self
            .restriction_repo
            .create_restriction(new_restriction)
            .await?;

        self.build_restriction_response(&restriction).await
    }

    /// Get a clock restriction by ID
    pub async fn get_restriction(
        &self,
        org_id: Uuid,
        restriction_id: Uuid,
    ) -> Result<ClockRestrictionResponse, AppError> {
        let restriction = self
            .restriction_repo
            .find_restriction_by_id(org_id, restriction_id)
            .await?;

        self.build_restriction_response(&restriction).await
    }

    /// List clock restrictions for organization
    pub async fn list_restrictions(
        &self,
        org_id: Uuid,
        filter: ClockRestrictionFilter,
    ) -> Result<Vec<ClockRestrictionResponse>, AppError> {
        let restrictions = self
            .restriction_repo
            .list_restrictions(org_id, &filter)
            .await?;

        let mut responses = Vec::with_capacity(restrictions.len());
        for restriction in &restrictions {
            responses.push(self.build_restriction_response(restriction).await?);
        }

        Ok(responses)
    }

    /// Update a clock restriction
    pub async fn update_restriction(
        &self,
        org_id: Uuid,
        restriction_id: Uuid,
        request: UpdateClockRestrictionRequest,
        updater_role: UserRole,
    ) -> Result<ClockRestrictionResponse, AppError> {
        if updater_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage clock restrictions".to_string(),
            ));
        }

        let update = ClockRestrictionUpdate {
            mode: request.mode,
            clock_in_earliest: request.clock_in_earliest,
            clock_in_latest: request.clock_in_latest,
            clock_out_earliest: request.clock_out_earliest,
            clock_out_latest: request.clock_out_latest,
            enforce_schedule: request.enforce_schedule,
            require_manager_approval: request.require_manager_approval,
            is_active: request.is_active,
            max_daily_clock_events: request.max_daily_clock_events,
            updated_at: None, // Will be set by repository
        };

        let restriction = self
            .restriction_repo
            .update_restriction(org_id, restriction_id, update)
            .await?;

        self.build_restriction_response(&restriction).await
    }

    /// Delete a clock restriction
    pub async fn delete_restriction(
        &self,
        org_id: Uuid,
        restriction_id: Uuid,
        deleter_role: UserRole,
    ) -> Result<(), AppError> {
        if deleter_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage clock restrictions".to_string(),
            ));
        }

        self.restriction_repo
            .delete_restriction(org_id, restriction_id)
            .await
    }

    // =====================
    // Validation Logic
    // =====================

    /// Validate if a user can perform a clock action (clock_in or clock_out)
    pub async fn validate_clock_action(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        action: &str, // "clock_in" or "clock_out"
    ) -> Result<ClockValidationResult, AppError> {
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
        let current_time = NaiveTime::from_hms_opt(
            now.hour(),
            now.minute(),
            now.second(),
        )
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

        // Outside allowed window
        let message = self.build_restriction_message(action, earliest, latest);

        match restriction.mode {
            ClockRestrictionMode::Strict => {
                // Strict mode - no override possible
                Ok(ClockValidationResult {
                    allowed: false,
                    message: Some(message),
                    can_request_override: false,
                    effective_restriction: Some(effective_restriction),
                })
            }
            ClockRestrictionMode::Flexible => {
                // Flexible mode - can request override
                Ok(ClockValidationResult {
                    allowed: false,
                    message: Some(message),
                    can_request_override: true,
                    effective_restriction: Some(effective_restriction),
                })
            }
            ClockRestrictionMode::Unrestricted => {
                // Should not reach here, but handle gracefully
                Ok(ClockValidationResult {
                    allowed: true,
                    message: None,
                    can_request_override: false,
                    effective_restriction: Some(effective_restriction),
                })
            }
        }
    }

    /// Get effective restriction for a user
    pub async fn get_effective_restriction(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<EffectiveRestriction>, AppError> {
        self.restriction_repo
            .get_effective_restriction(org_id, user_id)
            .await
    }

    // =====================
    // Override Request Management
    // =====================

    /// Create an override request (for flexible mode)
    pub async fn create_override_request(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        request: CreateOverrideRequest,
    ) -> Result<ClockOverrideRequestResponse, AppError> {
        // Validate action type
        if request.requested_action != "clock_in" && request.requested_action != "clock_out" {
            return Err(AppError::ValidationError(
                "Invalid action. Must be 'clock_in' or 'clock_out'".to_string(),
            ));
        }

        // Check if user already has a pending request for this action
        let existing = self
            .restriction_repo
            .find_pending_override_for_user(org_id, user_id, &request.requested_action)
            .await?;

        if existing.is_some() {
            return Err(AppError::Conflict(
                "You already have a pending override request for this action".to_string(),
            ));
        }

        // Get effective restriction to determine if auto-approve is allowed
        let effective = self
            .restriction_repo
            .get_effective_restriction(org_id, user_id)
            .await?;

        let (status, should_notify_managers) = match &effective {
            Some(eff) if eff.restriction.mode == ClockRestrictionMode::Flexible => {
                if eff.restriction.require_manager_approval {
                    (ClockOverrideStatus::Pending, true)
                } else {
                    (ClockOverrideStatus::AutoApproved, false)
                }
            }
            _ => {
                return Err(AppError::ValidationError(
                    "Override requests are only available in flexible mode".to_string(),
                ))
            }
        };

        let new_request = NewClockOverrideRequest {
            organization_id: org_id,
            user_id,
            clock_entry_id: None,
            requested_action: request.requested_action.clone(),
            requested_at: Utc::now(),
            reason: request.reason,
            status,
        };

        let override_request = self
            .restriction_repo
            .create_override_request(new_request)
            .await?;

        // Notify managers if approval is required
        if should_notify_managers {
            self.notify_managers_of_override_request(org_id, user_id, &override_request.id)
                .await;
        }

        self.build_override_response(&override_request).await
    }

    /// Review an override request (Manager+ only)
    pub async fn review_override_request(
        &self,
        org_id: Uuid,
        request_id: Uuid,
        reviewer_id: Uuid,
        reviewer_role: UserRole,
        review: ReviewOverrideRequest,
    ) -> Result<ClockOverrideRequestResponse, AppError> {
        // Only Manager+ can review
        if reviewer_role < UserRole::Manager {
            return Err(AppError::Forbidden(
                "Only managers can review override requests".to_string(),
            ));
        }

        // Get the request
        let override_request = self
            .restriction_repo
            .find_override_request_by_id(org_id, request_id)
            .await?;

        // Verify it's still pending
        if override_request.status != ClockOverrideStatus::Pending {
            return Err(AppError::ValidationError(
                "This request has already been reviewed".to_string(),
            ));
        }

        // For managers, verify they manage a team the user belongs to
        if reviewer_role == UserRole::Manager {
            let managed_teams = self
                .team_repo
                .get_managed_teams(org_id, reviewer_id)
                .await?;
            let mut can_review = false;

            for team in managed_teams {
                if self
                    .team_repo
                    .is_member(team.id, override_request.user_id)
                    .await?
                {
                    can_review = true;
                    break;
                }
            }

            if !can_review {
                return Err(AppError::Forbidden(
                    "You can only review requests from members of your team".to_string(),
                ));
            }
        }

        let updated = if review.approved {
            self.restriction_repo
                .approve_override_request(org_id, request_id, reviewer_id, review.review_notes, None)
                .await?
        } else {
            self.restriction_repo
                .reject_override_request(org_id, request_id, reviewer_id, review.review_notes)
                .await?
        };

        // Notify the user
        self.notify_user_of_review_result(org_id, &updated).await;

        self.build_override_response(&updated).await
    }

    /// List pending override requests
    pub async fn list_pending_override_requests(
        &self,
        org_id: Uuid,
        reviewer_id: Uuid,
        reviewer_role: UserRole,
        pagination: Pagination,
    ) -> Result<PaginatedClockOverrideRequests, AppError> {
        if reviewer_role < UserRole::Manager {
            return Err(AppError::Forbidden(
                "Only managers can view pending override requests".to_string(),
            ));
        }

        let (requests, total) = self
            .restriction_repo
            .list_pending_override_requests(org_id, &pagination)
            .await?;

        // Filter based on role
        let filtered_requests = if reviewer_role == UserRole::Manager {
            let managed_teams = self
                .team_repo
                .get_managed_teams(org_id, reviewer_id)
                .await?;

            let mut filtered = Vec::new();
            for request in requests {
                for team in &managed_teams {
                    if self
                        .team_repo
                        .is_member(team.id, request.user_id)
                        .await
                        .unwrap_or(false)
                    {
                        filtered.push(request);
                        break;
                    }
                }
            }
            filtered
        } else {
            requests
        };

        let mut responses = Vec::with_capacity(filtered_requests.len());
        for request in &filtered_requests {
            responses.push(self.build_override_response(request).await?);
        }

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedClockOverrideRequests {
            data: responses,
            total: filtered_requests.len() as i64,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    /// List user's own override requests
    pub async fn list_user_override_requests(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        filter: ClockOverrideFilter,
        pagination: Pagination,
    ) -> Result<PaginatedClockOverrideRequests, AppError> {
        let mut filter_with_user = filter;
        filter_with_user.user_id = Some(user_id);

        let (requests, total) = self
            .restriction_repo
            .list_override_requests(org_id, &filter_with_user, &pagination)
            .await?;

        let mut responses = Vec::with_capacity(requests.len());
        for request in &requests {
            responses.push(self.build_override_response(request).await?);
        }

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedClockOverrideRequests {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    // =====================
    // Helper Methods
    // =====================

    fn check_time_window(
        &self,
        current: NaiveTime,
        earliest: Option<NaiveTime>,
        latest: Option<NaiveTime>,
    ) -> bool {
        let after_earliest = earliest.map_or(true, |e| current >= e);
        let before_latest = latest.map_or(true, |l| current <= l);
        after_earliest && before_latest
    }

    fn build_restriction_message(
        &self,
        action: &str,
        earliest: Option<NaiveTime>,
        latest: Option<NaiveTime>,
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

    async fn build_restriction_response(
        &self,
        restriction: &ClockRestriction,
    ) -> Result<ClockRestrictionResponse, AppError> {
        let org = self.org_repo.find_by_id(restriction.organization_id).await?;

        let team_name = if let Some(team_id) = restriction.team_id {
            let team = self
                .team_repo
                .find_by_id(restriction.organization_id, team_id)
                .await?;
            Some(team.name)
        } else {
            None
        };

        let user_name = if let Some(_user_id) = restriction.user_id {
            // Would need UserRepository to get user name
            // For now, return None - can be enhanced later
            None
        } else {
            None
        };

        Ok(ClockRestrictionResponse::from_restriction(
            restriction,
            org.name,
            team_name,
            user_name,
        ))
    }

    async fn build_override_response(
        &self,
        request: &crate::models::ClockOverrideRequest,
    ) -> Result<ClockOverrideRequestResponse, AppError> {
        // Would need UserRepository to get user details
        // For now, use placeholders - can be enhanced later
        let user_name = "User".to_string();
        let user_email = "".to_string();

        let reviewer_name = if request.reviewed_by.is_some() {
            Some("Reviewer".to_string())
        } else {
            None
        };

        Ok(ClockOverrideRequestResponse::from_request(
            request,
            user_name,
            user_email,
            reviewer_name,
        ))
    }

    async fn notify_managers_of_override_request(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        _request_id: &Uuid,
    ) {
        // Get managers of user's teams
        let teams = self
            .team_repo
            .get_user_teams(org_id, user_id)
            .await
            .unwrap_or_default();

        let notification_service = NotificationService::new(self.pool().clone());

        for team in teams {
            if let Some(manager_id) = team.manager_id {
                let _ = notification_service
                    .create_notification(
                        org_id,
                        manager_id,
                        NotificationType::ClockCorrection,
                        "Override Request Pending".to_string(),
                        "A team member has requested a clock override that requires your approval."
                            .to_string(),
                        None,
                    )
                    .await;
            }
        }
    }

    async fn notify_user_of_review_result(
        &self,
        org_id: Uuid,
        request: &crate::models::ClockOverrideRequest,
    ) {
        let notification_service = NotificationService::new(self.pool().clone());

        let (title, message, notification_type) = match request.status {
            ClockOverrideStatus::Approved => (
                "Override Request Approved".to_string(),
                format!(
                    "Your {} override request has been approved.",
                    request.requested_action
                ),
                NotificationType::ClockApproved,
            ),
            ClockOverrideStatus::Rejected => (
                "Override Request Rejected".to_string(),
                format!(
                    "Your {} override request has been rejected. Reason: {}",
                    request.requested_action,
                    request.review_notes.as_deref().unwrap_or("Not specified")
                ),
                NotificationType::ClockRejected,
            ),
            _ => return,
        };

        let _ = notification_service
            .create_notification(org_id, request.user_id, notification_type, title, message, None)
            .await;
    }
}
