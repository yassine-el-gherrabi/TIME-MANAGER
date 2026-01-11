use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{Datelike, NaiveDate, Utc, Weekday};
use serde::Deserialize;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::{AbsenceStatus, NotificationType, UserRole};
use crate::error::AppError;
use crate::models::{
    Absence, AbsenceFilter, AbsenceResponse, AbsenceUpdate, NewAbsence, PaginatedAbsences,
    Pagination, PendingAbsenceFilter,
};
use crate::repositories::{
    AbsenceRepository, AbsenceTypeRepository, ClosedDayRepository, LeaveBalanceRepository,
    OrganizationRepository, TeamRepository,
};
use crate::services::NotificationService;

/// Request to create an absence
#[derive(Debug, Deserialize)]
pub struct CreateAbsenceRequest {
    pub type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
}

/// Service for absence operations
pub struct AbsenceService {
    absence_repo: AbsenceRepository,
    absence_type_repo: AbsenceTypeRepository,
    leave_balance_repo: LeaveBalanceRepository,
    closed_day_repo: ClosedDayRepository,
    team_repo: TeamRepository,
    org_repo: OrganizationRepository,
}

impl AbsenceService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            absence_repo: AbsenceRepository::new(pool.clone()),
            absence_type_repo: AbsenceTypeRepository::new(pool.clone()),
            leave_balance_repo: LeaveBalanceRepository::new(pool.clone()),
            closed_day_repo: ClosedDayRepository::new(pool.clone()),
            team_repo: TeamRepository::new(pool.clone()),
            org_repo: OrganizationRepository::new(pool),
        }
    }

    /// Create an absence request
    pub async fn create_request(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        request: CreateAbsenceRequest,
    ) -> Result<AbsenceResponse, AppError> {
        // Validate dates
        if request.end_date < request.start_date {
            return Err(AppError::ValidationError(
                "End date must be on or after start date".to_string(),
            ));
        }

        // Don't allow requests in the past (except for same-day)
        let today = Utc::now().date_naive();
        if request.start_date < today {
            return Err(AppError::ValidationError(
                "Cannot request absence for past dates".to_string(),
            ));
        }

        // Get absence type
        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, request.type_id)
            .await?;

        // Check for overlapping absences
        let has_overlap = self
            .absence_repo
            .check_overlap(org_id, user_id, request.start_date, request.end_date, None)
            .await?;

        if has_overlap {
            return Err(AppError::Conflict(
                "You already have an absence request for these dates".to_string(),
            ));
        }

        // Calculate working days
        let days_count = self
            .calculate_working_days(org_id, request.start_date, request.end_date)
            .await?;

        if days_count <= 0.0 {
            return Err(AppError::ValidationError(
                "No working days in the selected period".to_string(),
            ));
        }

        // Check balance if affects_balance
        if absence_type.affects_balance {
            let year = request.start_date.year();
            let balance = self
                .leave_balance_repo
                .find_by_user_type_year(org_id, user_id, request.type_id, year)
                .await?;

            if let Some(b) = balance {
                let initial = b.initial_balance.to_f64().unwrap_or(0.0);
                let used = b.used.to_f64().unwrap_or(0.0);
                let adj = b.adjustment.to_f64().unwrap_or(0.0);
                let remaining = initial - used + adj;

                if days_count > remaining {
                    return Err(AppError::ValidationError(format!(
                        "Insufficient balance. Remaining: {} days, Requested: {} days",
                        remaining, days_count
                    )));
                }
            } else {
                return Err(AppError::ValidationError(
                    "No leave balance set for this absence type".to_string(),
                ));
            }
        }

        // Determine initial status
        let status = if absence_type.requires_approval {
            AbsenceStatus::Pending
        } else {
            AbsenceStatus::Approved
        };

        // Create absence
        let new_absence = NewAbsence {
            organization_id: org_id,
            user_id,
            type_id: request.type_id,
            start_date: request.start_date,
            end_date: request.end_date,
            days_count: BigDecimal::try_from(days_count).unwrap_or_default(),
            status,
            reason: request.reason,
        };

        let absence = self.absence_repo.create(new_absence).await?;

        // If auto-approved, update balance immediately
        if status == AbsenceStatus::Approved && absence_type.affects_balance {
            let year = request.start_date.year();
            self.leave_balance_repo
                .increment_used(
                    org_id,
                    user_id,
                    request.type_id,
                    year,
                    BigDecimal::try_from(days_count).unwrap_or_default(),
                )
                .await?;
        }

        self.build_response(&absence).await
    }

    /// Approve an absence request
    pub async fn approve(
        &self,
        org_id: Uuid,
        absence_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
    ) -> Result<AbsenceResponse, AppError> {
        // Check permission
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers and admins can approve absences".to_string(),
            ));
        }

        // Get the absence
        let absence = self.absence_repo.find_by_id(org_id, absence_id).await?;

        if absence.status != AbsenceStatus::Pending {
            return Err(AppError::ValidationError(
                "Only pending absences can be approved".to_string(),
            ));
        }

        // For managers, verify they manage a team the user belongs to
        if approver_role == UserRole::Manager {
            self.verify_manager_permission(org_id, approver_id, absence.user_id)
                .await?;
        }

        // Get absence type to check if affects balance
        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, absence.type_id)
            .await?;

        // Update status
        let update = AbsenceUpdate {
            status: Some(AbsenceStatus::Approved),
            approved_by: Some(Some(approver_id)),
            approved_at: Some(Some(Utc::now())),
            ..Default::default()
        };

        let updated = self.absence_repo.update(org_id, absence_id, update).await?;

        // Update balance if affects_balance
        if absence_type.affects_balance {
            let year = absence.start_date.year();
            self.leave_balance_repo
                .increment_used(
                    org_id,
                    absence.user_id,
                    absence.type_id,
                    year,
                    absence.days_count,
                )
                .await?;
        }

        // Create notification for the employee
        let notification_service = NotificationService::new(self.absence_repo.pool().clone());
        let _ = notification_service
            .create_notification(
                org_id,
                absence.user_id,
                NotificationType::AbsenceApproved,
                "Absence Approved".to_string(),
                format!(
                    "Your {} request from {} to {} has been approved.",
                    absence_type.name,
                    absence.start_date.format("%Y-%m-%d"),
                    absence.end_date.format("%Y-%m-%d")
                ),
                None,
            )
            .await;

        self.build_response(&updated).await
    }

    /// Reject an absence request
    pub async fn reject(
        &self,
        org_id: Uuid,
        absence_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
        reason: Option<String>,
    ) -> Result<AbsenceResponse, AppError> {
        // Check permission
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers and admins can reject absences".to_string(),
            ));
        }

        // Get the absence
        let absence = self.absence_repo.find_by_id(org_id, absence_id).await?;

        if absence.status != AbsenceStatus::Pending {
            return Err(AppError::ValidationError(
                "Only pending absences can be rejected".to_string(),
            ));
        }

        // For managers, verify they manage a team the user belongs to
        if approver_role == UserRole::Manager {
            self.verify_manager_permission(org_id, approver_id, absence.user_id)
                .await?;
        }

        // Keep reason for notification before moving
        let reason_text = reason.as_deref().unwrap_or("Not specified").to_string();

        // Update status
        let update = AbsenceUpdate {
            status: Some(AbsenceStatus::Rejected),
            rejection_reason: Some(reason),
            approved_by: Some(Some(approver_id)),
            approved_at: Some(Some(Utc::now())),
            ..Default::default()
        };

        let updated = self.absence_repo.update(org_id, absence_id, update).await?;

        // Create notification for the employee
        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, absence.type_id)
            .await?;
        let notification_service = NotificationService::new(self.absence_repo.pool().clone());
        let _ = notification_service
            .create_notification(
                org_id,
                absence.user_id,
                NotificationType::AbsenceRejected,
                "Absence Rejected".to_string(),
                format!(
                    "Your {} request from {} to {} has been rejected. Reason: {}",
                    absence_type.name,
                    absence.start_date.format("%Y-%m-%d"),
                    absence.end_date.format("%Y-%m-%d"),
                    reason_text
                ),
                None,
            )
            .await;

        self.build_response(&updated).await
    }

    /// Cancel an absence request (owner only)
    pub async fn cancel(
        &self,
        org_id: Uuid,
        absence_id: Uuid,
        user_id: Uuid,
    ) -> Result<AbsenceResponse, AppError> {
        // Get the absence
        let absence = self.absence_repo.find_by_id(org_id, absence_id).await?;

        // Verify ownership
        if absence.user_id != user_id {
            return Err(AppError::Forbidden(
                "You can only cancel your own absences".to_string(),
            ));
        }

        // Can only cancel pending or future approved absences
        let today = Utc::now().date_naive();
        match absence.status {
            AbsenceStatus::Pending => {}
            AbsenceStatus::Approved => {
                if absence.start_date <= today {
                    return Err(AppError::ValidationError(
                        "Cannot cancel an absence that has already started".to_string(),
                    ));
                }
            }
            _ => {
                return Err(AppError::ValidationError(
                    "Can only cancel pending or future approved absences".to_string(),
                ));
            }
        }

        // Get absence type to check if balance should be restored
        let absence_type = self
            .absence_type_repo
            .find_by_id(org_id, absence.type_id)
            .await?;

        // Update status
        let update = AbsenceUpdate {
            status: Some(AbsenceStatus::Cancelled),
            ..Default::default()
        };

        let updated = self.absence_repo.update(org_id, absence_id, update).await?;

        // Restore balance if was approved and affects_balance
        if absence.status == AbsenceStatus::Approved && absence_type.affects_balance {
            let year = absence.start_date.year();
            self.leave_balance_repo
                .decrement_used(
                    org_id,
                    absence.user_id,
                    absence.type_id,
                    year,
                    absence.days_count,
                )
                .await?;
        }

        self.build_response(&updated).await
    }

    /// Get an absence by ID
    pub async fn get(&self, org_id: Uuid, absence_id: Uuid) -> Result<AbsenceResponse, AppError> {
        let absence = self.absence_repo.find_by_id(org_id, absence_id).await?;
        self.build_response(&absence).await
    }

    /// List absences with filters
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: AbsenceFilter,
        pagination: Pagination,
    ) -> Result<PaginatedAbsences, AppError> {
        let (absences, total) = self.absence_repo.list(org_id, &filter, &pagination).await?;

        let mut responses = Vec::with_capacity(absences.len());
        for absence in &absences {
            responses.push(self.build_response(absence).await?);
        }

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedAbsences {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    /// List pending absences for approval
    ///
    /// - SuperAdmin: Can filter by organization_id (defaults to their org), can filter by team_id
    /// - Admin: Uses their org, can filter by team_id
    /// - Manager: Uses their org, filters by managed teams, can further filter by team_id
    pub async fn list_pending(
        &self,
        user_org_id: Uuid,
        approver_id: Uuid,
        approver_role: UserRole,
        filter: PendingAbsenceFilter,
        pagination: Pagination,
    ) -> Result<PaginatedAbsences, AppError> {
        if approver_role == UserRole::Employee {
            return Err(AppError::Forbidden(
                "Only managers and admins can view pending absences".to_string(),
            ));
        }

        // Determine which organization to query
        // SuperAdmin can specify a different org, others use their own
        let org_id = if approver_role == UserRole::SuperAdmin {
            filter.organization_id.unwrap_or(user_org_id)
        } else {
            user_org_id
        };

        // For managers, get team member IDs from managed teams
        let user_ids = if approver_role == UserRole::Manager {
            let managed_teams = self
                .team_repo
                .get_managed_teams(org_id, approver_id)
                .await?;
            let mut member_ids = Vec::new();

            for team in managed_teams {
                // If team_id filter is set, only include that team
                if let Some(filter_team_id) = filter.team_id {
                    if team.id != filter_team_id {
                        continue;
                    }
                }
                let members = self.team_repo.list_members(team.id).await?;
                for member in members {
                    if !member_ids.contains(&member.id) {
                        member_ids.push(member.id);
                    }
                }
            }

            Some(member_ids)
        } else if let Some(filter_team_id) = filter.team_id {
            // Admin or SuperAdmin filtering by team
            let members = self.team_repo.list_members(filter_team_id).await?;
            Some(members.iter().map(|m| m.id).collect())
        } else {
            None
        };

        let (absences, total) = self
            .absence_repo
            .list_pending(org_id, user_ids, &pagination)
            .await?;

        let mut responses = Vec::with_capacity(absences.len());
        for absence in &absences {
            responses.push(self.build_response(absence).await?);
        }

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedAbsences {
            data: responses,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    /// Get absences for calendar view
    pub async fn get_calendar(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
        user_ids: Option<Vec<Uuid>>,
    ) -> Result<Vec<AbsenceResponse>, AppError> {
        let absences = self
            .absence_repo
            .get_for_date_range(org_id, start_date, end_date, user_ids)
            .await?;

        let mut responses = Vec::with_capacity(absences.len());
        for absence in &absences {
            responses.push(self.build_response(absence).await?);
        }

        Ok(responses)
    }

    /// Calculate working days between two dates (excluding weekends and closed days)
    async fn calculate_working_days(
        &self,
        org_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<f64, AppError> {
        // Get closed days in range
        let closed_days = self
            .closed_day_repo
            .list_range(org_id, start_date, end_date)
            .await?;

        let mut working_days = 0.0;
        let mut current = start_date;

        while current <= end_date {
            let weekday = current.weekday();

            // Skip weekends
            if weekday != Weekday::Sat && weekday != Weekday::Sun {
                // Skip closed days
                if !closed_days.contains(&current) {
                    working_days += 1.0;
                }
            }

            current = current.succ_opt().unwrap_or(current);
        }

        Ok(working_days)
    }

    /// Verify manager can manage this user
    async fn verify_manager_permission(
        &self,
        org_id: Uuid,
        manager_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let managed_teams = self.team_repo.get_managed_teams(org_id, manager_id).await?;

        for team in managed_teams {
            if self.team_repo.is_member(team.id, user_id).await? {
                return Ok(());
            }
        }

        Err(AppError::Forbidden(
            "You can only manage absences for members of your team".to_string(),
        ))
    }

    /// Build response with enriched data
    async fn build_response(&self, absence: &Absence) -> Result<AbsenceResponse, AppError> {
        // Get organization info
        let organization = self.org_repo.find_by_id(absence.organization_id).await?;

        // Get user info (simplified - you'd want a user repo method)
        let (user_name, user_email) = self.get_user_info(absence.user_id).await?;

        // Get team info
        let teams = self
            .team_repo
            .get_user_teams(absence.organization_id, absence.user_id)
            .await
            .unwrap_or_default();
        let team_id = teams.first().map(|t| t.id);
        let team_name = teams.first().map(|t| t.name.clone());

        // Get absence type info
        let absence_type = self
            .absence_type_repo
            .find_by_id(absence.organization_id, absence.type_id)
            .await?;

        // Get approver info if present
        let approver_name = if let Some(approver_id) = absence.approved_by {
            let (name, _) = self.get_user_info(approver_id).await?;
            Some(name)
        } else {
            None
        };

        Ok(AbsenceResponse {
            id: absence.id,
            organization_id: absence.organization_id,
            organization_name: organization.name,
            user_id: absence.user_id,
            user_name,
            user_email,
            team_id,
            team_name,
            type_id: absence.type_id,
            type_name: absence_type.name,
            type_code: absence_type.code,
            type_color: absence_type.color.unwrap_or_else(|| "#3B82F6".to_string()),
            start_date: absence.start_date,
            end_date: absence.end_date,
            days_count: absence.days_count.to_f64().unwrap_or(0.0),
            status: absence.status,
            reason: absence.reason.clone(),
            rejection_reason: absence.rejection_reason.clone(),
            approved_by: absence.approved_by,
            approver_name,
            approved_at: absence.approved_at,
            created_at: absence.created_at,
        })
    }

    /// Get user info (placeholder - should use user repo)
    async fn get_user_info(&self, user_id: Uuid) -> Result<(String, String), AppError> {
        use crate::schema::users;
        use diesel::prelude::*;
        use diesel_async::RunQueryDsl;

        let pool = self.absence_repo.pool();
        let mut conn = pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let (first_name, last_name, email): (String, String, String) = users::table
            .filter(users::id.eq(user_id))
            .select((users::first_name, users::last_name, users::email))
            .first(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        Ok((format!("{} {}", first_name, last_name), email))
    }
}
