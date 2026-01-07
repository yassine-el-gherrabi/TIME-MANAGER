// Authentication endpoints
// This module contains handlers for user authentication (register, login, logout, etc.)

pub mod login;
pub mod logout;
pub mod logout_all;
pub mod me;
pub mod refresh;
pub mod register;

// Re-export handler functions
pub use login::login;
pub use logout::logout;
pub use logout_all::logout_all;
pub use me::me;
pub use refresh::refresh;
pub use register::register;
