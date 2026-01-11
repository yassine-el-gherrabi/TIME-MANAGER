use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{NewTeam, NewTeamMember, Pagination, Team, TeamFilter, TeamMember, TeamUpdate};
use crate::repositories::User;
use crate::schema::{team_members, teams, users};

/// Team repository for database operations
pub struct TeamRepository {
    pool: DbPool,
}

impl TeamRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new team
    pub async fn create(&self, new_team: NewTeam) -> Result<Team, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        diesel::insert_into(teams::table)
            .values(&new_team)
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict("A team with this name already exists".to_string()),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Find team by ID within organization
    pub async fn find_by_id(&self, org_id: Uuid, team_id: Uuid) -> Result<Team, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        teams::table
            .filter(teams::organization_id.eq(org_id))
            .find(team_id)
            .first::<Team>(&mut conn)
            .await
            .map_err(|_| AppError::NotFound("Team not found".to_string()))
    }

    /// List teams with filters and pagination
    pub async fn list(
        &self,
        org_id: Uuid,
        filter: &TeamFilter,
        pagination: &Pagination,
    ) -> Result<(Vec<Team>, i64), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let search_pattern = filter
            .search
            .as_ref()
            .map(|s| format!("%{}%", s.to_lowercase()));

        // Get total count
        let total: i64 = {
            let mut count_query = teams::table
                .filter(teams::organization_id.eq(org_id))
                .into_boxed();

            if let Some(manager_id) = filter.manager_id {
                count_query = count_query.filter(teams::manager_id.eq(manager_id));
            }

            if let Some(ref pattern) = search_pattern {
                count_query = count_query.filter(teams::name.ilike(pattern));
            }

            count_query
                .count()
                .get_result(&mut conn)
                .await
                .map_err(AppError::DatabaseError)?
        };

        // Build data query
        let mut query = teams::table
            .filter(teams::organization_id.eq(org_id))
            .into_boxed();

        if let Some(manager_id) = filter.manager_id {
            query = query.filter(teams::manager_id.eq(manager_id));
        }

        if let Some(ref pattern) = search_pattern {
            query = query.filter(teams::name.ilike(pattern));
        }

        let offset = (pagination.page - 1) * pagination.per_page;
        let results = query
            .order(teams::name.asc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<Team>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok((results, total))
    }

    /// Update a team
    pub async fn update(
        &self,
        org_id: Uuid,
        team_id: Uuid,
        update: TeamUpdate,
    ) -> Result<Team, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let affected = diesel::update(
            teams::table
                .filter(teams::organization_id.eq(org_id))
                .filter(teams::id.eq(team_id)),
        )
        .set(&update)
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if affected == 0 {
            return Err(AppError::NotFound("Team not found".to_string()));
        }

        self.find_by_id(org_id, team_id).await
    }

    /// Delete a team
    pub async fn delete(&self, org_id: Uuid, team_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            teams::table
                .filter(teams::organization_id.eq(org_id))
                .filter(teams::id.eq(team_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Team not found".to_string()));
        }

        Ok(())
    }

    /// Add a member to a team
    pub async fn add_member(&self, team_id: Uuid, user_id: Uuid) -> Result<TeamMember, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let new_member = NewTeamMember { team_id, user_id };

        diesel::insert_into(team_members::table)
            .values(&new_member)
            .get_result(&mut conn)
            .await
            .map_err(|e| match e {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => AppError::Conflict("User is already a member of this team".to_string()),
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation,
                    _,
                ) => AppError::NotFound("Team or user not found".to_string()),
                _ => AppError::DatabaseError(e),
            })
    }

    /// Remove a member from a team
    pub async fn remove_member(&self, team_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let deleted = diesel::delete(
            team_members::table
                .filter(team_members::team_id.eq(team_id))
                .filter(team_members::user_id.eq(user_id)),
        )
        .execute(&mut conn)
        .await
        .map_err(AppError::DatabaseError)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Team member not found".to_string()));
        }

        Ok(())
    }

    /// List all members of a team
    pub async fn list_members(&self, team_id: Uuid) -> Result<Vec<User>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let members = team_members::table
            .filter(team_members::team_id.eq(team_id))
            .inner_join(users::table)
            .select(User::as_select())
            .load::<User>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(members)
    }

    /// List all members of a team with their joined_at timestamp
    pub async fn list_members_with_joined_at(
        &self,
        team_id: Uuid,
    ) -> Result<Vec<(User, chrono::DateTime<chrono::Utc>)>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let members = team_members::table
            .filter(team_members::team_id.eq(team_id))
            .inner_join(users::table)
            .select((User::as_select(), team_members::joined_at))
            .load::<(User, chrono::DateTime<chrono::Utc>)>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(members)
    }

    /// Get all teams for a user
    pub async fn get_user_teams(&self, org_id: Uuid, user_id: Uuid) -> Result<Vec<Team>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let user_teams = team_members::table
            .filter(team_members::user_id.eq(user_id))
            .inner_join(teams::table)
            .filter(teams::organization_id.eq(org_id))
            .select(Team::as_select())
            .load::<Team>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(user_teams)
    }

    /// Get member count for a team
    pub async fn get_member_count(&self, team_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let count = team_members::table
            .filter(team_members::team_id.eq(team_id))
            .count()
            .get_result(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(count)
    }

    /// Check if user is member of team
    pub async fn is_member(&self, team_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let count = team_members::table
            .filter(team_members::team_id.eq(team_id))
            .filter(team_members::user_id.eq(user_id))
            .count()
            .get_result::<i64>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(count > 0)
    }

    /// Get teams managed by a user
    pub async fn get_managed_teams(
        &self,
        org_id: Uuid,
        manager_id: Uuid,
    ) -> Result<Vec<Team>, AppError> {
        let mut conn = self
            .pool
            .get()
            .await
            .map_err(|e| AppError::PoolError(e.to_string()))?;

        let managed = teams::table
            .filter(teams::organization_id.eq(org_id))
            .filter(teams::manager_id.eq(manager_id))
            .load::<Team>(&mut conn)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(managed)
    }
}
