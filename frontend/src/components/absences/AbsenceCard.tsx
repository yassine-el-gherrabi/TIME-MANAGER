/**
 * Absence Card Component
 *
 * Displays a single absence request with details and status.
 */

import type { FC } from 'react';
import { format } from 'date-fns';
import { Calendar, User } from 'lucide-react';
import { Card, CardContent } from '../ui/card';
import { cn } from '../../lib/utils';
import { AbsenceStatusBadge } from './AbsenceStatusBadge';
import type { Absence, AbsenceType } from '../../types/absence';

interface AbsenceCardProps {
  absence: Absence;
  absenceType?: AbsenceType;
  showUser?: boolean;
  userName?: string;
  className?: string;
  actions?: React.ReactNode;
}

/**
 * Format date range for display
 */
const formatDateRange = (startDate: string, endDate: string): string => {
  const start = new Date(startDate);
  const end = new Date(endDate);

  if (startDate === endDate) {
    return format(start, 'EEEE, MMM d, yyyy');
  }

  // Same month
  if (start.getMonth() === end.getMonth() && start.getFullYear() === end.getFullYear()) {
    return `${format(start, 'MMM d')} - ${format(end, 'd, yyyy')}`;
  }

  // Different months
  return `${format(start, 'MMM d')} - ${format(end, 'MMM d, yyyy')}`;
};

export const AbsenceCard: FC<AbsenceCardProps> = ({
  absence,
  absenceType,
  showUser = false,
  userName,
  className,
  actions,
}) => {
  const dateRange = formatDateRange(absence.start_date, absence.end_date);

  return (
    <Card className={cn('w-full', className)}>
      <CardContent className="p-4">
        <div className="flex items-start justify-between">
          <div className="flex items-start gap-3">
            {/* Type color indicator */}
            <div
              className="mt-1 h-10 w-1 rounded-full"
              style={{ backgroundColor: absenceType?.color || '#6B7280' }}
            />

            <div className="space-y-1">
              {/* Header with type and status */}
              <div className="flex items-center gap-2">
                <p className="text-sm font-medium">
                  {absenceType?.name || 'Unknown Type'}
                </p>
                <AbsenceStatusBadge status={absence.status} />
              </div>

              {/* Date info */}
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Calendar className="h-3.5 w-3.5" />
                <span>{dateRange}</span>
                <span className="font-medium text-foreground">
                  ({absence.days_count} {absence.days_count === 1 ? 'day' : 'days'})
                </span>
              </div>

              {/* User info if showing */}
              {showUser && userName && (
                <div className="flex items-center gap-2 text-sm text-muted-foreground">
                  <User className="h-3.5 w-3.5" />
                  <span>{userName}</span>
                </div>
              )}

              {/* Reason if provided */}
              {absence.reason && (
                <p className="text-xs text-muted-foreground mt-1">
                  {absence.reason}
                </p>
              )}

              {/* Rejection reason if rejected */}
              {absence.rejection_reason && (
                <p className="text-xs text-red-600 mt-1">
                  Rejected: {absence.rejection_reason}
                </p>
              )}
            </div>
          </div>

          {/* Actions slot */}
          {actions && (
            <div className="flex items-center gap-2">
              {actions}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
};
