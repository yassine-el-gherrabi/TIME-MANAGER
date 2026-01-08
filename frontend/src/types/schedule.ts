/**
 * Work Schedule Types
 *
 * TypeScript type definitions for work schedule management.
 */

/**
 * Work schedule from API
 */
export interface WorkSchedule {
  id: string;
  organization_id: string;
  name: string;
  description: string | null;
  is_default: boolean;
  created_at: string;
  updated_at: string;
}

/**
 * Work schedule day (e.g., Monday 9:00 - 18:00)
 */
export interface WorkScheduleDay {
  id: string;
  work_schedule_id: string;
  day_of_week: number; // 0 = Monday, 6 = Sunday
  start_time: string; // "09:00:00"
  end_time: string; // "18:00:00"
  break_minutes: number;
}

/**
 * Work schedule with its days
 */
export interface WorkScheduleWithDays {
  schedule: WorkSchedule;
  days: WorkScheduleDay[];
}

/**
 * Day configuration for creating schedule
 */
export interface DayConfig {
  day_of_week: number;
  start_time: string;
  end_time: string;
  break_minutes: number;
}

/**
 * Create schedule request
 */
export interface CreateScheduleRequest {
  name: string;
  description?: string;
  is_default: boolean;
  days: DayConfig[];
}

/**
 * Update schedule request
 */
export interface UpdateScheduleRequest {
  name?: string;
  description?: string | null;
  is_default?: boolean;
}

/**
 * Add day request
 */
export interface AddDayRequest {
  day_of_week: number;
  start_time: string;
  end_time: string;
  break_minutes: number;
}

/**
 * Update day request
 */
export interface UpdateDayRequest {
  start_time?: string;
  end_time?: string;
  break_minutes?: number;
}

/**
 * Assign schedule request
 */
export interface AssignScheduleRequest {
  schedule_id: string;
}

/**
 * Day of week labels
 */
export const DAY_LABELS = [
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
  'Sunday',
] as const;
