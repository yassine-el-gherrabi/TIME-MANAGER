/**
 * Clock Widget Component
 *
 * Main clock in/out widget with timer display.
 */

import { useEffect, useState, type FC } from 'react';
import { Clock, LogIn, LogOut, Loader2 } from 'lucide-react';
import { Button } from '../ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import { useClockStore, initializeClockStore } from '../../stores/clockStore';

/**
 * Format seconds to HH:MM:SS
 */
const formatElapsedTime = (seconds: number | null): string => {
  if (seconds === null) return '--:--:--';
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);
  return `${hours.toString().padStart(2, '0')}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
};

/**
 * Format ISO date to readable time
 */
const formatTime = (isoDate: string): string => {
  return new Date(isoDate).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
};

export const ClockWidget: FC = () => {
  const {
    status,
    isLoading,
    isClockingIn,
    isClockingOut,
    error,
    clockIn,
    clockOut,
    clearError,
  } = useClockStore();

  const [elapsedSeconds, setElapsedSeconds] = useState<number | null>(null);

  // Initialize store on mount
  useEffect(() => {
    initializeClockStore();
  }, []);

  // Update elapsed time every second when clocked in
  useEffect(() => {
    if (status?.is_clocked_in && status.current_entry) {
      const updateElapsed = () => {
        const clockInTime = new Date(status.current_entry!.clock_in);
        const now = new Date();
        const diffMs = now.getTime() - clockInTime.getTime();
        setElapsedSeconds(Math.floor(diffMs / 1000));
      };

      updateElapsed();
      const interval = setInterval(updateElapsed, 1000); // Update every second
      return () => clearInterval(interval);
    } else {
      setElapsedSeconds(null);
    }
  }, [status]);

  const handleClockIn = async () => {
    try {
      await clockIn();
    } catch (err) {
      // Error is handled by store
    }
  };

  const handleClockOut = async () => {
    try {
      await clockOut();
    } catch (err) {
      // Error is handled by store
    }
  };

  const isClockedIn = status?.is_clocked_in ?? false;

  return (
    <Card className="w-full">
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Clock className="h-5 w-5 text-primary" />
            <CardTitle className="text-lg">Time Clock</CardTitle>
          </div>
          <span
            className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
              isClockedIn
                ? 'bg-green-100 text-green-800'
                : 'bg-gray-100 text-gray-800'
            }`}
          >
            {isClockedIn ? 'Clocked In' : 'Clocked Out'}
          </span>
        </div>
        {isClockedIn && status?.current_entry && (
          <CardDescription>
            Started at {formatTime(status.current_entry.clock_in)}
          </CardDescription>
        )}
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Timer Display */}
        <div className="text-center">
          <div className="text-4xl font-mono font-bold text-foreground">
            {formatElapsedTime(elapsedSeconds)}
          </div>
          {isClockedIn && (
            <p className="text-sm text-muted-foreground mt-1">
              Time worked today
            </p>
          )}
        </div>

        {/* Error Message */}
        {error && (
          <div className="rounded-md bg-destructive/15 p-3">
            <p className="text-sm text-destructive">{error}</p>
            <button
              onClick={clearError}
              className="text-xs text-destructive/80 underline mt-1"
            >
              Dismiss
            </button>
          </div>
        )}

        {/* Clock In/Out Button */}
        <div className="flex justify-center">
          {isClockedIn ? (
            <Button
              onClick={handleClockOut}
              disabled={isClockingOut || isLoading}
              variant="destructive"
              size="lg"
              className="w-full max-w-xs gap-2"
            >
              {isClockingOut ? (
                <Loader2 className="h-5 w-5 animate-spin" />
              ) : (
                <LogOut className="h-5 w-5" />
              )}
              {isClockingOut ? 'Clocking Out...' : 'Clock Out'}
            </Button>
          ) : (
            <Button
              onClick={handleClockIn}
              disabled={isClockingIn || isLoading}
              size="lg"
              className="w-full max-w-xs gap-2"
            >
              {isClockingIn ? (
                <Loader2 className="h-5 w-5 animate-spin" />
              ) : (
                <LogIn className="h-5 w-5" />
              )}
              {isClockingIn ? 'Clocking In...' : 'Clock In'}
            </Button>
          )}
        </div>
      </CardContent>
    </Card>
  );
};
