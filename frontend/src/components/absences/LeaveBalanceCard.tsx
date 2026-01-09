/**
 * Leave Balance Card Component
 *
 * Displays a leave balance with remaining days and progress.
 */

import type { FC } from 'react';
import { Card, CardContent } from '../ui/card';
import { cn } from '../../lib/utils';
import type { LeaveBalance } from '../../types/absence';

interface LeaveBalanceCardProps {
  balance: LeaveBalance;
  className?: string;
}

/**
 * Calculate the percentage used of initial balance
 */
const calculatePercentage = (used: number, initial: number): number => {
  if (initial <= 0) return 0;
  return Math.min(100, Math.round((used / initial) * 100));
};

/**
 * Get color based on remaining percentage
 */
const getColorClass = (remaining: number, initial: number): string => {
  if (initial <= 0) return 'bg-gray-400';
  const percentage = (remaining / initial) * 100;
  if (percentage > 50) return 'bg-green-500';
  if (percentage > 25) return 'bg-amber-500';
  return 'bg-red-500';
};

export const LeaveBalanceCard: FC<LeaveBalanceCardProps> = ({
  balance,
  className,
}) => {
  const { type_name, initial_balance, used, adjustment, remaining } = balance;
  const usedPercentage = calculatePercentage(used, initial_balance);
  const colorClass = getColorClass(remaining, initial_balance);

  return (
    <Card className={cn('w-full', className)}>
      <CardContent className="p-4">
        <div className="space-y-3">
          <div className="flex items-center justify-between">
            <h4 className="text-sm font-medium text-muted-foreground">
              {type_name}
            </h4>
            <span className="text-2xl font-bold">{remaining}</span>
          </div>

          {/* Progress bar */}
          <div className="h-2 w-full rounded-full bg-gray-200 dark:bg-gray-700">
            <div
              className={cn('h-full rounded-full transition-all', colorClass)}
              style={{ width: `${100 - usedPercentage}%` }}
            />
          </div>

          {/* Details */}
          <div className="flex items-center justify-between text-xs text-muted-foreground">
            <span>
              Used: {used} / {initial_balance}
            </span>
            {adjustment !== 0 && (
              <span className={adjustment > 0 ? 'text-green-600' : 'text-red-600'}>
                {adjustment > 0 ? '+' : ''}{adjustment} adj.
              </span>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
};
