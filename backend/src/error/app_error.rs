use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Database pool error: {0}")]
    PoolError(#[from] diesel::r2d2::PoolError),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Too many requests: {0}")]
    TooManyRequests(String),

    #[error("Internal server error")]
    InternalError,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message, details) = match self {
            AppError::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DatabaseError",
                "A database error occurred",
                Some(err.to_string()),
            ),
            AppError::PoolError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "PoolError",
                "Database connection pool error",
                Some(err.to_string()),
            ),
            AppError::ConfigError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "ConfigError",
                "Configuration error",
                Some(msg),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "ValidationError",
                "Validation failed",
                Some(msg),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NotFound",
                "Resource not found",
                Some(msg),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "Authentication required",
                Some(msg),
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                "Forbidden",
                "Access denied",
                Some(msg),
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "Conflict",
                "Resource conflict",
                Some(msg),
            ),
            AppError::TooManyRequests(msg) => (
                StatusCode::TOO_MANY_REQUESTS,
                "TooManyRequests",
                "Too many requests",
                Some(msg),
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalError",
                "An internal error occurred",
                None,
            ),
        };

        // Log error for debugging
        tracing::error!(
            error_type = error_type,
            message = message,
            details = ?details,
            "Request error"
        );

        let body = Json(ErrorResponse {
            error: error_type.to_string(),
            message: message.to_string(),
            details,
        });

        (status, body).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_response_serialization() {
        let error = AppError::ValidationError("Invalid email".to_string());
        let response = error.into_response();

        // Verify status code
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
