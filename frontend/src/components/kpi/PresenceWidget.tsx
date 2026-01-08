/**
 * Presence Widget Component
 *
 * Displays real-time presence overview (Manager+ only).
 */

import { useEffect, type FC } from 'react';
import { Users, UserCheck, Loader2 } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { useKPIStore } from '../../stores/kpiStore';

/**
 * Format elapsed minutes to readable format
 */
const formatElapsed = (minutes: number): string => {
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  if (hours > 0) {
    return `${hours}h ${mins}m`;
  }
  return `${mins}m`;
};

export const PresenceWidget: FC = () => {
  const { presence, isLoading, error, fetchPresence } = useKPIStore();

  useEffect(() => {
    fetchPresence();
    // Refresh every 5 minutes
    const interval = setInterval(fetchPresence, 5 * 60 * 1000);
    return () => clearInterval(interval);
  }, [fetchPresence]);

  if (isLoading && !presence) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <Users className="h-5 w-5" />
            Presence
          </CardTitle>
        </CardHeader>
        <CardContent className="flex items-center justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <Users className="h-5 w-5" />
            Presence
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-destructive">{error}</p>
        </CardContent>
      </Card>
    );
  }

  if (!presence) {
    return null;
  }

  const presenceRate = presence.total_employees > 0
    ? Math.round((presence.currently_present / presence.total_employees) * 100)
    : 0;

  return (
    <Card>
      <CardHeader className="pb-3">
        <CardTitle className="flex items-center justify-between text-lg">
          <span className="flex items-center gap-2">
            <Users className="h-5 w-5 text-primary" />
            Real-time Presence
          </span>
          <span className="text-sm font-normal text-muted-foreground">
            {presenceRate}% present
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Stats */}
        <div className="grid grid-cols-2 gap-4">
          <div className="text-center p-3 bg-muted/50 rounded-lg">
            <div className="text-2xl font-bold text-green-600">
              {presence.currently_present}
            </div>
            <div className="text-xs text-muted-foreground">Currently Present</div>
          </div>
          <div className="text-center p-3 bg-muted/50 rounded-lg">
            <div className="text-2xl font-bold">
              {presence.total_employees}
            </div>
            <div className="text-xs text-muted-foreground">Total Employees</div>
          </div>
        </div>

        {/* Present Users List */}
        {presence.present_users.length > 0 && (
          <div className="space-y-2">
            <h4 className="text-sm font-medium text-muted-foreground">
              Who&apos;s In ({presence.present_users.length})
            </h4>
            <div className="space-y-2 max-h-48 overflow-y-auto">
              {presence.present_users.map((user) => (
                <div
                  key={user.user_id}
                  className="flex items-center justify-between p-2 bg-muted/30 rounded-md"
                >
                  <div className="flex items-center gap-2">
                    <UserCheck className="h-4 w-4 text-green-600" />
                    <span className="text-sm font-medium">{user.user_name}</span>
                  </div>
                  <span className="text-xs text-muted-foreground">
                    {formatElapsed(user.elapsed_minutes)}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}

        {presence.present_users.length === 0 && (
          <p className="text-center text-sm text-muted-foreground py-4">
            No one is currently clocked in
          </p>
        )}
      </CardContent>
    </Card>
  );
};
