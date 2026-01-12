//! JSON serialization helper utilities.
//!
//! This module provides helper functions for JSON serialization that properly
//! handle errors instead of panicking, returning AppError for consistent
//! error handling across the application.

use serde::Serialize;

use crate::error::AppError;

/// Serialize a value to a serde_json::Value, returning AppError on failure.
///
/// This function provides a safe alternative to `serde_json::to_value(...).unwrap()`,
/// logging the error and returning an InternalError on serialization failure.
///
/// # Example
/// ```
/// use serde::Serialize;
/// use timemanager_backend::utils::json::to_json_value;
///
/// #[derive(Serialize)]
/// struct User { name: String }
///
/// let user = User { name: "Alice".to_string() };
/// let value = to_json_value(&user).unwrap();
/// assert_eq!(value["name"], "Alice");
/// ```
pub fn to_json_value<T: Serialize>(value: &T) -> Result<serde_json::Value, AppError> {
    serde_json::to_value(value).map_err(|e| {
        tracing::error!(error = %e, "Failed to serialize value to JSON");
        AppError::InternalError
    })
}

/// Serialize a value to a JSON string, returning AppError on failure.
///
/// This function provides a safe alternative to `serde_json::to_string(...).unwrap()`,
/// logging the error and returning an InternalError on serialization failure.
///
/// # Example
/// ```
/// use serde::Serialize;
/// use timemanager_backend::utils::json::to_json_string;
///
/// #[derive(Serialize)]
/// struct User { name: String }
///
/// let user = User { name: "Alice".to_string() };
/// let json = to_json_string(&user).unwrap();
/// assert!(json.contains("Alice"));
/// ```
pub fn to_json_string<T: Serialize>(value: &T) -> Result<String, AppError> {
    serde_json::to_string(value).map_err(|e| {
        tracing::error!(error = %e, "Failed to serialize value to JSON string");
        AppError::InternalError
    })
}

/// Serialize a value to a pretty-printed JSON string, returning AppError on failure.
///
/// Useful for logging or debugging output where readability is important.
pub fn to_json_string_pretty<T: Serialize>(value: &T) -> Result<String, AppError> {
    serde_json::to_string_pretty(value).map_err(|e| {
        tracing::error!(error = %e, "Failed to serialize value to pretty JSON string");
        AppError::InternalError
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        value: i32,
    }

    #[test]
    fn test_to_json_value() {
        let data = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        let result = to_json_value(&data).unwrap();

        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_to_json_string() {
        let data = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        let result = to_json_string(&data).unwrap();

        assert!(result.contains("\"name\":\"test\""));
        assert!(result.contains("\"value\":42"));
    }

    #[test]
    fn test_to_json_string_pretty() {
        let data = TestStruct {
            name: "test".to_string(),
            value: 42,
        };

        let result = to_json_string_pretty(&data).unwrap();

        // Pretty print has newlines
        assert!(result.contains('\n'));
        assert!(result.contains("test"));
    }

    #[test]
    fn test_to_json_value_with_nested() {
        #[derive(Serialize)]
        struct Nested {
            inner: TestStruct,
        }

        let data = Nested {
            inner: TestStruct {
                name: "nested".to_string(),
                value: 100,
            },
        };

        let result = to_json_value(&data).unwrap();

        assert_eq!(result["inner"]["name"], "nested");
        assert_eq!(result["inner"]["value"], 100);
    }
}
