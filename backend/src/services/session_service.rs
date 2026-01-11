use uuid::Uuid;

use crate::config::database::DbPool;
use crate::error::AppError;
use crate::models::{NewUserSession, UserSession};
use crate::repositories::user_session_repository::UserSessionRepository;

/// User session management service
pub struct SessionService {
    session_repo: UserSessionRepository,
}

impl SessionService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            session_repo: UserSessionRepository::new(pool),
        }
    }

    /// Create a new session for user
    pub async fn create_session(
        &self,
        user_id: Uuid,
        refresh_token_id: Uuid,
        user_agent: Option<String>,
        expires_at: chrono::NaiveDateTime,
    ) -> Result<UserSession, AppError> {
        let new_session = NewUserSession {
            user_id,
            refresh_token_id,
            user_agent,
            expires_at,
        };

        self.session_repo.create(new_session).await
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<UserSession>, AppError> {
        self.session_repo
            .get_active_sessions_for_user(user_id)
            .await
    }

    /// Update session last activity timestamp
    pub async fn update_session_activity(&self, session_id: Uuid) -> Result<(), AppError> {
        self.session_repo.update_last_activity(session_id).await
    }

    /// Revoke a specific session
    pub async fn revoke_session(&self, session_id: Uuid) -> Result<(), AppError> {
        self.session_repo.delete(session_id).await
    }

    /// Revoke all sessions for a user
    pub async fn revoke_all_user_sessions(&self, user_id: Uuid) -> Result<(), AppError> {
        self.session_repo.delete_all_for_user(user_id).await
    }

    /// Delete session by refresh token ID
    pub async fn delete_session_by_token(&self, refresh_token_id: Uuid) -> Result<(), AppError> {
        self.session_repo
            .delete_by_refresh_token(refresh_token_id)
            .await
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<usize, AppError> {
        self.session_repo.delete_expired().await
    }

    /// Get session count for user
    pub async fn get_session_count(&self, user_id: Uuid) -> Result<i64, AppError> {
        self.session_repo.count_active_sessions(user_id).await
    }

    /// Check if user has too many active sessions (max 5)
    pub async fn check_session_limit(&self, user_id: Uuid) -> Result<(), AppError> {
        let count = self.get_session_count(user_id).await?;
        const MAX_SESSIONS: i64 = 5;

        if count >= MAX_SESSIONS {
            Err(AppError::Conflict(format!(
                "Maximum number of active sessions ({}) reached",
                MAX_SESSIONS
            )))
        } else {
            Ok(())
        }
    }

    /// Revoke oldest session if limit is reached
    pub async fn revoke_oldest_if_limit_reached(&self, user_id: Uuid) -> Result<(), AppError> {
        let sessions = self
            .session_repo
            .get_active_sessions_for_user(user_id)
            .await?;
        const MAX_SESSIONS: usize = 5;

        if sessions.len() >= MAX_SESSIONS {
            // Find oldest session by created_at
            if let Some(oldest) = sessions.iter().min_by_key(|s| s.created_at) {
                self.session_repo.delete(oldest.id).await?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Compile-time verification that max sessions constant is reasonable
    const _: () = {
        const MAX_SESSIONS: usize = 5;
        assert!(MAX_SESSIONS > 0);
        assert!(MAX_SESSIONS <= 10);
    };
}
