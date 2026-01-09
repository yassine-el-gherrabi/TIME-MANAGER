/**
 * Pending Absence Card Component
 *
 * Displays a pending absence request with approve/reject actions.
 */

import type { FC } from 'react';
import { format } from 'date-fns';
import { Calendar, User, CheckCircle, XCircle } from 'lucide-react';
import { Card, CardContent } from '../ui/card';
import { Button } from '../ui/button';
import { cn } from '../../lib/utils';
import type { Absence, AbsenceType } from '../../types/absence';

interface PendingAbsenceCardProps {
  absence: Absence;
  absenceType?: AbsenceType;
  userName: string;
  onApprove: (absenceId: string) => void;
  onReject: (absenceId: string) => void;
  isApproving?: boolean;
  isRejecting?: boolean;
  className?: string;
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

  if (start.getMonth() === end.getMonth() && start.getFullYear() === end.getFullYear()) {
    return `${format(start, 'MMM d')} - ${format(end, 'd, yyyy')}`;
  }

  return `${format(start, 'MMM d')} - ${format(end, 'MMM d, yyyy')}`;
};

export const PendingAbsenceCard: FC<PendingAbsenceCardProps> = ({
  absence,
  absenceType,
  userName,
  onApprove,
  onReject,
  isApproving = false,
  isRejecting = false,
  className,
}) => {
  const dateRange = formatDateRange(absence.start_date, absence.end_date);
  const isLoading = isApproving || isRejecting;

  return (
    <Card className={cn('w-full', className)}>
      <CardContent className="p-4">
        <div className="flex items-start justify-between gap-4">
          <div className="flex items-start gap-3 flex-1">
            {/* Type color indicator */}
            <div
              className="mt-1 h-12 w-1 rounded-full flex-shrink-0"
              style={{ backgroundColor: absenceType?.color || '#6B7280' }}
            />

            <div className="space-y-2 flex-1">
              {/* User and type */}
              <div className="flex items-center gap-2 flex-wrap">
                <div className="flex items-center gap-1.5">
                  <User className="h-4 w-4 text-muted-foreground" />
                  <span className="font-medium">{userName}</span>
                </div>
                <span className="text-muted-foreground">•</span>
                <span className="text-sm" style={{ color: absenceType?.color }}>
                  {absenceType?.name || 'Unknown Type'}
                </span>
              </div>

              {/* Date info */}
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Calendar className="h-3.5 w-3.5" />
                <span>{dateRange}</span>
                <span className="font-medium text-foreground">
                  ({absence.days_count} {absence.days_count === 1 ? 'day' : 'days'})
                </span>
              </div>

              {/* Reason if provided */}
              {absence.reason && (
                <p className="text-sm text-muted-foreground bg-muted/50 rounded-md px-2 py-1">
                  "{absence.reason}"
                </p>
              )}

              {/* Submitted date */}
              <p className="text-xs text-muted-foreground">
                Submitted {format(new Date(absence.created_at), 'MMM d, yyyy \'at\' HH:mm')}
              </p>
            </div>
          </div>

          {/* Action buttons */}
          <div className="flex flex-col gap-2 flex-shrink-0">
            <Button
              size="sm"
              onClick={() => onApprove(absence.id)}
              disabled={isLoading}
              className="gap-1"
            >
              <CheckCircle className="h-4 w-4" />
              {isApproving ? 'Approving...' : 'Approve'}
            </Button>
            <Button
              size="sm"
              variant="outline"
              onClick={() => onReject(absence.id)}
              disabled={isLoading}
              className="gap-1 text-destructive hover:text-destructive"
            >
              <XCircle className="h-4 w-4" />
              {isRejecting ? 'Rejecting...' : 'Reject'}
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};
