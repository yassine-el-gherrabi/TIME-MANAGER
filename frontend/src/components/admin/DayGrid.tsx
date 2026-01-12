/**
 * Day Grid Component
 *
 * Container for 7-day schedule configuration.
 * Calculates and displays weekly total hours.
 */

import { useMemo } from 'react';
import type { FC } from 'react';
import { DayRow } from './DayRow';
import type { DayRowData } from './DayRow';

export interface DayGridProps {
  days: DayRowData[];
  onChange: (days: DayRowData[]) => void;
  disabled?: boolean;
}

/**
 * Calculate hours between two time strings (HH:MM)
 */
function calculateHours(start: string, end: string): number {
  if (!start || !end) return 0;
  const [startH, startM] = start.split(':').map(Number);
  const [endH, endM] = end.split(':').map(Number);
  const startMinutes = startH * 60 + startM;
  const endMinutes = endH * 60 + endM;
  return Math.max(0, (endMinutes - startMinutes) / 60);
}

/**
 * Create default days array with all days inactive
 */
export function createDefaultDays(): DayRowData[] {
  return Array.from({ length: 7 }, (_, i) => ({
    day_of_week: i,
    active: false,
    start_time: '09:00',
    end_time: '18:00',
    break_minutes: 60,
  }));
}

/**
 * Create days array for a standard workweek (Mon-Fri 9-18)
 */
export function createStandardWorkweek(): DayRowData[] {
  return Array.from({ length: 7 }, (_, i) => ({
    day_of_week: i,
    active: i < 5, // Monday to Friday
    start_time: '09:00',
    end_time: '18:00',
    break_minutes: 60,
  }));
}

export const DayGrid: FC<DayGridProps> = ({ days, onChange, disabled = false }) => {
  // Ensure we always have 7 days in order
  const normalizedDays = useMemo(() => {
    const dayMap = new Map(days.map((d) => [d.day_of_week, d]));
    return Array.from({ length: 7 }, (_, i) =>
      dayMap.get(i) || {
        day_of_week: i,
        active: false,
        start_time: '09:00',
        end_time: '18:00',
        break_minutes: 60,
      }
    );
  }, [days]);

  // Calculate weekly totals
  const { totalHours, totalBreak, netHours, activeDays } = useMemo(() => {
    let total = 0;
    let breakTotal = 0;
    let active = 0;

    normalizedDays.forEach((day) => {
      if (day.active) {
        const dayHours = calculateHours(day.start_time, day.end_time);
        total += dayHours;
        breakTotal += day.break_minutes / 60;
        active++;
      }
    });

    return {
      totalHours: total,
      totalBreak: breakTotal,
      netHours: total - breakTotal,
      activeDays: active,
    };
  }, [normalizedDays]);

  const handleDayChange = (updatedDay: DayRowData) => {
    const newDays = normalizedDays.map((day) =>
      day.day_of_week === updatedDay.day_of_week ? updatedDay : day
    );
    onChange(newDays);
  };

  return (
    <div className="space-y-2">
      {/* Header */}
      <div className="grid grid-cols-[120px_60px_1fr_1fr_80px] gap-3 items-center px-3 pb-2 border-b">
        <div className="text-xs font-medium text-muted-foreground uppercase">Day</div>
        <div className="text-xs font-medium text-muted-foreground uppercase text-center">Active</div>
        <div className="text-xs font-medium text-muted-foreground uppercase">Start</div>
        <div className="text-xs font-medium text-muted-foreground uppercase">End</div>
        <div className="text-xs font-medium text-muted-foreground uppercase">Break</div>
      </div>

      {/* Day rows */}
      <div className="space-y-1">
        {normalizedDays.map((day) => (
          <DayRow
            key={day.day_of_week}
            data={day}
            onChange={handleDayChange}
            disabled={disabled}
          />
        ))}
      </div>

      {/* Summary */}
      <div className="flex items-center justify-between pt-3 mt-2 border-t">
        <div className="text-sm text-muted-foreground">
          <span className="font-medium">{activeDays}</span> working day{activeDays !== 1 ? 's' : ''}
        </div>
        <div className="text-sm">
          <span className="text-muted-foreground">Total: </span>
          <span className="font-semibold">{netHours.toFixed(1)}h</span>
          <span className="text-muted-foreground text-xs ml-1">
            ({totalHours.toFixed(1)}h - {totalBreak.toFixed(1)}h break)
          </span>
        </div>
      </div>
    </div>
  );
};
