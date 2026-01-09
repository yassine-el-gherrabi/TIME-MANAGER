use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{NewNotification, Notification, NotificationUpdate, Pagination};
use crate::schema::notifications;

type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Notification repository for database operations
pub struct NotificationRepository {
    pool: DbPool,
}

impl NotificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new notification
    pub async fn create(&self, new_notification: NewNotification) -> Result<Notification, AppError> {
        let mut conn = self.pool.get()?;

        diesel::insert_into(notifications::table)
            .values(&new_notification)
            .get_result(&mut conn)
            .map_err(AppError::DatabaseError)
    }

    /// Find notification by ID within organization
    pub async fn find_by_id(
        &self,
        org_id: Uuid,
        notification_id: Uuid,
    ) -> Result<Notification, AppError> {
        let mut conn = self.pool.get()?;

        notifications::table
            .filter(notifications::organization_id.eq(org_id))
            .find(notification_id)
            .first::<Notification>(&mut conn)
            .map_err(|_| AppError::NotFound("Notification not found".to_string()))
    }

    /// List notifications for a user with pagination
    pub async fn list_by_user(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> Result<(Vec<Notification>, i64), AppError> {
        let mut conn = self.pool.get()?;

        // Count total
        let total: i64 = notifications::table
            .filter(notifications::organization_id.eq(org_id))
            .filter(notifications::user_id.eq(user_id))
            .count()
            .get_result(&mut conn)?;

        // Apply pagination
        let offset = (pagination.page - 1) * pagination.per_page;
        let results = notifications::table
            .filter(notifications::organization_id.eq(org_id))
            .filter(notifications::user_id.eq(user_id))
            .order(notifications::created_at.desc())
            .limit(pagination.per_page)
            .offset(offset)
            .load::<Notification>(&mut conn)?;

        Ok((results, total))
    }

    /// Count unread notifications for a user
    pub async fn count_unread(&self, org_id: Uuid, user_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self.pool.get()?;

        let count: i64 = notifications::table
            .filter(notifications::organization_id.eq(org_id))
            .filter(notifications::user_id.eq(user_id))
            .filter(notifications::read_at.is_null())
            .count()
            .get_result(&mut conn)?;

        Ok(count)
    }

    /// Mark a notification as read
    pub async fn mark_as_read(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        notification_id: Uuid,
    ) -> Result<Notification, AppError> {
        let mut conn = self.pool.get()?;

        let update = NotificationUpdate {
            read_at: Some(Some(Utc::now())),
        };

        let affected = diesel::update(
            notifications::table
                .filter(notifications::organization_id.eq(org_id))
                .filter(notifications::user_id.eq(user_id))
                .filter(notifications::id.eq(notification_id)),
        )
        .set(&update)
        .execute(&mut conn)?;

        if affected == 0 {
            return Err(AppError::NotFound("Notification not found".to_string()));
        }

        self.find_by_id(org_id, notification_id).await
    }

    /// Mark all notifications as read for a user
    pub async fn mark_all_as_read(&self, org_id: Uuid, user_id: Uuid) -> Result<i64, AppError> {
        let mut conn = self.pool.get()?;

        let update = NotificationUpdate {
            read_at: Some(Some(Utc::now())),
        };

        let affected = diesel::update(
            notifications::table
                .filter(notifications::organization_id.eq(org_id))
                .filter(notifications::user_id.eq(user_id))
                .filter(notifications::read_at.is_null()),
        )
        .set(&update)
        .execute(&mut conn)?;

        Ok(affected as i64)
    }

    /// Delete a notification
    pub async fn delete(&self, org_id: Uuid, notification_id: Uuid) -> Result<(), AppError> {
        let mut conn = self.pool.get()?;

        let deleted = diesel::delete(
            notifications::table
                .filter(notifications::organization_id.eq(org_id))
                .filter(notifications::id.eq(notification_id)),
        )
        .execute(&mut conn)?;

        if deleted == 0 {
            return Err(AppError::NotFound("Notification not found".to_string()));
        }

        Ok(())
    }
}
