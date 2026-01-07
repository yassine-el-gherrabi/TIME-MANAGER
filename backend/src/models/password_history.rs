use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::password_history;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = password_history)]
pub struct PasswordHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = password_history)]
pub struct NewPasswordHistory {
    pub user_id: Uuid,
    pub password_hash: String,
}
