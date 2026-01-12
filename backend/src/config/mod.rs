pub mod app;
pub mod database;
pub mod email;
pub mod hibp;

// Re-export commonly used types
pub use app::{AppConfig, AppState};
pub use email::EmailConfig;
pub use hibp::HibpConfig;
