use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::domain::enums::NotificationType;
use crate::schema::notifications;

/// Notification entity from database
#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = notifications)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Notification {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    #[diesel(column_name = type_)]
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<JsonValue>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// NewNotification for creating notifications
#[derive(Debug, Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification {
    pub organization_id: Uuid,
    pub user_id: Uuid,
    #[diesel(column_name = type_)]
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<JsonValue>,
}

/// Notification update struct for marking as read
#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = notifications)]
pub struct NotificationUpdate {
    pub read_at: Option<Option<DateTime<Utc>>>,
}

/// Notification response for API
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: Option<JsonValue>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id,
            notification_type: n.notification_type,
            title: n.title,
            message: n.message,
            data: n.data,
            read_at: n.read_at,
            created_at: n.created_at,
        }
    }
}

/// Paginated notifications response
#[derive(Debug, Serialize)]
pub struct PaginatedNotifications {
    pub data: Vec<NotificationResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// Unread count response
#[derive(Debug, Serialize)]
pub struct UnreadCountResponse {
    pub count: i64,
}
