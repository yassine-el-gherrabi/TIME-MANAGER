use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::enums::UserRole;

/// JWT Claims structure for access tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject: user_id
    pub sub: Uuid,
    /// Organization ID
    pub org_id: Uuid,
    /// User role
    pub role: UserRole,
    /// Expiration time (Unix timestamp)
    pub exp: i64,
    /// Issued at (Unix timestamp)
    pub iat: i64,
}

impl Claims {
    pub fn new(user_id: Uuid, org_id: Uuid, role: UserRole, expiry_seconds: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            sub: user_id,
            org_id,
            role,
            iat: now,
            exp: now + expiry_seconds,
        }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.exp
    }
}
