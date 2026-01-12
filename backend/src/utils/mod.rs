// Utility functions for authentication, security, and common operations
// This module contains helper functions and utilities

pub mod datetime;
pub mod json;
pub mod jwt;
pub mod password;

// Re-export commonly used types
pub use jwt::JwtService;
pub use password::PasswordService;

// Re-export datetime helpers for convenience
pub use datetime::{
    end_of_day, end_of_day_naive, end_of_day_time, end_of_day_tz, end_of_year, midnight,
    start_of_day, start_of_day_naive, start_of_day_tz, start_of_year,
};

// Re-export JSON helpers for convenience
pub use json::{to_json_string, to_json_string_pretty, to_json_value};
