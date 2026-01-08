// Database models for Time Manager application
// This module contains Diesel models for database entities

pub mod claims;
pub mod invite_token;
pub mod login_attempt;
pub mod password_history;
pub mod password_reset_token;
pub mod refresh_token;
pub mod token_pair;
pub mod user;
pub mod user_session;

// Re-export commonly used types
pub use claims::Claims;
pub use invite_token::{InviteToken, NewInviteToken};
pub use login_attempt::{LoginAttempt, NewLoginAttempt};
pub use password_history::{NewPasswordHistory, PasswordHistory};
pub use password_reset_token::{NewPasswordResetToken, PasswordResetToken};
pub use refresh_token::{NewRefreshToken, RefreshToken};
pub use token_pair::TokenPair;
pub use user::{NewUser, PaginatedUsers, Pagination, UserFilter, UserResponse, UserUpdate};
pub use user_session::{NewUserSession, UserSession};
