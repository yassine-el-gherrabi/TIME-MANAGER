// Authentication endpoints
// This module contains handlers for user authentication (login, logout, etc.)

pub mod accept_invite;
pub mod bootstrap;
pub mod change_password;
pub mod login;
pub mod logout;
pub mod logout_all;
pub mod me;
pub mod refresh;
pub mod sessions;

// Re-export handler functions
pub use accept_invite::{accept_invite, verify_invite};
pub use bootstrap::bootstrap;
pub use change_password::change_password;
pub use login::login;
pub use logout::logout;
pub use logout_all::logout_all;
pub use me::me;
pub use refresh::refresh;
pub use sessions::{get_sessions, revoke_session};
