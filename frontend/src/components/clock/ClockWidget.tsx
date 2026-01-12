/**
 * Clock Widget Component
 *
 * Main clock in/out widget with timer display, break tracking, and restriction handling.
 */

import { useEffect, useState, useRef, useCallback, type FC } from 'react';
import { useTranslation } from 'react-i18next';
import { Clock, LogIn, LogOut, Loader2, Coffee, Play, Square, AlertTriangle, CheckCircle } from 'lucide-react';
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
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '../ui/dialog';
import { useClockStore, initializeClockStore } from '../../stores/clockStore';
import { clockRestrictionsApi } from '../../api/clockRestrictions';
import { getBreakStatus, getEffectiveBreakPolicy, startBreak, endBreak } from '../../api/breaks';
import type { ClockValidationResult } from '../../types/clockRestriction';
import type { BreakStatus, EffectiveBreakPolicy } from '../../types/break';
import { toast } from 'sonner';
import { mapErrorToMessage } from '../../utils/errorHandling';

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

  // Restriction & Break states
  const [restrictionStatus, setRestrictionStatus] = useState<ClockValidationResult | null>(null);
  const [breakStatus, setBreakStatus] = useState<BreakStatus | null>(null);
  const [effectivePolicy, setEffectivePolicy] = useState<EffectiveBreakPolicy | null>(null);
  const [overrideDialogOpen, setOverrideDialogOpen] = useState(false);
  const [overrideReason, setOverrideReason] = useState('');
  const [isSubmittingOverride, setIsSubmittingOverride] = useState(false);
  const [isBreakLoading, setIsBreakLoading] = useState(false);

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

  // Fetch restriction and break status
  const isClockedIn = status?.is_clocked_in ?? false;

  const fetchRestrictionStatus = useCallback(async () => {
    try {
      const action = isClockedIn ? 'clock_out' : 'clock_in';
      const result = await clockRestrictionsApi.validateAction(action);
      setRestrictionStatus(result);
    } catch {
      // Silently fail - restrictions are optional
      setRestrictionStatus(null);
    }
  }, [isClockedIn]);

  const fetchBreakStatus = useCallback(async () => {
    if (!isClockedIn) {
      setBreakStatus(null);
      setEffectivePolicy(null);
      return;
    }
    try {
      const [breakStatusResult, policyResult] = await Promise.all([
        getBreakStatus(),
        getEffectiveBreakPolicy(),
      ]);
      setBreakStatus(breakStatusResult);
      setEffectivePolicy(policyResult);
    } catch {
      // Silently fail - break tracking is optional
      setBreakStatus(null);
      setEffectivePolicy(null);
    }
  }, [isClockedIn]);

  // Fetch on mount and when clock status changes
  useEffect(() => {
    fetchRestrictionStatus();
    fetchBreakStatus();
  }, [fetchRestrictionStatus, fetchBreakStatus]);

  // Refresh restriction status every minute
  useEffect(() => {
    const interval = setInterval(() => {
      fetchRestrictionStatus();
    }, 60000);
    return () => clearInterval(interval);
  }, [fetchRestrictionStatus]);

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
    // Check restriction before allowing clock in
    if (restrictionStatus && !restrictionStatus.allowed) {
      if (restrictionStatus.can_request_override) {
        setOverrideDialogOpen(true);
      } else {
        toast.error(restrictionStatus.message || t('clock.actionNotAllowed'));
      }
      return;
    }

    try {
      await clockIn();
      startDebounce();
      fetchRestrictionStatus();
      fetchBreakStatus();
    } catch {
      // Error is handled by store
    }
  };

  const handleClockOutClick = () => {
    // Check restriction before allowing clock out
    if (restrictionStatus && !restrictionStatus.allowed) {
      if (restrictionStatus.can_request_override) {
        setOverrideDialogOpen(true);
      } else {
        toast.error(restrictionStatus.message || t('clock.actionNotAllowed'));
      }
      return;
    }

    setClockOutNote('');
    setShowClockOutConfirm(true);
  };

  const handleClockOutConfirm = async () => {
    try {
      await clockOut(clockOutNote.trim() || undefined);
      setClockOutNote('');
      setShowClockOutConfirm(false);
      startDebounce();
      fetchRestrictionStatus();
      fetchBreakStatus();
    } catch {
      // Error is handled by store
    }
  };

  const handleClockOutCancel = () => {
    setShowClockOutConfirm(false);
    setClockOutNote('');
  };

  // Break handlers
  const handleStartBreak = async () => {
    if (!status?.current_entry?.id) return;
    setIsBreakLoading(true);
    try {
      await startBreak(status.current_entry.id);
      await fetchBreakStatus();
      toast.success(t('clock.breakStarted'));
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setIsBreakLoading(false);
    }
  };

  const handleEndBreak = async () => {
    setIsBreakLoading(true);
    try {
      await endBreak();
      await fetchBreakStatus();
      toast.success(t('clock.breakEnded'));
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setIsBreakLoading(false);
    }
  };

  // Override request handler
  const handleOverrideRequest = async () => {
    if (!overrideReason.trim()) {
      toast.error(t('clock.overrideReasonRequired'));
      return;
    }

    setIsSubmittingOverride(true);
    try {
      const wasClockIn = !isClockedIn; // Store action type before it changes
      const result = await clockRestrictionsApi.createOverrideRequest({
        requested_action: isClockedIn ? 'clock_out' : 'clock_in',
        reason: overrideReason.trim(),
      });

      if (result.status === 'auto_approved') {
        toast.success(t('clock.overrideAutoApproved'));
        // Refresh restriction status
        await fetchRestrictionStatus();
        // Close dialog first
        setOverrideDialogOpen(false);
        setOverrideReason('');

        // Auto-trigger the clock action after a short delay
        setTimeout(async () => {
          try {
            if (wasClockIn) {
              // Clock in directly
              await clockIn();
            } else {
              // Clock out directly - override reason serves as justification, no need for another confirmation
              await clockOut();
            }
            startDebounce();
            fetchRestrictionStatus();
            fetchBreakStatus();
          } catch (err) {
            // Feedback if auto-clock fails after override approval
            toast.error(t('clock.autoClockFailed'));
            console.error('Auto clock failed after override:', err);
          }
        }, 500);
        return; // Exit early since we handled dialog closing above
      } else {
        toast.info(t('clock.overridePendingApproval'));
      }

      setOverrideDialogOpen(false);
      setOverrideReason('');
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setIsSubmittingOverride(false);
    }
  };

  const isButtonDisabled = isDebounced || isLoading || isClockingIn || isClockingOut;
  const showBreakSection = isClockedIn && effectivePolicy?.policy?.tracking_mode === 'explicit_tracking';

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
          {/* Restriction Banner */}
          {restrictionStatus && restrictionStatus.mode !== 'unrestricted' && (
            <div
              className={`p-3 rounded-md text-sm ${
                restrictionStatus.allowed
                  ? 'bg-green-50 text-green-800 border border-green-200 dark:bg-green-950 dark:text-green-200 dark:border-green-800'
                  : 'bg-yellow-50 text-yellow-800 border border-yellow-200 dark:bg-yellow-950 dark:text-yellow-200 dark:border-yellow-800'
              }`}
            >
              {restrictionStatus.allowed ? (
                <div className="flex items-center gap-2">
                  <CheckCircle className="h-4 w-4 flex-shrink-0" />
                  <span>
                    {isClockedIn ? t('clock.clockOutAllowed') : t('clock.clockInAllowed')}
                  </span>
                </div>
              ) : (
                <div className="flex items-center justify-between gap-2">
                  <div className="flex items-center gap-2">
                    <AlertTriangle className="h-4 w-4 flex-shrink-0" />
                    <span>{restrictionStatus.message || t('clock.outsideAllowedWindow')}</span>
                  </div>
                  {restrictionStatus.can_request_override && (
                    <Button
                      variant="link"
                      size="sm"
                      className="h-auto p-0 text-yellow-800 dark:text-yellow-200"
                      onClick={() => setOverrideDialogOpen(true)}
                    >
                      {t('clock.requestOverride')}
                    </Button>
                  )}
                </div>
              )}
            </div>
          )}

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

          {/* Break Section (explicit tracking mode only) */}
          {showBreakSection && (
            <div className="flex items-center justify-between p-3 bg-muted/50 rounded-md">
              <div className="flex items-center gap-2">
                <Coffee className="h-4 w-4 text-muted-foreground" />
                {breakStatus?.is_on_break ? (
                  <span className="text-sm">
                    {t('clock.onBreak')}:{' '}
                    <strong>{breakStatus.elapsed_minutes ?? 0} min</strong>
                  </span>
                ) : (
                  <span className="text-sm text-muted-foreground">
                    {t('clock.noBreakTaken')}
                  </span>
                )}
              </div>

              <Button
                variant={breakStatus?.is_on_break ? 'destructive' : 'outline'}
                size="sm"
                onClick={breakStatus?.is_on_break ? handleEndBreak : handleStartBreak}
                disabled={isBreakLoading}
              >
                {isBreakLoading ? (
                  <Loader2 className="h-3 w-3 animate-spin" />
                ) : breakStatus?.is_on_break ? (
                  <>
                    <Square className="h-3 w-3 mr-1" />
                    {t('clock.endBreak')}
                  </>
                ) : (
                  <>
                    <Play className="h-3 w-3 mr-1" />
                    {t('clock.startBreak')}
                  </>
                )}
              </Button>
            </div>
          )}

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

      {/* Override Request Dialog */}
      <Dialog open={overrideDialogOpen} onOpenChange={setOverrideDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('clock.overrideRequestTitle')}</DialogTitle>
            <DialogDescription>
              {t('clock.overrideRequestDescription')}
            </DialogDescription>
          </DialogHeader>
          <div className="py-2">
            <Label htmlFor="override-reason" className="text-sm font-medium">
              {t('clock.overrideReason')}
            </Label>
            <Textarea
              id="override-reason"
              value={overrideReason}
              onChange={(e) => setOverrideReason(e.target.value)}
              placeholder={t('clock.overrideReasonPlaceholder')}
              className="mt-2 min-h-[100px] text-sm resize-none"
              maxLength={500}
            />
          </div>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => {
                setOverrideDialogOpen(false);
                setOverrideReason('');
              }}
              disabled={isSubmittingOverride}
            >
              {t('common.cancel')}
            </Button>
            <Button
              onClick={handleOverrideRequest}
              disabled={isSubmittingOverride || !overrideReason.trim()}
            >
              {isSubmittingOverride ? (
                <>
                  <Loader2 className="h-4 w-4 mr-2 animate-spin" />
                  {t('common.submitting')}
                </>
              ) : (
                t('clock.submitOverrideRequest')
              )}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};
