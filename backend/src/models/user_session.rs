use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user_sessions;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = user_sessions)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_id: Uuid,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
    pub last_activity: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_sessions)]
pub struct NewUserSession {
    pub user_id: Uuid,
    pub refresh_token_id: Uuid,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub expires_at: NaiveDateTime,
}

impl UserSession {
    pub fn is_active(&self) -> bool {
        self.expires_at > chrono::Utc::now().naive_utc()
    }
}
