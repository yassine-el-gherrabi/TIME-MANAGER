/**
 * Schedules Table Component
 *
 * Displays work schedules in a table format with actions.
 */

import React from 'react';
import { Button } from '../ui/button';
import { Badge } from '../ui/badge';
import type { WorkScheduleWithDays } from '../../types/schedule';

export interface SchedulesTableProps {
  schedules: WorkScheduleWithDays[];
  onEdit: (schedule: WorkScheduleWithDays) => void;
  onAssign: (schedule: WorkScheduleWithDays) => void;
  onDelete: (schedule: WorkScheduleWithDays) => void;
  isLoading?: boolean;
}

/**
 * Calculate total weekly hours for a schedule
 */
function calculateWeeklyHours(schedule: WorkScheduleWithDays): number {
  return schedule.days.reduce((total, day) => {
    const [startH, startM] = day.start_time.split(':').map(Number);
    const [endH, endM] = day.end_time.split(':').map(Number);
    const startMinutes = startH * 60 + startM;
    const endMinutes = endH * 60 + endM;
    const workMinutes = endMinutes - startMinutes - day.break_minutes;
    return total + Math.max(0, workMinutes) / 60;
  }, 0);
}

/**
 * Format active days as abbreviated string
 */
function formatActiveDays(schedule: WorkScheduleWithDays): string {
  const dayAbbrevs = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];
  const activeDays = new Set(schedule.days.map((d) => d.day_of_week));

  // Check for common patterns
  const isWeekdays = activeDays.size === 5 &&
    [0, 1, 2, 3, 4].every((d) => activeDays.has(d));
  if (isWeekdays) return 'Mon-Fri';

  const isWeekend = activeDays.size === 2 &&
    [5, 6].every((d) => activeDays.has(d));
  if (isWeekend) return 'Sat-Sun';

  // Otherwise list days
  return schedule.days
    .map((d) => dayAbbrevs[d.day_of_week])
    .sort((a, b) => dayAbbrevs.indexOf(a) - dayAbbrevs.indexOf(b))
    .join(', ');
}

export const SchedulesTable: React.FC<SchedulesTableProps> = ({
  schedules,
  onEdit,
  onAssign,
  onDelete,
  isLoading,
}) => {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">Loading schedules...</div>
      </div>
    );
  }

  if (schedules.length === 0) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">No schedules found</div>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b bg-muted/50">
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              Name
            </th>
            <th className="px-4 py-3 text-center text-sm font-medium text-muted-foreground">
              Hours/Week
            </th>
            <th className="px-4 py-3 text-center text-sm font-medium text-muted-foreground">
              Days
            </th>
            <th className="px-4 py-3 text-center text-sm font-medium text-muted-foreground">
              Default
            </th>
            <th className="px-4 py-3 text-right text-sm font-medium text-muted-foreground">
              Actions
            </th>
          </tr>
        </thead>
        <tbody>
          {schedules.map((schedule) => {
            const weeklyHours = calculateWeeklyHours(schedule);
            const activeDays = formatActiveDays(schedule);

            return (
              <tr
                key={schedule.schedule.id}
                className="border-b hover:bg-muted/25 transition-colors"
              >
                <td className="px-4 py-3 text-sm">
                  <div>
                    <div className="font-medium">{schedule.schedule.name}</div>
                    {schedule.schedule.description && (
                      <div className="text-xs text-muted-foreground truncate max-w-xs">
                        {schedule.schedule.description}
                      </div>
                    )}
                  </div>
                </td>
                <td className="px-4 py-3 text-sm text-center">
                  <span className="font-semibold">{weeklyHours.toFixed(0)}h</span>
                </td>
                <td className="px-4 py-3 text-sm text-center">
                  <span className="text-muted-foreground">{activeDays}</span>
                  <span className="text-xs text-muted-foreground/70 ml-1">
                    ({schedule.days.length} days)
                  </span>
                </td>
                <td className="px-4 py-3 text-sm text-center">
                  {schedule.schedule.is_default ? (
                    <Badge variant="secondary" className="bg-green-100 text-green-800 border-green-200">
                      Default
                    </Badge>
                  ) : (
                    <span className="text-muted-foreground/50">-</span>
                  )}
                </td>
                <td className="px-4 py-3 text-sm text-right">
                  <div className="flex items-center justify-end gap-2">
                    <Button variant="outline" size="sm" onClick={() => onEdit(schedule)}>
                      Edit
                    </Button>
                    <Button variant="outline" size="sm" onClick={() => onAssign(schedule)}>
                      Assign
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="text-destructive hover:text-destructive hover:bg-destructive/10 border-destructive/50"
                      onClick={() => onDelete(schedule)}
                      disabled={schedule.schedule.is_default}
                      title={schedule.schedule.is_default ? 'Cannot delete default schedule' : 'Delete schedule'}
                    >
                      Delete
                    </Button>
                  </div>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};
