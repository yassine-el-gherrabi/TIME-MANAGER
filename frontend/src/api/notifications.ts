/**
 * Notifications API Client
 *
 * API methods for notification operations.
 */

import { apiRequest } from './client';
import { NOTIFICATION_ENDPOINTS } from '../config/constants';
import type {
  Notification,
  PaginatedNotifications,
  UnreadCountResponse,
  MarkAllReadResponse,
  NotificationListParams,
} from '../types/notification';

/**
 * Notifications API methods
 */
export const notificationsApi = {
  /**
   * Get paginated notifications for the current user
   *
   * @param params - Query parameters for pagination
   * @returns Paginated notifications
   */
  list: async (params: NotificationListParams = {}): Promise<PaginatedNotifications> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }

    const queryString = queryParams.toString();
    const url = queryString
      ? `${NOTIFICATION_ENDPOINTS.LIST}?${queryString}`
      : NOTIFICATION_ENDPOINTS.LIST;

    return apiRequest<PaginatedNotifications>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get unread notification count
   *
   * @returns Unread count
   */
  getUnreadCount: async (): Promise<UnreadCountResponse> => {
    return apiRequest<UnreadCountResponse>({
      method: 'GET',
      url: NOTIFICATION_ENDPOINTS.UNREAD_COUNT,
    });
  },

  /**
   * Mark a notification as read
   *
   * @param id - Notification ID
   * @returns Updated notification
   */
  markAsRead: async (id: string): Promise<Notification> => {
    return apiRequest<Notification>({
      method: 'PUT',
      url: NOTIFICATION_ENDPOINTS.MARK_READ(id),
    });
  },

  /**
   * Mark all notifications as read
   *
   * @returns Number of notifications marked as read
   */
  markAllAsRead: async (): Promise<MarkAllReadResponse> => {
    return apiRequest<MarkAllReadResponse>({
      method: 'PUT',
      url: NOTIFICATION_ENDPOINTS.MARK_ALL_READ,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const { list, getUnreadCount, markAsRead, markAllAsRead } = notificationsApi;
