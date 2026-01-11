/**
 * Clock Widget Component
 *
 * Main clock in/out widget with timer display and confirmation dialog.
 */

import { useEffect, useState, useRef, useCallback, type FC } from 'react';
import { useTranslation } from 'react-i18next';
import { Clock, LogIn, LogOut, Loader2 } from 'lucide-react';
import { Button } from '../ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import { Textarea } from '../ui/textarea';
import { Label } from '../ui/label';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '../ui/alert-dialog';
import { useClockStore, initializeClockStore } from '../../stores/clockStore';

/** Debounce delay in milliseconds */
const DEBOUNCE_DELAY = 2000;

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
  const { t } = useTranslation();
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
  const [showClockOutConfirm, setShowClockOutConfirm] = useState(false);
  const [clockOutNote, setClockOutNote] = useState('');
  const [isDebounced, setIsDebounced] = useState(false);
  const debounceTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Initialize store on mount
  useEffect(() => {
    initializeClockStore();
  }, []);

  // Cleanup debounce timer on unmount
  useEffect(() => {
    return () => {
      if (debounceTimerRef.current) {
        clearTimeout(debounceTimerRef.current);
      }
    };
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
      const interval = setInterval(updateElapsed, 1000);
      return () => clearInterval(interval);
    } else {
      setElapsedSeconds(null);
    }
  }, [status]);

  /**
   * Start debounce timer after action
   */
  const startDebounce = useCallback(() => {
    setIsDebounced(true);
    debounceTimerRef.current = setTimeout(() => {
      setIsDebounced(false);
    }, DEBOUNCE_DELAY);
  }, []);

  const handleClockIn = async () => {
    try {
      await clockIn();
      startDebounce();
    } catch {
      // Error is handled by store
    }
  };

  const handleClockOutClick = () => {
    setClockOutNote('');
    setShowClockOutConfirm(true);
  };

  const handleClockOutConfirm = async () => {
    try {
      await clockOut(clockOutNote.trim() || undefined);
      setClockOutNote('');
      setShowClockOutConfirm(false);
      startDebounce();
    } catch {
      // Error is handled by store
    }
  };

  const handleClockOutCancel = () => {
    setShowClockOutConfirm(false);
    setClockOutNote('');
  };

  const isClockedIn = status?.is_clocked_in ?? false;
  const isButtonDisabled = isDebounced || isLoading || isClockingIn || isClockingOut;

  return (
    <>
      <Card className="w-full">
        <CardHeader className="pb-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Clock className="h-5 w-5 text-primary" />
              <CardTitle className="text-lg">{t('clock.title')}</CardTitle>
            </div>
            <span
              className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
                isClockedIn
                  ? 'bg-green-100 text-green-800'
                  : 'bg-gray-100 text-gray-800'
              }`}
            >
              {isClockedIn ? t('clock.clockedIn') : t('clock.clockedOut')}
            </span>
          </div>
          {isClockedIn && status?.current_entry && (
            <CardDescription>
              {t('clock.startedAt')} {formatTime(status.current_entry.clock_in)}
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
                {t('clock.timeWorkedToday')}
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
                {t('common.dismiss')}
              </button>
            </div>
          )}

          {/* Clock In/Out Button */}
          <div className="flex justify-center">
            {isClockedIn ? (
              <Button
                onClick={handleClockOutClick}
                disabled={isButtonDisabled}
                variant="destructive"
                size="lg"
                className="w-full max-w-xs gap-2"
              >
                {isClockingOut ? (
                  <Loader2 className="h-5 w-5 animate-spin" />
                ) : (
                  <LogOut className="h-5 w-5" />
                )}
                {isClockingOut ? t('clock.clockingOut') : t('clock.clockOut')}
              </Button>
            ) : (
              <Button
                onClick={handleClockIn}
                disabled={isButtonDisabled}
                size="lg"
                className="w-full max-w-xs gap-2"
              >
                {isClockingIn ? (
                  <Loader2 className="h-5 w-5 animate-spin" />
                ) : (
                  <LogIn className="h-5 w-5" />
                )}
                {isClockingIn ? t('clock.clockingIn') : t('clock.clockIn')}
              </Button>
            )}
          </div>

          {/* Debounce indicator */}
          {isDebounced && (
            <p className="text-xs text-center text-muted-foreground">
              {t('clock.pleaseWait')}
            </p>
          )}
        </CardContent>
      </Card>

      {/* Clock Out Confirmation Dialog */}
      <AlertDialog open={showClockOutConfirm} onOpenChange={setShowClockOutConfirm}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>{t('clock.confirmClockOut')}</AlertDialogTitle>
            <AlertDialogDescription>
              {t('clock.confirmClockOutDescription')}
            </AlertDialogDescription>
          </AlertDialogHeader>

          <div className="py-2">
            <Label htmlFor="clock-out-note" className="text-sm font-medium">
              {t('clock.addNote')}
            </Label>
            <Textarea
              id="clock-out-note"
              value={clockOutNote}
              onChange={(e) => setClockOutNote(e.target.value)}
              placeholder={t('clock.notePlaceholder')}
              className="mt-2 min-h-[80px] text-sm resize-none"
              maxLength={500}
            />
          </div>

          <AlertDialogFooter>
            <AlertDialogCancel onClick={handleClockOutCancel} disabled={isClockingOut}>
              {t('common.cancel')}
            </AlertDialogCancel>
            <AlertDialogAction
              onClick={handleClockOutConfirm}
              disabled={isClockingOut}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              {isClockingOut ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  {t('clock.clockingOut')}
                </>
              ) : (
                t('clock.clockOut')
              )}
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
};
