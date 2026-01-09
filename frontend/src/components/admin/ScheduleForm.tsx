/**
 * Schedule Form Component
 *
 * Form for creating and editing work schedules.
 * Integrates DayGrid for visual day configuration.
 */

import { useState, useEffect } from 'react';
import type { FC, FormEvent } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Switch } from '../ui/switch';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import { DayGrid, createStandardWorkweek } from './DayGrid';
import type { DayRowData } from './DayRow';
import type { WorkScheduleWithDays, CreateScheduleRequest, DayConfig } from '../../types/schedule';

export interface ScheduleFormProps {
  schedule?: WorkScheduleWithDays | null;
  onSubmit: (data: CreateScheduleRequest) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string;
  variant?: 'card' | 'sheet';
}

interface FormData {
  name: string;
  description: string;
  is_default: boolean;
  days: DayRowData[];
}

interface FormErrors {
  name?: string;
  days?: string;
}

/**
 * Convert API schedule days to form DayRowData format
 */
function apiDaysToFormDays(schedule: WorkScheduleWithDays | null | undefined): DayRowData[] {
  if (!schedule?.days) {
    return createStandardWorkweek();
  }

  // Create a map of existing days
  const dayMap = new Map(
    schedule.days.map((d) => [
      d.day_of_week,
      {
        day_of_week: d.day_of_week,
        active: true,
        start_time: d.start_time.slice(0, 5), // "09:00:00" -> "09:00"
        end_time: d.end_time.slice(0, 5),
        break_minutes: d.break_minutes,
      },
    ])
  );

  // Fill in all 7 days
  return Array.from({ length: 7 }, (_, i) =>
    dayMap.get(i) || {
      day_of_week: i,
      active: false,
      start_time: '09:00',
      end_time: '18:00',
      break_minutes: 60,
    }
  );
}

/**
 * Convert form days to API DayConfig format
 */
function formDaysToApiDays(days: DayRowData[]): DayConfig[] {
  return days
    .filter((d) => d.active)
    .map((d) => ({
      day_of_week: d.day_of_week,
      start_time: d.start_time,
      end_time: d.end_time,
      break_minutes: d.break_minutes,
    }));
}

export const ScheduleForm: FC<ScheduleFormProps> = ({
  schedule,
  onSubmit,
  onCancel,
  isLoading,
  error,
  variant = 'card',
}) => {
  const isEditing = !!schedule;

  const [formData, setFormData] = useState<FormData>({
    name: '',
    description: '',
    is_default: false,
    days: createStandardWorkweek(),
  });

  const [errors, setErrors] = useState<FormErrors>({});

  useEffect(() => {
    if (schedule) {
      setFormData({
        name: schedule.schedule.name,
        description: schedule.schedule.description || '',
        is_default: schedule.schedule.is_default,
        days: apiDaysToFormDays(schedule),
      });
    } else {
      setFormData({
        name: '',
        description: '',
        is_default: false,
        days: createStandardWorkweek(),
      });
    }
  }, [schedule]);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Schedule name is required';
    }

    const activeDays = formData.days.filter((d) => d.active);
    if (activeDays.length === 0) {
      newErrors.days = 'At least one working day is required';
    }

    // Validate time ranges
    for (const day of activeDays) {
      if (!day.start_time || !day.end_time) {
        newErrors.days = 'All active days must have start and end times';
        break;
      }
      if (day.start_time >= day.end_time) {
        newErrors.days = 'End time must be after start time';
        break;
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    const data: CreateScheduleRequest = {
      name: formData.name.trim(),
      is_default: formData.is_default,
      days: formDaysToApiDays(formData.days),
    };

    if (formData.description.trim()) {
      data.description = formData.description.trim();
    }

    await onSubmit(data);
  };

  const handleChange = (field: keyof Omit<FormData, 'days'>, value: string | boolean) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const handleDaysChange = (days: DayRowData[]) => {
    setFormData((prev) => ({ ...prev, days }));
    if (errors.days) {
      setErrors((prev) => ({ ...prev, days: undefined }));
    }
  };

  const formContent = (
    <>
      {error && (
        <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
          {error}
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="schedule-name">Schedule Name *</Label>
        <Input
          id="schedule-name"
          type="text"
          value={formData.name}
          onChange={(e) => handleChange('name', e.target.value)}
          error={errors.name}
          disabled={isLoading}
          placeholder="e.g., 35h Standard, Part-time"
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="schedule-description">Description</Label>
        <Input
          id="schedule-description"
          type="text"
          value={formData.description}
          onChange={(e) => handleChange('description', e.target.value)}
          disabled={isLoading}
          placeholder="Optional description"
        />
      </div>

      <div className="flex items-center justify-between py-2">
        <div className="space-y-0.5">
          <Label htmlFor="schedule-default">Default Schedule</Label>
          <p className="text-xs text-muted-foreground">
            Assign to new employees automatically
          </p>
        </div>
        <Switch
          id="schedule-default"
          checked={formData.is_default}
          onCheckedChange={(checked) => handleChange('is_default', checked)}
          disabled={isLoading}
        />
      </div>

      <div className="space-y-2 pt-2">
        <Label>Working Days *</Label>
        {errors.days && (
          <p className="text-sm text-destructive">{errors.days}</p>
        )}
        <div className="border rounded-lg p-4">
          <DayGrid
            days={formData.days}
            onChange={handleDaysChange}
            disabled={isLoading}
          />
        </div>
      </div>
    </>
  );

  const formButtons = (
    <>
      <Button type="button" variant="outline" onClick={onCancel} disabled={isLoading}>
        Cancel
      </Button>
      <Button type="submit" disabled={isLoading}>
        {isLoading ? (isEditing ? 'Saving...' : 'Creating...') : isEditing ? 'Save' : 'Create'}
      </Button>
    </>
  );

  // Sheet variant
  if (variant === 'sheet') {
    return (
      <form onSubmit={handleSubmit} className="flex flex-col gap-4 py-4">
        <div className="space-y-4">{formContent}</div>
        <div className="flex justify-end gap-4 pt-4 border-t">{formButtons}</div>
      </form>
    );
  }

  // Default Card variant
  return (
    <Card className="w-full max-w-2xl">
      <CardHeader>
        <CardTitle>{isEditing ? 'Edit Schedule' : 'Create Schedule'}</CardTitle>
        <CardDescription>
          {isEditing
            ? 'Update schedule configuration'
            : 'Create a new work schedule template.'}
        </CardDescription>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">{formContent}</CardContent>
        <CardFooter className="flex justify-end gap-4">{formButtons}</CardFooter>
      </form>
    </Card>
  );
};
