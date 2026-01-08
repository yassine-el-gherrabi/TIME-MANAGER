use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::team_members;

/// TeamMember entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = team_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamMember {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

/// NewTeamMember for adding members to teams
#[derive(Debug, Insertable)]
#[diesel(table_name = team_members)]
pub struct NewTeamMember {
    pub team_id: Uuid,
    pub user_id: Uuid,
}
