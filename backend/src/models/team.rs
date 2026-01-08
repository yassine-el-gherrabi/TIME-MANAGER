use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::teams;

/// Team entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Team {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// NewTeam for creating teams
#[derive(Debug, Insertable)]
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
}

/// Team update struct for partial updates
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = teams)]
pub struct TeamUpdate {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub manager_id: Option<Option<Uuid>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Team response with member count
#[derive(Debug, Serialize)]
pub struct TeamResponse {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
    pub member_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Team with members for detailed view
#[derive(Debug, Serialize)]
pub struct TeamWithMembers {
    pub team: Team,
    pub members: Vec<TeamMemberInfo>,
}

/// Team member info for response
#[derive(Debug, Serialize)]
pub struct TeamMemberInfo {
    pub user_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub joined_at: DateTime<Utc>,
}

/// Team filter options
#[derive(Debug, Clone, Default)]
pub struct TeamFilter {
    pub search: Option<String>,
    pub manager_id: Option<Uuid>,
}
