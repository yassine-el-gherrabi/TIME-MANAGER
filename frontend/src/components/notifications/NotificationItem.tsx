import type { FC } from 'react';
import { formatDistanceToNow } from 'date-fns';
import { fr } from 'date-fns/locale';
import {
  CheckCircle,
  XCircle,
  Clock,
  Edit,
} from 'lucide-react';
import { cn } from '../../lib/utils';
import type { Notification } from '../../types/notification';
import { NotificationType } from '../../types/notification';

interface NotificationItemProps {
  notification: Notification;
  onClick?: () => void;
}

/**
 * Get icon for notification type
 */
const getNotificationIcon = (type: NotificationType) => {
  switch (type) {
    case NotificationType.AbsenceApproved:
    case NotificationType.ClockApproved:
      return <CheckCircle className="h-4 w-4 text-green-600" />;
    case NotificationType.AbsenceRejected:
    case NotificationType.ClockRejected:
      return <XCircle className="h-4 w-4 text-red-600" />;
    case NotificationType.AbsencePending:
      return <Clock className="h-4 w-4 text-yellow-600" />;
    case NotificationType.ClockCorrection:
      return <Edit className="h-4 w-4 text-blue-600" />;
    default:
      return <Clock className="h-4 w-4 text-muted-foreground" />;
  }
};

/**
 * NotificationItem component
 *
 * Displays a single notification with icon, title, message, and timestamp.
 */
export const NotificationItem: FC<NotificationItemProps> = ({ notification, onClick }) => {
  const isUnread = !notification.read_at;
  const timeAgo = formatDistanceToNow(new Date(notification.created_at), {
    addSuffix: true,
    locale: fr,
  });

  return (
    <button
      type="button"
      className={cn(
        'flex w-full items-start gap-3 rounded-md px-3 py-2 text-left transition-colors hover:bg-accent',
        isUnread && 'bg-accent/50'
      )}
      onClick={onClick}
    >
      <div className="mt-0.5 flex-shrink-0">
        {getNotificationIcon(notification.type)}
      </div>
      <div className="min-w-0 flex-1">
        <div className="flex items-center gap-2">
          <p className={cn('text-sm truncate', isUnread && 'font-medium')}>
            {notification.title}
          </p>
          {isUnread && (
            <span className="flex-shrink-0 h-2 w-2 rounded-full bg-primary" />
          )}
        </div>
        <p className="mt-0.5 text-xs text-muted-foreground line-clamp-2">
          {notification.message}
        </p>
        <p className="mt-1 text-xs text-muted-foreground/70">
          {timeAgo}
        </p>
      </div>
    </button>
  );
};
