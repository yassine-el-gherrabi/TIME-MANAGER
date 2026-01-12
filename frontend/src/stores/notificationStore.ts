/**
 * Notification Store (Zustand)
 *
 * Global state management for notifications.
 */

import { create } from 'zustand';
import { notificationsApi } from '../api/notifications';
import type { Notification, NotificationListParams } from '../types/notification';

/**
 * Notification store state and actions
 */
interface NotificationStore {
  // State
  notifications: Notification[];
  total: number;
  page: number;
  totalPages: number;
  unreadCount: number;
  isLoading: boolean;
  isLoadingCount: boolean;
  error: string | null;

  // Actions
  fetchNotifications: (params?: NotificationListParams) => Promise<void>;
  fetchUnreadCount: () => Promise<void>;
  markAsRead: (id: string) => Promise<void>;
  markAllAsRead: () => Promise<void>;
  setPage: (page: number) => void;
  clearError: () => void;
  reset: () => void;
}

const initialState = {
  notifications: [],
  total: 0,
  page: 1,
  totalPages: 0,
  unreadCount: 0,
  isLoading: false,
  isLoadingCount: false,
  error: null,
};

/**
 * Zustand notification store
 */
export const useNotificationStore = create<NotificationStore>()((set, get) => ({
  ...initialState,

  /**
   * Fetch paginated notifications
   */
  fetchNotifications: async (params: NotificationListParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const response = await notificationsApi.list({
        page: params.page ?? get().page,
        per_page: params.per_page ?? 10,
      });
      set({
        notifications: response.data,
        total: response.total,
        page: response.page,
        totalPages: response.total_pages,
        isLoading: false,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch notifications';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Fetch unread notification count
   */
  fetchUnreadCount: async () => {
    set({ isLoadingCount: true });
    try {
      const response = await notificationsApi.getUnreadCount();
      set({ unreadCount: response.count, isLoadingCount: false });
    } catch (error) {
      // Don't throw error for count fetch - it's not critical
      set({ isLoadingCount: false });
    }
  },

  /**
   * Mark a notification as read
   */
  markAsRead: async (id: string) => {
    try {
      await notificationsApi.markAsRead(id);
      // Update local state
      set((state) => ({
        notifications: state.notifications.map((n) =>
          n.id === id ? { ...n, read_at: new Date().toISOString() } : n
        ),
        unreadCount: Math.max(0, state.unreadCount - 1),
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to mark notification as read';
      set({ error: message });
      throw error;
    }
  },

  /**
   * Mark all notifications as read
   */
  markAllAsRead: async () => {
    try {
      await notificationsApi.markAllAsRead();
      // Update local state
      set((state) => ({
        notifications: state.notifications.map((n) => ({
          ...n,
          read_at: n.read_at ?? new Date().toISOString(),
        })),
        unreadCount: 0,
      }));
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to mark all notifications as read';
      set({ error: message });
      throw error;
    }
  },

  /**
   * Set current page (for pagination)
   */
  setPage: (page: number) => {
    set({ page });
  },

  /**
   * Clear error message
   */
  clearError: () => {
    set({ error: null });
  },

  /**
   * Reset store to initial state
   */
  reset: () => {
    set(initialState);
  },
}));

/**
 * Initialize notification store (fetch unread count)
 */
export const initializeNotificationStore = async (): Promise<void> => {
  const store = useNotificationStore.getState();
  await store.fetchUnreadCount();
};
