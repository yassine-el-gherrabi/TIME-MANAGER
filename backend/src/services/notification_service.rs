use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::config::database::DbPool;
use crate::domain::enums::NotificationType;
use crate::error::AppError;
use crate::models::{
    NewNotification, Notification, NotificationResponse, PaginatedNotifications, Pagination,
    UnreadCountResponse,
};
use crate::repositories::NotificationRepository;

/// Notification service for business logic
pub struct NotificationService {
    notification_repo: NotificationRepository,
}

impl NotificationService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            notification_repo: NotificationRepository::new(pool),
        }
    }

    /// Create a new notification for a user
    pub async fn create_notification(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        notification_type: NotificationType,
        title: String,
        message: String,
        data: Option<JsonValue>,
    ) -> Result<Notification, AppError> {
        let new_notification = NewNotification {
            organization_id: org_id,
            user_id,
            notification_type,
            title,
            message,
            data,
        };

        self.notification_repo.create(new_notification).await
    }

    /// Get paginated notifications for a user
    pub async fn get_user_notifications(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<PaginatedNotifications, AppError> {
        let (notifications, total) = self
            .notification_repo
            .list_by_user(org_id, user_id, &pagination)
            .await?;

        let data: Vec<NotificationResponse> = notifications.into_iter().map(|n| n.into()).collect();

        let total_pages = (total as f64 / pagination.per_page as f64).ceil() as i64;

        Ok(PaginatedNotifications {
            data,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        })
    }

    /// Get unread notification count for a user
    pub async fn get_unread_count(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<UnreadCountResponse, AppError> {
        let count = self.notification_repo.count_unread(org_id, user_id).await?;
        Ok(UnreadCountResponse { count })
    }

    /// Mark a notification as read
    pub async fn mark_as_read(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        notification_id: Uuid,
    ) -> Result<NotificationResponse, AppError> {
        let notification = self
            .notification_repo
            .mark_as_read(org_id, user_id, notification_id)
            .await?;
        Ok(notification.into())
    }

    /// Mark all notifications as read for a user
    pub async fn mark_all_as_read(&self, org_id: Uuid, user_id: Uuid) -> Result<i64, AppError> {
        self.notification_repo
            .mark_all_as_read(org_id, user_id)
            .await
    }
}
