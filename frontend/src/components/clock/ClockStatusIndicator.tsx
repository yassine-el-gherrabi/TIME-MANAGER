/**
 * Clock Status Indicator
 *
 * Small badge showing clock in/out status for sidebar navigation.
 */

import { useEffect } from 'react';
import { useClockStore } from '../../stores/clockStore';
import { cn } from '../../lib/utils';

export interface ClockStatusIndicatorProps {
  className?: string;
}

export function ClockStatusIndicator({ className }: ClockStatusIndicatorProps) {
  const { status, fetchStatus } = useClockStore();

  // Fetch status on mount if not already loaded
  useEffect(() => {
    if (!status) {
      fetchStatus();
    }
  }, [status, fetchStatus]);

  const isClockedIn = status?.is_clocked_in ?? false;

  return (
    <span
      className={cn(
        'inline-flex w-2 h-2 rounded-full transition-colors',
        isClockedIn ? 'bg-green-500 animate-pulse' : 'bg-gray-400',
        className
      )}
      title={isClockedIn ? 'Clocked In' : 'Clocked Out'}
    />
  );
}
