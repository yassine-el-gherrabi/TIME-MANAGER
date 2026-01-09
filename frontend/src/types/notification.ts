/**
 * Notification Types
 *
 * TypeScript type definitions for the notification system.
 */

/**
 * Notification type enum
 */
export enum NotificationType {
  AbsenceApproved = 'absence_approved',
  AbsenceRejected = 'absence_rejected',
  AbsencePending = 'absence_pending',
  ClockCorrection = 'clock_correction',
  ClockApproved = 'clock_approved',
  ClockRejected = 'clock_rejected',
}

/**
 * Notification entity
 */
export interface Notification {
  id: string;
  type: NotificationType;
  title: string;
  message: string;
  data?: Record<string, unknown>;
  read_at: string | null;
  created_at: string;
}

/**
 * Paginated notifications response
 */
export interface PaginatedNotifications {
  data: Notification[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

/**
 * Unread count response
 */
export interface UnreadCountResponse {
  count: number;
}

/**
 * Mark all as read response
 */
export interface MarkAllReadResponse {
  marked_count: number;
}

/**
 * Notification list query params
 */
export interface NotificationListParams {
  page?: number;
  per_page?: number;
}

/**
 * Notification type labels (French)
 */
export const NOTIFICATION_TYPE_LABELS: Record<NotificationType, string> = {
  [NotificationType.AbsenceApproved]: 'Absence approuvée',
  [NotificationType.AbsenceRejected]: 'Absence refusée',
  [NotificationType.AbsencePending]: 'Absence en attente',
  [NotificationType.ClockCorrection]: 'Correction de pointage',
  [NotificationType.ClockApproved]: 'Pointage approuvé',
  [NotificationType.ClockRejected]: 'Pointage refusé',
};

/**
 * Notification type icons
 */
export const NOTIFICATION_TYPE_ICONS: Record<NotificationType, string> = {
  [NotificationType.AbsenceApproved]: 'check-circle',
  [NotificationType.AbsenceRejected]: 'x-circle',
  [NotificationType.AbsencePending]: 'clock',
  [NotificationType.ClockCorrection]: 'edit',
  [NotificationType.ClockApproved]: 'check-circle',
  [NotificationType.ClockRejected]: 'x-circle',
};

/**
 * Notification type colors
 */
export const NOTIFICATION_TYPE_COLORS: Record<NotificationType, string> = {
  [NotificationType.AbsenceApproved]: 'text-green-600',
  [NotificationType.AbsenceRejected]: 'text-red-600',
  [NotificationType.AbsencePending]: 'text-yellow-600',
  [NotificationType.ClockCorrection]: 'text-blue-600',
  [NotificationType.ClockApproved]: 'text-green-600',
  [NotificationType.ClockRejected]: 'text-red-600',
};
