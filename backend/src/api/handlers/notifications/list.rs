use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::config::AppState;
use crate::error::AppError;
use crate::extractors::AuthenticatedUser;
use crate::models::Pagination;
use crate::services::NotificationService;

#[derive(Debug, Deserialize)]
pub struct ListNotificationsQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

/// GET /api/v1/notifications
///
/// List notifications for the authenticated user
pub async fn list_notifications(
    State(state): State<AppState>,
    AuthenticatedUser(claims): AuthenticatedUser,
    Query(query): Query<ListNotificationsQuery>,
) -> Result<impl IntoResponse, AppError> {
    let service = NotificationService::new(state.db_pool.clone());

    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    };

    let notifications = service
        .get_user_notifications(claims.org_id, claims.sub, pagination)
        .await?;

    Ok((StatusCode::OK, Json(notifications)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_notifications_query_defaults() {
        let query = ListNotificationsQuery {
            page: None,
            per_page: None,
        };
        assert_eq!(query.page.unwrap_or(1), 1);
        assert_eq!(query.per_page.unwrap_or(20), 20);
    }

    #[test]
    fn test_list_notifications_query_custom_values() {
        let query = ListNotificationsQuery {
            page: Some(5),
            per_page: Some(50),
        };
        assert_eq!(query.page.unwrap_or(1), 5);
        assert_eq!(query.per_page.unwrap_or(20), 50);
    }

    #[test]
    fn test_pagination_structure() {
        let pagination = Pagination {
            page: 3,
            per_page: 25,
        };
        assert_eq!(pagination.page, 3);
        assert_eq!(pagination.per_page, 25);
    }
}
