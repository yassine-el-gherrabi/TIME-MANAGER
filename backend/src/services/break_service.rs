use chrono::{DateTime, Datelike, Utc};
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{BreakTrackingMode, UserRole};
use crate::error::AppError;
use crate::models::{
    BreakDeduction, BreakEntry, BreakEntryFilter, BreakEntryResponse, BreakEntryUpdate,
    BreakPolicy, BreakPolicyFilter, BreakPolicyResponse, BreakPolicyUpdate, BreakStatus,
    BreakWindowResponse, CreateBreakPolicyRequest, CreateBreakWindowRequest, EffectiveBreakPolicy,
    EndBreakRequest, NewBreakEntry, NewBreakPolicy, NewBreakWindow, PaginatedBreakEntries,
    PaginatedBreakPolicies, Pagination, StartBreakRequest, UpdateBreakPolicyRequest,
};
use crate::repositories::{
    BreakRepository, OrganizationRepository, TeamRepository, UserRepository,
};

/// Service for break policies, windows, and entries
pub struct BreakService {
    break_repo: BreakRepository,
    team_repo: TeamRepository,
    org_repo: OrganizationRepository,
    user_repo: UserRepository,
}

impl BreakService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            break_repo: BreakRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool.clone()),
            org_repo: OrganizationRepository::new(pool.clone()),
            user_repo: UserRepository::new(pool),
        }
    }

    /// Get the pool reference
    pub fn pool(&self) -> &DbPool {
        self.break_repo.pool()
    }

    // =====================
    // Break Policy Management
    // =====================

    /// Create a break policy (Admin+ only)
    pub async fn create_policy(
        &self,
        org_id: Uuid,
        request: CreateBreakPolicyRequest,
        creator_role: UserRole,
    ) -> Result<BreakPolicyResponse, AppError> {
        // Only Admin+ can create policies
        if creator_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage break policies".to_string(),
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

        // Validate user_id if provided
        if let Some(user_id) = request.user_id {
            let user = self.user_repo.find_by_id(user_id).await?;
            if user.organization_id != org_id {
                return Err(AppError::ValidationError(
                    "User does not belong to this organization".to_string(),
                ));
            }
        }

        let new_policy = NewBreakPolicy {
            organization_id: org_id,
            team_id: request.team_id,
            user_id: request.user_id,
            name: request.name,
            description: request.description,
            tracking_mode: request.tracking_mode,
            notify_missing_break: request.notify_missing_break,
            is_active: true,
        };

        let policy = self.break_repo.create_policy(new_policy).await?;

        // Create windows if provided
        if let Some(windows) = request.windows {
            for window_req in windows {
                let new_window = NewBreakWindow {
                    break_policy_id: policy.id,
                    day_of_week: window_req.day_of_week,
                    window_start: BreakRepository::parse_time(&window_req.window_start)?,
                    window_end: BreakRepository::parse_time(&window_req.window_end)?,
                    min_duration_minutes: window_req.min_duration_minutes,
                    max_duration_minutes: window_req.max_duration_minutes,
                    is_mandatory: window_req.is_mandatory,
                };
                self.break_repo.create_window(new_window).await?;
            }
        }

        self.build_policy_response(&policy).await
    }

    /// Get a break policy by ID
    pub async fn get_policy(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
    ) -> Result<BreakPolicyResponse, AppError> {
        let policy = self.break_repo.find_policy_by_id(org_id, policy_id).await?;
        self.build_policy_response(&policy).await
    }

    /// List break policies for organization
    pub async fn list_policies(
        &self,
        org_id: Uuid,
        filter: BreakPolicyFilter,
        pagination: Pagination,
    ) -> Result<PaginatedBreakPolicies, AppError> {
        let (policies, total) = self
            .break_repo
            .list_policies(org_id, &filter, &pagination)
            .await?;

        let mut responses = Vec::with_capacity(policies.len());
        for policy in &policies {
            responses.push(self.build_policy_response(policy).await?);
        }

        Ok(PaginatedBreakPolicies {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
        })
    }

    /// Update a break policy
    pub async fn update_policy(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
        request: UpdateBreakPolicyRequest,
        updater_role: UserRole,
    ) -> Result<BreakPolicyResponse, AppError> {
        if updater_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage break policies".to_string(),
            ));
        }

        let update = BreakPolicyUpdate {
            name: request.name,
            description: request.description.map(Some),
            tracking_mode: request.tracking_mode,
            notify_missing_break: request.notify_missing_break,
            is_active: request.is_active,
        };

        let policy = self
            .break_repo
            .update_policy(org_id, policy_id, update)
            .await?;

        self.build_policy_response(&policy).await
    }

    /// Delete a break policy
    pub async fn delete_policy(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
        deleter_role: UserRole,
    ) -> Result<(), AppError> {
        if deleter_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage break policies".to_string(),
            ));
        }

        self.break_repo.delete_policy(org_id, policy_id).await
    }

    // =====================
    // Break Window Management
    // =====================

    /// Add a break window to a policy
    pub async fn add_window(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
        request: CreateBreakWindowRequest,
        updater_role: UserRole,
    ) -> Result<BreakWindowResponse, AppError> {
        if updater_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage break windows".to_string(),
            ));
        }

        // Verify policy exists and belongs to org
        let _policy = self.break_repo.find_policy_by_id(org_id, policy_id).await?;

        // Validate day of week
        if request.day_of_week < 0 || request.day_of_week > 6 {
            return Err(AppError::ValidationError(
                "Day of week must be between 0 (Sunday) and 6 (Saturday)".to_string(),
            ));
        }

        // Validate durations
        if request.min_duration_minutes <= 0 {
            return Err(AppError::ValidationError(
                "Minimum duration must be greater than 0".to_string(),
            ));
        }
        if request.max_duration_minutes < request.min_duration_minutes {
            return Err(AppError::ValidationError(
                "Maximum duration must be >= minimum duration".to_string(),
            ));
        }

        let new_window = NewBreakWindow {
            break_policy_id: policy_id,
            day_of_week: request.day_of_week,
            window_start: BreakRepository::parse_time(&request.window_start)?,
            window_end: BreakRepository::parse_time(&request.window_end)?,
            min_duration_minutes: request.min_duration_minutes,
            max_duration_minutes: request.max_duration_minutes,
            is_mandatory: request.is_mandatory,
        };

        let window = self.break_repo.create_window(new_window).await?;

        Ok(BreakWindowResponse {
            id: window.id,
            day_of_week: window.day_of_week,
            window_start: window.window_start.format("%H:%M").to_string(),
            window_end: window.window_end.format("%H:%M").to_string(),
            min_duration_minutes: window.min_duration_minutes,
            max_duration_minutes: window.max_duration_minutes,
            is_mandatory: window.is_mandatory,
        })
    }

    /// Delete a break window
    pub async fn delete_window(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
        window_id: Uuid,
        deleter_role: UserRole,
    ) -> Result<(), AppError> {
        if deleter_role < UserRole::Admin {
            return Err(AppError::Forbidden(
                "Only admins can manage break windows".to_string(),
            ));
        }

        // Verify policy exists and belongs to org
        let _policy = self.break_repo.find_policy_by_id(org_id, policy_id).await?;

        // Verify window belongs to this policy
        let window = self.break_repo.find_window_by_id(window_id).await?;
        if window.break_policy_id != policy_id {
            return Err(AppError::NotFound(
                "Window not found for this policy".to_string(),
            ));
        }

        self.break_repo.delete_window(window_id).await
    }

    /// Get windows for a policy
    pub async fn get_windows(
        &self,
        org_id: Uuid,
        policy_id: Uuid,
    ) -> Result<Vec<BreakWindowResponse>, AppError> {
        // Verify policy exists
        let _policy = self.break_repo.find_policy_by_id(org_id, policy_id).await?;

        let windows = self.break_repo.get_windows_for_policy(policy_id).await?;

        Ok(windows
            .into_iter()
            .map(|w| BreakWindowResponse {
                id: w.id,
                day_of_week: w.day_of_week,
                window_start: w.window_start.format("%H:%M").to_string(),
                window_end: w.window_end.format("%H:%M").to_string(),
                min_duration_minutes: w.min_duration_minutes,
                max_duration_minutes: w.max_duration_minutes,
                is_mandatory: w.is_mandatory,
            })
            .collect())
    }

    // =====================
    // Break Entry Management (for explicit tracking)
    // =====================

    /// Start a break (for explicit tracking mode)
    pub async fn start_break(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        clock_entry_id: Uuid,
        request: StartBreakRequest,
    ) -> Result<BreakEntryResponse, AppError> {
        // Check if user's policy allows explicit tracking
        let effective = self
            .break_repo
            .get_effective_policy(org_id, user_id)
            .await?;

        match &effective {
            Some((policy, _)) if policy.tracking_mode == BreakTrackingMode::ExplicitTracking => {}
            Some((policy, _)) => {
                return Err(AppError::ValidationError(format!(
                    "Your break policy '{}' uses automatic deduction. You don't need to track breaks manually.",
                    policy.name
                )));
            }
            None => {
                return Err(AppError::ValidationError(
                    "No break policy is configured for you. Contact your administrator."
                        .to_string(),
                ));
            }
        }

        // Check if user already has an active break
        if let Some(active_break) = self
            .break_repo
            .get_active_break_for_user(org_id, user_id)
            .await?
        {
            return Err(AppError::Conflict(format!(
                "You already have an active break that started at {}. Please end it before starting a new one.",
                active_break.break_start.format("%H:%M")
            )));
        }

        let new_entry = NewBreakEntry {
            organization_id: org_id,
            user_id,
            clock_entry_id,
            break_start: Utc::now(),
            notes: request.notes,
        };

        let entry = self.break_repo.create_entry(new_entry).await?;
        self.build_entry_response(&entry).await
    }

    /// End a break (for explicit tracking mode)
    pub async fn end_break(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        request: EndBreakRequest,
    ) -> Result<BreakEntryResponse, AppError> {
        // Get active break
        let active_break = self
            .break_repo
            .get_active_break_for_user(org_id, user_id)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("You don't have an active break to end".to_string())
            })?;

        // Calculate duration
        let now = Utc::now();
        let duration = now.signed_duration_since(active_break.break_start);
        let duration_minutes = (duration.num_seconds() / 60) as i32;

        let update = BreakEntryUpdate {
            break_end: Some(now),
            duration_minutes: Some(duration_minutes),
            notes: request.notes.map(Some),
        };

        let entry = self
            .break_repo
            .update_entry(org_id, active_break.id, update)
            .await?;

        self.build_entry_response(&entry).await
    }

    /// Get current break status for a user
    pub async fn get_break_status(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<BreakStatus, AppError> {
        // Get active break
        let active_break = self
            .break_repo
            .get_active_break_for_user(org_id, user_id)
            .await?;

        // Calculate elapsed time if on break
        let (is_on_break, current_break, elapsed_minutes) = if let Some(ref entry) = active_break {
            let elapsed = Utc::now().signed_duration_since(entry.break_start);
            (
                true,
                Some(self.build_entry_response(entry).await?),
                Some((elapsed.num_seconds() / 60) as i32),
            )
        } else {
            (false, None, None)
        };

        // Get effective policy
        let effective_policy = self.get_effective_policy(org_id, user_id).await?;

        Ok(BreakStatus {
            is_on_break,
            current_break,
            elapsed_minutes,
            policy: effective_policy,
        })
    }

    /// List break entries for user
    pub async fn list_entries(
        &self,
        org_id: Uuid,
        filter: BreakEntryFilter,
        pagination: Pagination,
    ) -> Result<PaginatedBreakEntries, AppError> {
        let (entries, total) = self
            .break_repo
            .list_entries(org_id, &filter, &pagination)
            .await?;

        let mut responses = Vec::with_capacity(entries.len());
        for entry in &entries {
            responses.push(self.build_entry_response(entry).await?);
        }

        Ok(PaginatedBreakEntries {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
        })
    }

    // =====================
    // Break Deduction Calculation
    // =====================

    /// Calculate break deduction for a clock entry
    /// This is used by KPI service to adjust actual worked hours
    pub async fn calculate_break_deduction(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        clock_entry_id: Uuid,
        clock_in: DateTime<Utc>,
        clock_out: Option<DateTime<Utc>>,
    ) -> Result<BreakDeduction, AppError> {
        let effective = self
            .break_repo
            .get_effective_policy(org_id, user_id)
            .await?;

        // If no policy, no deduction
        let Some((policy, _source)) = effective else {
            return Ok(BreakDeduction {
                total_minutes: 0,
                source: "none".to_string(),
                entries: vec![],
            });
        };

        match policy.tracking_mode {
            BreakTrackingMode::AutoDeduct => {
                // Get the break window for the day
                let day_of_week = clock_in.weekday().num_days_from_sunday() as i16;
                let window = self
                    .break_repo
                    .get_window_for_day(policy.id, day_of_week)
                    .await?;

                let total_minutes = if let Some(w) = window {
                    // Check if clock period overlaps with break window
                    let clock_out_time = clock_out.unwrap_or_else(Utc::now);
                    let clock_in_time = clock_in.time();
                    let clock_out_naive = clock_out_time.time();

                    // Simple overlap check: if clocked during break window, deduct mandatory minimum
                    if clock_in_time <= w.window_end && clock_out_naive >= w.window_start {
                        w.min_duration_minutes
                    } else {
                        0
                    }
                } else {
                    0
                };

                Ok(BreakDeduction {
                    total_minutes,
                    source: "auto_deduct".to_string(),
                    entries: vec![],
                })
            }
            BreakTrackingMode::ExplicitTracking => {
                // Sum up actual tracked break entries
                let entries = self
                    .break_repo
                    .get_entries_for_clock_entry(clock_entry_id)
                    .await?;

                let total_minutes: i32 = entries.iter().filter_map(|e| e.duration_minutes).sum();

                let mut entry_responses = Vec::with_capacity(entries.len());
                for entry in &entries {
                    entry_responses.push(self.build_entry_response(entry).await?);
                }

                Ok(BreakDeduction {
                    total_minutes,
                    source: "tracked".to_string(),
                    entries: entry_responses,
                })
            }
        }
    }

    /// Get effective break policy for a user
    pub async fn get_effective_policy(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<EffectiveBreakPolicy>, AppError> {
        let effective = self
            .break_repo
            .get_effective_policy(org_id, user_id)
            .await?;

        match effective {
            Some((policy, source_level)) => {
                let response = self.build_policy_response(&policy).await?;
                Ok(Some(EffectiveBreakPolicy {
                    policy: Some(response),
                    source_level,
                }))
            }
            None => Ok(Some(EffectiveBreakPolicy {
                policy: None,
                source_level: "default".to_string(),
            })),
        }
    }

    // =====================
    // Helper Methods
    // =====================

    async fn build_policy_response(
        &self,
        policy: &BreakPolicy,
    ) -> Result<BreakPolicyResponse, AppError> {
        let org = self.org_repo.find_by_id(policy.organization_id).await?;

        let team_name = if let Some(team_id) = policy.team_id {
            let team = self
                .team_repo
                .find_by_id(policy.organization_id, team_id)
                .await?;
            Some(team.name)
        } else {
            None
        };

        let user_name = if let Some(user_id) = policy.user_id {
            let user = self.user_repo.find_by_id(user_id).await?;
            Some(format!("{} {}", user.first_name, user.last_name))
        } else {
            None
        };

        // Get windows for the policy
        let windows = self
            .break_repo
            .get_windows_for_policy(policy.id)
            .await?
            .into_iter()
            .map(|w| BreakWindowResponse {
                id: w.id,
                day_of_week: w.day_of_week,
                window_start: w.window_start.format("%H:%M").to_string(),
                window_end: w.window_end.format("%H:%M").to_string(),
                min_duration_minutes: w.min_duration_minutes,
                max_duration_minutes: w.max_duration_minutes,
                is_mandatory: w.is_mandatory,
            })
            .collect();

        Ok(BreakPolicyResponse {
            id: policy.id,
            organization_id: policy.organization_id,
            organization_name: org.name,
            team_id: policy.team_id,
            team_name,
            user_id: policy.user_id,
            user_name,
            name: policy.name.clone(),
            description: policy.description.clone(),
            tracking_mode: policy.tracking_mode,
            notify_missing_break: policy.notify_missing_break,
            is_active: policy.is_active,
            windows,
            created_at: policy.created_at,
            updated_at: policy.updated_at,
        })
    }

    async fn build_entry_response(
        &self,
        entry: &BreakEntry,
    ) -> Result<BreakEntryResponse, AppError> {
        let user = self.user_repo.find_by_id(entry.user_id).await?;
        let user_name = format!("{} {}", user.first_name, user.last_name);

        Ok(BreakEntryResponse {
            id: entry.id,
            organization_id: entry.organization_id,
            user_id: entry.user_id,
            user_name,
            clock_entry_id: entry.clock_entry_id,
            break_start: entry.break_start,
            break_end: entry.break_end,
            duration_minutes: entry.duration_minutes,
            notes: entry.notes.clone(),
            created_at: entry.created_at,
        })
    }
}
