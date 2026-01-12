use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::invite_tokens;

/// Invite token for user invitation workflow
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = invite_tokens)]
pub struct InviteToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
    pub used_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

/// New invite token for insertion
#[derive(Debug, Insertable)]
#[diesel(table_name = invite_tokens)]
pub struct NewInviteToken {
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: NaiveDateTime,
}

impl InviteToken {
    /// Check if the token is valid (not used and not expired)
    pub fn is_valid(&self) -> bool {
        self.used_at.is_none() && self.expires_at > chrono::Utc::now().naive_utc()
    }
}
