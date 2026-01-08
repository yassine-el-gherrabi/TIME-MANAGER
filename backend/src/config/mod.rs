pub mod app;
pub mod database;
pub mod email;

// Re-export commonly used types
pub use app::{AppConfig, AppState};
pub use email::EmailConfig;
