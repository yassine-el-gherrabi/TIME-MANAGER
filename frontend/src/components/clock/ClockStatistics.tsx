import { Clock, Calculator, CalendarDays, CheckCircle } from 'lucide-react';
import { Card, CardContent } from '../ui/card';
import type { ClockEntryResponse } from '../../types/clock';

export interface ClockStatisticsProps {
  entries: ClockEntryResponse[];
  isLoading?: boolean;
}

interface StatCardProps {
  title: string;
  value: string;
  subtitle?: string;
  icon: React.ReactNode;
  color: string;
}

function StatCard({ title, value, subtitle, icon, color }: StatCardProps) {
  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex items-center gap-4">
          <div className={`p-3 rounded-lg ${color}`}>{icon}</div>
          <div>
            <p className="text-sm text-muted-foreground">{title}</p>
            <p className="text-2xl font-bold">{value}</p>
            {subtitle && <p className="text-xs text-muted-foreground">{subtitle}</p>}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}

export function ClockStatistics({ entries, isLoading }: ClockStatisticsProps) {
  if (isLoading) {
    return (
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {[...Array(4)].map((_, i) => (
          <Card key={i}>
            <CardContent className="pt-6">
              <div className="animate-pulse flex items-center gap-4">
                <div className="w-12 h-12 bg-muted rounded-lg" />
                <div className="space-y-2">
                  <div className="h-3 w-20 bg-muted rounded" />
                  <div className="h-6 w-16 bg-muted rounded" />
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    );
  }

  // Calculate statistics
  const totalMinutes = entries.reduce((sum, entry) => sum + (entry.duration_minutes || 0), 0);
  const totalHours = Math.floor(totalMinutes / 60);
  const remainingMinutes = totalMinutes % 60;

  // Get unique days worked
  const uniqueDays = new Set(
    entries.map((entry) => entry.clock_in.split('T')[0])
  ).size;

  const avgHoursPerDay = uniqueDays > 0 ? totalMinutes / uniqueDays / 60 : 0;

  // Status breakdown
  const statusCounts = entries.reduce(
    (acc, entry) => {
      acc[entry.status] = (acc[entry.status] || 0) + 1;
      return acc;
    },
    {} as Record<string, number>
  );

  const approvedCount = statusCounts['approved'] || 0;
  const pendingCount = statusCounts['pending'] || 0;
  const rejectedCount = statusCounts['rejected'] || 0;

  return (
    <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <StatCard
        title="Total Hours"
        value={`${totalHours}h ${remainingMinutes}m`}
        subtitle={`${entries.length} entries`}
        icon={<Clock className="h-5 w-5 text-blue-600" />}
        color="bg-blue-100"
      />
      <StatCard
        title="Average Daily"
        value={`${avgHoursPerDay.toFixed(1)}h`}
        subtitle={`${uniqueDays} days worked`}
        icon={<Calculator className="h-5 w-5 text-purple-600" />}
        color="bg-purple-100"
      />
      <StatCard
        title="Entries"
        value={entries.length.toString()}
        subtitle={`${uniqueDays} unique days`}
        icon={<CalendarDays className="h-5 w-5 text-orange-600" />}
        color="bg-orange-100"
      />
      <StatCard
        title="Approved"
        value={`${approvedCount}/${entries.length}`}
        subtitle={pendingCount > 0 ? `${pendingCount} pending` : rejectedCount > 0 ? `${rejectedCount} rejected` : 'All processed'}
        icon={<CheckCircle className="h-5 w-5 text-green-600" />}
        color="bg-green-100"
      />
    </div>
  );
}
