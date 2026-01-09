/**
 * Day Row Component
 *
 * Single day configuration row for schedule editing.
 * Contains toggle, start/end time inputs, and break duration.
 */

import type { FC } from 'react';
import { Switch } from '../ui/switch';
import { Input } from '../ui/input';
import { DAY_LABELS } from '../../types/schedule';

export interface DayRowData {
  day_of_week: number;
  active: boolean;
  start_time: string;
  end_time: string;
  break_minutes: number;
}

export interface DayRowProps {
  data: DayRowData;
  onChange: (data: DayRowData) => void;
  disabled?: boolean;
}

export const DayRow: FC<DayRowProps> = ({ data, onChange, disabled = false }) => {
  const dayLabel = DAY_LABELS[data.day_of_week];

  const handleActiveChange = (checked: boolean) => {
    onChange({
      ...data,
      active: checked,
      // Reset to defaults when activating
      ...(checked && !data.start_time
        ? { start_time: '09:00', end_time: '18:00', break_minutes: 60 }
        : {}),
    });
  };

  const handleStartTimeChange = (value: string) => {
    onChange({ ...data, start_time: value });
  };

  const handleEndTimeChange = (value: string) => {
    onChange({ ...data, end_time: value });
  };

  const handleBreakChange = (value: string) => {
    const minutes = parseInt(value, 10);
    onChange({ ...data, break_minutes: isNaN(minutes) ? 0 : Math.max(0, minutes) });
  };

  const isWeekend = data.day_of_week >= 5;

  return (
    <div
      className={`grid grid-cols-[120px_60px_1fr_1fr_80px] gap-3 items-center py-2 px-3 rounded-md ${
        isWeekend ? 'bg-muted/30' : ''
      } ${!data.active ? 'opacity-60' : ''}`}
    >
      {/* Day label */}
      <div className="text-sm font-medium">{dayLabel}</div>

      {/* Active toggle */}
      <div className="flex justify-center">
        <Switch
          checked={data.active}
          onCheckedChange={handleActiveChange}
          disabled={disabled}
          aria-label={`Enable ${dayLabel}`}
        />
      </div>

      {/* Start time */}
      <Input
        type="time"
        value={data.active ? data.start_time : ''}
        onChange={(e) => handleStartTimeChange(e.target.value)}
        disabled={disabled || !data.active}
        className="h-8 text-sm"
        aria-label={`${dayLabel} start time`}
      />

      {/* End time */}
      <Input
        type="time"
        value={data.active ? data.end_time : ''}
        onChange={(e) => handleEndTimeChange(e.target.value)}
        disabled={disabled || !data.active}
        className="h-8 text-sm"
        aria-label={`${dayLabel} end time`}
      />

      {/* Break minutes */}
      <Input
        type="number"
        min={0}
        max={480}
        value={data.active ? data.break_minutes : ''}
        onChange={(e) => handleBreakChange(e.target.value)}
        disabled={disabled || !data.active}
        className="h-8 text-sm"
        placeholder="min"
        aria-label={`${dayLabel} break minutes`}
      />
    </div>
  );
};
