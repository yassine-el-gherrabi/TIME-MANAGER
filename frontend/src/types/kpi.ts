/**
 * KPI Types
 *
 * TypeScript type definitions for KPI stats and dashboards.
 */

/**
 * User KPIs
 */
export interface UserKPIs {
  user_id: string;
  user_name: string;
  total_hours_worked: number;
  theoretical_hours: number;
  hours_variance: number;
  punctuality_rate: number;
  days_worked: number;
  days_late: number;
  average_daily_hours: number;
}

/**
 * Team member KPI summary
 */
export interface MemberKPISummary {
  user_id: string;
  user_name: string;
  hours_worked: number;
  punctuality_rate: number;
  is_clocked_in: boolean;
}

/**
 * Team KPIs
 */
export interface TeamKPIs {
  team_id: string;
  team_name: string;
  member_count: number;
  total_hours: number;
  average_punctuality: number;
  currently_clocked_in: number;
  members: MemberKPISummary[];
}

/**
 * Organization KPIs
 */
export interface OrgKPIs {
  total_employees: number;
  total_hours: number;
  average_punctuality: number;
  currently_clocked_in: number;
  attendance_rate: number;
}

/**
 * Present user info
 */
export interface PresentUser {
  user_id: string;
  user_name: string;
  clock_in_time: string;
  elapsed_minutes: number;
}

/**
 * Real-time presence overview
 */
export interface PresenceOverview {
  total_employees: number;
  currently_present: number;
  present_users: PresentUser[];
}

/**
 * Chart data point
 */
export interface ChartDataPoint {
  date: string;
  hours_worked: number;
  theoretical_hours: number;
}

/**
 * Chart data response
 */
export interface ChartData {
  data: ChartDataPoint[];
  granularity: 'day' | 'week' | 'month';
}

/**
 * KPI query parameters
 */
export interface KPIQueryParams {
  start_date?: string;
  end_date?: string;
}

/**
 * Chart query parameters
 */
export interface ChartQueryParams {
  start_date?: string;
  end_date?: string;
  user_id?: string;
  granularity?: 'day' | 'week' | 'month';
}

/**
 * Date range helper
 */
export interface DateRange {
  start: Date;
  end: Date;
}
