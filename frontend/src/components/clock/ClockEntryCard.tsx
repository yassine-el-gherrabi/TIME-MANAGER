/**
 * Clock Entry Card Component
 *
 * Displays a single clock entry with status and actions.
 */

import type { FC } from 'react';
import { format } from 'date-fns';
import { Clock, CheckCircle, XCircle, AlertCircle } from 'lucide-react';
import { Card, CardContent } from '../ui/card';
import { Badge } from '../ui/badge';
import { cn } from '../../lib/utils';
import type { ClockEntryResponse, ClockEntryStatus } from '../../types/clock';

interface ClockEntryCardProps {
  entry: ClockEntryResponse;
  showUser?: boolean;
  className?: string;
}

const statusConfig: Record<ClockEntryStatus, { label: string; variant: 'default' | 'secondary' | 'destructive' | 'outline'; icon: typeof CheckCircle }> = {
  pending: { label: 'Pending', variant: 'secondary', icon: AlertCircle },
  approved: { label: 'Approved', variant: 'default', icon: CheckCircle },
  rejected: { label: 'Rejected', variant: 'destructive', icon: XCircle },
};

/**
 * Calculate duration between two timestamps in hours and minutes
 */
const calculateDuration = (clockIn: string, clockOut: string | null): string => {
  if (!clockOut) return 'In progress';

  const start = new Date(clockIn).getTime();
  const end = new Date(clockOut).getTime();
  const diffMs = end - start;

  const hours = Math.floor(diffMs / (1000 * 60 * 60));
  const minutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));

  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  return `${minutes}m`;
};

export const ClockEntryCard: FC<ClockEntryCardProps> = ({
  entry,
  showUser: _showUser = false,
  className,
}) => {
  const status = statusConfig[entry.status];
  const StatusIcon = status.icon;
  const duration = calculateDuration(entry.clock_in, entry.clock_out);

  return (
    <Card className={cn('w-full', className)}>
      <CardContent className="p-4">
        <div className="flex items-start justify-between">
          <div className="flex items-start gap-3">
            <div className="rounded-full bg-primary/10 p-2 mt-0.5">
              <Clock className="h-4 w-4 text-primary" />
            </div>
            <div className="space-y-1">
              <div className="flex items-center gap-2">
                <p className="text-sm font-medium">
                  {format(new Date(entry.clock_in), 'EEEE, MMM d')}
                </p>
                <Badge variant={status.variant} className="gap-1">
                  <StatusIcon className="h-3 w-3" />
                  {status.label}
                </Badge>
              </div>
              <div className="flex items-center gap-4 text-sm text-muted-foreground">
                <span>
                  In: {format(new Date(entry.clock_in), 'HH:mm')}
                </span>
                {entry.clock_out && (
                  <span>
                    Out: {format(new Date(entry.clock_out), 'HH:mm')}
                  </span>
                )}
                <span className="font-medium text-foreground">
                  {duration}
                </span>
              </div>
              {entry.notes && (
                <p className="text-xs text-muted-foreground mt-1">
                  {entry.notes}
                </p>
              )}
            </div>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};
