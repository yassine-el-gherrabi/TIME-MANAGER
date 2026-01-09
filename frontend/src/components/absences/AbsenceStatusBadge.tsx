/**
 * Absence Status Badge Component
 *
 * Displays absence status with color-coded styling.
 */

import type { FC } from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { CheckCircle, XCircle, Clock, Ban } from 'lucide-react';
import { cn } from '../../lib/utils';
import { AbsenceStatus } from '../../types/absence';

const absenceStatusVariants = cva(
  'inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 text-xs font-semibold transition-colors',
  {
    variants: {
      status: {
        pending:
          'bg-amber-100 text-amber-800 dark:bg-amber-900/30 dark:text-amber-400',
        approved:
          'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400',
        rejected:
          'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400',
        cancelled:
          'bg-gray-100 text-gray-800 dark:bg-gray-900/30 dark:text-gray-400',
      },
    },
    defaultVariants: {
      status: 'pending',
    },
  }
);

const STATUS_CONFIG: Record<
  AbsenceStatus,
  { label: string; Icon: typeof CheckCircle }
> = {
  [AbsenceStatus.Pending]: { label: 'Pending', Icon: Clock },
  [AbsenceStatus.Approved]: { label: 'Approved', Icon: CheckCircle },
  [AbsenceStatus.Rejected]: { label: 'Rejected', Icon: XCircle },
  [AbsenceStatus.Cancelled]: { label: 'Cancelled', Icon: Ban },
};

export interface AbsenceStatusBadgeProps
  extends Omit<VariantProps<typeof absenceStatusVariants>, 'status'> {
  status: AbsenceStatus;
  className?: string;
  showIcon?: boolean;
}

export const AbsenceStatusBadge: FC<AbsenceStatusBadgeProps> = ({
  status,
  className,
  showIcon = true,
}) => {
  const config = STATUS_CONFIG[status];
  const Icon = config.Icon;

  return (
    <span className={cn(absenceStatusVariants({ status }), className)}>
      {showIcon && <Icon className="h-3 w-3" />}
      {config.label}
    </span>
  );
};
