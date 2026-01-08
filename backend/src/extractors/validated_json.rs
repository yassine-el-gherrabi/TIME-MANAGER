use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::AppError;

/// A JSON extractor that automatically validates the payload using the `validator` crate.
///
/// This extractor combines JSON parsing and validation into a single step,
/// eliminating the need to manually call `.validate()` in handlers.
///
/// # Example
///
/// ```ignore
/// use crate::extractors::ValidatedJson;
///
/// #[derive(Debug, Deserialize, Validate)]
/// pub struct CreateUserRequest {
///     #[validate(email)]
///     pub email: String,
///     #[validate(length(min = 8))]
///     pub password: String,
/// }
///
/// pub async fn create_user(
///     ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
/// ) -> Result<impl IntoResponse, AppError> {
///     // payload is already validated here
///     // ...
/// }
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + Send,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        // First, extract the JSON payload
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection: JsonRejection| {
                AppError::ValidationError(format!("Invalid JSON: {}", rejection))
            })?;

        // Then validate the payload
        value
            .validate()
            .map_err(|e| AppError::ValidationError(format!("Validation failed: {}", e)))?;

        Ok(ValidatedJson(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::post,
        Router,
    };
    use serde::Deserialize;
    use tower::ServiceExt;

    #[derive(Debug, Deserialize, Validate)]
    struct TestPayload {
        #[validate(email(message = "Invalid email"))]
        email: String,
        #[validate(length(min = 3, message = "Name too short"))]
        name: String,
    }

    async fn test_handler(
        ValidatedJson(_payload): ValidatedJson<TestPayload>,
    ) -> StatusCode {
        StatusCode::OK
    }

    fn create_test_app() -> Router {
        Router::new().route("/test", post(test_handler))
    }

    #[tokio::test]
    async fn test_valid_payload() {
        let app = create_test_app();

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email": "test@example.com", "name": "John"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let app = create_test_app();

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email": "not-an-email", "name": "John"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_invalid_json() {
        let app = create_test_app();

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"invalid json"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_name_too_short() {
        let app = create_test_app();

        let request = Request::builder()
            .method("POST")
            .uri("/test")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"email": "test@example.com", "name": "Jo"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
