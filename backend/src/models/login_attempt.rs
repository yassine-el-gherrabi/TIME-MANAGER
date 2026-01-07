use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::login_attempts;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = login_attempts)]
pub struct LoginAttempt {
    pub id: Uuid,
    pub email: String,
    pub ip_address: String,
    pub attempted_at: NaiveDateTime,
    pub successful: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = login_attempts)]
pub struct NewLoginAttempt {
    pub email: String,
    pub ip_address: String,
    pub successful: bool,
}
