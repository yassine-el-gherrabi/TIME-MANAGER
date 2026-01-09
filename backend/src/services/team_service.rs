use chrono::Utc;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    NewTeam, Pagination, Team, TeamFilter, TeamMember, TeamResponse, TeamUpdate, TeamWithMembers,
};
use crate::models::team::TeamMemberInfo;
use crate::repositories::TeamRepository;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Request to create a team
#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
    pub work_schedule_id: Option<Uuid>,
}

/// Request to update a team
#[derive(Debug, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub manager_id: Option<Option<Uuid>>,
    pub work_schedule_id: Option<Option<Uuid>>,
}

/// Service for team operations
pub struct TeamService {
    team_repo: TeamRepository,
}

impl TeamService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            team_repo: TeamRepository::new(pool),
        }
    }

    /// Create a new team
    pub async fn create_team(
        &self,
        org_id: Uuid,
        request: CreateTeamRequest,
    ) -> Result<Team, AppError> {
        let new_team = NewTeam {
            organization_id: org_id,
            name: request.name,
            description: request.description,
            manager_id: request.manager_id,
            work_schedule_id: request.work_schedule_id,
        };

        self.team_repo.create(new_team).await
    }

    /// Get a team by ID with member count
    pub async fn get_team(&self, org_id: Uuid, team_id: Uuid) -> Result<TeamResponse, AppError> {
        let team = self.team_repo.find_by_id(org_id, team_id).await?;
        let member_count = self.team_repo.get_member_count(team_id).await?;

        Ok(TeamResponse {
            id: team.id,
            organization_id: team.organization_id,
            name: team.name,
            description: team.description,
            manager_id: team.manager_id,
            work_schedule_id: team.work_schedule_id,
            member_count,
            created_at: team.created_at,
            updated_at: team.updated_at,
        })
    }

    /// Get a team with its members
    pub async fn get_team_with_members(
        &self,
        org_id: Uuid,
        team_id: Uuid,
    ) -> Result<TeamWithMembers, AppError> {
        let team = self.team_repo.find_by_id(org_id, team_id).await?;
        let users = self.team_repo.list_members(team_id).await?;

        let members = users
            .iter()
            .map(|u| TeamMemberInfo {
                user_id: u.id,
                email: u.email.clone(),
                first_name: u.first_name.clone(),
                last_name: u.last_name.clone(),
                joined_at: Utc::now(), // TODO: get actual joined_at from team_members
            })
            .collect();

        Ok(TeamWithMembers { team, members })
    }

    /// List teams with pagination
    pub async fn list_teams(
        &self,
        org_id: Uuid,
        filter: TeamFilter,
        pagination: Pagination,
    ) -> Result<(Vec<TeamResponse>, i64), AppError> {
        let (teams, total) = self.team_repo.list(org_id, &filter, &pagination).await?;

        let mut responses = Vec::with_capacity(teams.len());
        for team in teams {
            let member_count = self.team_repo.get_member_count(team.id).await?;
            responses.push(TeamResponse {
                id: team.id,
                organization_id: team.organization_id,
                name: team.name,
                description: team.description,
                manager_id: team.manager_id,
                work_schedule_id: team.work_schedule_id,
                member_count,
                created_at: team.created_at,
                updated_at: team.updated_at,
            });
        }

        Ok((responses, total))
    }

    /// Update a team
    pub async fn update_team(
        &self,
        org_id: Uuid,
        team_id: Uuid,
        request: UpdateTeamRequest,
    ) -> Result<Team, AppError> {
        let update = TeamUpdate {
            name: request.name,
            description: request.description,
            manager_id: request.manager_id,
            work_schedule_id: request.work_schedule_id,
            updated_at: Some(Utc::now()),
        };

        self.team_repo.update(org_id, team_id, update).await
    }

    /// Delete a team
    pub async fn delete_team(&self, org_id: Uuid, team_id: Uuid) -> Result<(), AppError> {
        self.team_repo.delete(org_id, team_id).await
    }

    /// Add a member to a team
    pub async fn add_member(
        &self,
        org_id: Uuid,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<TeamMember, AppError> {
        // Verify team belongs to organization
        let _ = self.team_repo.find_by_id(org_id, team_id).await?;

        self.team_repo.add_member(team_id, user_id).await
    }

    /// Remove a member from a team
    pub async fn remove_member(
        &self,
        org_id: Uuid,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        // Verify team belongs to organization
        let _ = self.team_repo.find_by_id(org_id, team_id).await?;

        self.team_repo.remove_member(team_id, user_id).await
    }

    /// Get teams for a specific user
    pub async fn get_user_teams(&self, org_id: Uuid, user_id: Uuid) -> Result<Vec<Team>, AppError> {
        self.team_repo.get_user_teams(org_id, user_id).await
    }

    /// Get teams managed by a user
    pub async fn get_managed_teams(
        &self,
        org_id: Uuid,
        manager_id: Uuid,
    ) -> Result<Vec<Team>, AppError> {
        self.team_repo.get_managed_teams(org_id, manager_id).await
    }
}
