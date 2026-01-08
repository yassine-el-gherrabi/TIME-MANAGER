import { useState, useEffect, type FC } from 'react';
import { toast } from 'sonner';
import { MonitorSmartphone, Calendar, Clock, Loader2, RefreshCw, LogOut } from 'lucide-react';
import { Button } from '../../components/ui/button';
import { ConfirmDialog } from '../../components/ui/confirm-dialog';
import { authApi } from '../../api/auth';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { SessionInfo } from '../../types/auth';

export const SessionsPage: FC = () => {
  const [sessions, setSessions] = useState<SessionInfo[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [revokeDialog, setRevokeDialog] = useState<{
    open: boolean;
    session: SessionInfo | null;
    loading: boolean;
    isAll: boolean;
  }>({ open: false, session: null, loading: false, isAll: false });

  const fetchSessions = async () => {
    setIsLoading(true);
    try {
      const response = await authApi.getActiveSessions();
      setSessions(response.sessions);
    } catch (error) {
      toast.error(mapErrorToMessage(error));
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchSessions();
  }, []);

  const handleRevokeClick = (session: SessionInfo) => {
    setRevokeDialog({ open: true, session, loading: false, isAll: false });
  };

  const handleRevokeAllClick = () => {
    setRevokeDialog({ open: true, session: null, loading: false, isAll: true });
  };

  const handleRevokeConfirm = async () => {
    setRevokeDialog((prev) => ({ ...prev, loading: true }));

    try {
      if (revokeDialog.isAll) {
        await authApi.logoutAll();
        toast.success('All sessions have been revoked');
        // This will log the user out
        window.location.href = '/login';
      } else if (revokeDialog.session) {
        await authApi.revokeSession(revokeDialog.session.id);
        toast.success('Session revoked successfully');
        setRevokeDialog({ open: false, session: null, loading: false, isAll: false });
        await fetchSessions();
      }
    } catch (error) {
      toast.error(mapErrorToMessage(error));
      setRevokeDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString();
  };

  const getRelativeTime = (dateString: string) => {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMins / 60);
    const diffDays = Math.floor(diffHours / 24);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins} minute${diffMins > 1 ? 's' : ''} ago`;
    if (diffHours < 24) return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`;
    return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`;
  };

  return (
    <div className="mx-auto max-w-3xl">
      <div className="mb-8 flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Active Sessions</h1>
          <p className="mt-2 text-muted-foreground">
            Manage your active sessions across all devices
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" size="sm" onClick={fetchSessions} disabled={isLoading}>
            <RefreshCw className={`mr-2 h-4 w-4 ${isLoading ? 'animate-spin' : ''}`} />
            Refresh
          </Button>
          {sessions.length > 1 && (
            <Button
              variant="destructive"
              size="sm"
              onClick={handleRevokeAllClick}
              disabled={isLoading}
            >
              <LogOut className="mr-2 h-4 w-4" />
              Revoke All
            </Button>
          )}
        </div>
      </div>

      {isLoading ? (
        <div className="flex items-center justify-center py-12">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </div>
      ) : sessions.length === 0 ? (
        <div className="rounded-lg border bg-card p-8 text-center">
          <MonitorSmartphone className="mx-auto h-12 w-12 text-muted-foreground" />
          <p className="mt-4 text-muted-foreground">No active sessions found</p>
        </div>
      ) : (
        <div className="space-y-4">
          {sessions.map((session, index) => (
            <div
              key={session.id}
              className="flex items-start justify-between rounded-lg border bg-card p-4"
            >
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
                  <MonitorSmartphone className="h-5 w-5 text-muted-foreground" />
                </div>
                <div className="space-y-1">
                  <div className="flex items-center gap-2">
                    <span className="font-medium">
                      {session.user_agent || 'Unknown Device'}
                    </span>
                    {index === 0 && (
                      <span className="rounded-full bg-green-100 px-2 py-0.5 text-xs font-medium text-green-700 dark:bg-green-900 dark:text-green-300">
                        Current
                      </span>
                    )}
                  </div>
                  <div className="flex flex-wrap gap-x-4 gap-y-1 text-sm text-muted-foreground">
                    <span className="flex items-center gap-1">
                      <Calendar className="h-3 w-3" />
                      Created: {formatDate(session.created_at)}
                    </span>
                    <span className="flex items-center gap-1">
                      <Clock className="h-3 w-3" />
                      Last activity: {getRelativeTime(session.last_activity)}
                    </span>
                  </div>
                </div>
              </div>
              {index !== 0 && (
                <Button
                  variant="ghost"
                  size="sm"
                  className="text-destructive hover:bg-destructive/10 hover:text-destructive"
                  onClick={() => handleRevokeClick(session)}
                >
                  Revoke
                </Button>
              )}
            </div>
          ))}
        </div>
      )}

      <ConfirmDialog
        open={revokeDialog.open}
        onOpenChange={(open) =>
          setRevokeDialog({ open, session: null, loading: false, isAll: false })
        }
        title={revokeDialog.isAll ? 'Revoke All Sessions' : 'Revoke Session'}
        description={
          revokeDialog.isAll
            ? 'This will log you out from all devices including this one. You will need to log in again.'
            : 'This will end the selected session. The device will need to log in again.'
        }
        confirmText={revokeDialog.isAll ? 'Revoke All' : 'Revoke'}
        variant="destructive"
        onConfirm={handleRevokeConfirm}
        loading={revokeDialog.loading}
      />
    </div>
  );
};
