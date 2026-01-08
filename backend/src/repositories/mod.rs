// Repository layer for database access logic
// Provides data access abstractions for all entities

pub mod invite_token_repository;
pub mod login_attempt_repository;
pub mod password_history_repository;
pub mod password_reset_repository;
pub mod refresh_token_repository;
pub mod user_repository;
pub mod user_session_repository;

// Re-export repository types for convenience
pub use invite_token_repository::InviteTokenRepository;
pub use login_attempt_repository::LoginAttemptRepository;
pub use password_history_repository::PasswordHistoryRepository;
pub use password_reset_repository::PasswordResetRepository;
pub use refresh_token_repository::RefreshTokenRepository;
pub use user_repository::{User, UserRepository};
pub use user_session_repository::UserSessionRepository;
