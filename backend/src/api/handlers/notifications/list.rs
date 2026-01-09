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
