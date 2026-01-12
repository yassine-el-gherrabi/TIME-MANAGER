// Break system types for frontend

export type BreakTrackingMode = 'auto_deduct' | 'explicit_tracking';

// ============================================================================
// Break Policy Types
// ============================================================================

export interface BreakPolicy {
  id: string;
  organization_id: string;
  team_id: string | null;
  user_id: string | null;
  name: string;
  description: string | null;
  tracking_mode: BreakTrackingMode;
  notify_missing_break: boolean;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface BreakPolicyResponse extends BreakPolicy {
  organization_name: string;
  team_name: string | null;
  user_name: string | null;
  windows: BreakWindowResponse[];
}

export interface CreateBreakPolicyRequest {
  team_id?: string | null;
  user_id?: string | null;
  name: string;
  description?: string | null;
  tracking_mode: BreakTrackingMode;
  notify_missing_break?: boolean;
  windows?: CreateBreakWindowRequest[];
}

export interface UpdateBreakPolicyRequest {
  name?: string;
  description?: string | null;
  tracking_mode?: BreakTrackingMode;
  notify_missing_break?: boolean;
  is_active?: boolean;
}

export interface BreakPolicyFilter {
  team_id?: string;
  user_id?: string;
  tracking_mode?: BreakTrackingMode;
  is_active?: boolean;
}

export interface PaginatedBreakPolicies {
  data: BreakPolicyResponse[];
  total: number;
  page: number;
  per_page: number;
}

// ============================================================================
// Break Window Types
// ============================================================================

export interface BreakWindow {
  id: string;
  break_policy_id: string;
  day_of_week: number; // 0 = Sunday, 6 = Saturday
  window_start: string; // "HH:MM" format
  window_end: string; // "HH:MM" format
  min_duration_minutes: number;
  max_duration_minutes: number;
  is_mandatory: boolean;
  created_at: string;
}

export interface BreakWindowResponse {
  id: string;
  day_of_week: number;
  window_start: string;
  window_end: string;
  min_duration_minutes: number;
  max_duration_minutes: number;
  is_mandatory: boolean;
}

export interface CreateBreakWindowRequest {
  day_of_week: number;
  window_start: string;
  window_end: string;
  min_duration_minutes: number;
  max_duration_minutes: number;
  is_mandatory?: boolean;
}

// ============================================================================
// Break Entry Types (for explicit tracking)
// ============================================================================

export interface BreakEntry {
  id: string;
  organization_id: string;
  user_id: string;
  clock_entry_id: string;
  break_start: string;
  break_end: string | null;
  duration_minutes: number | null;
  notes: string | null;
  created_at: string;
}

export interface BreakEntryResponse extends BreakEntry {
  user_name: string;
}

export interface StartBreakRequest {
  notes?: string | null;
}

export interface EndBreakRequest {
  notes?: string | null;
}

export interface BreakEntryFilter {
  user_id?: string;
  clock_entry_id?: string;
  start_date?: string;
  end_date?: string;
}

export interface PaginatedBreakEntries {
  data: BreakEntryResponse[];
  total: number;
  page: number;
  per_page: number;
}

// ============================================================================
// Break Status & Effective Policy Types
// ============================================================================

export interface BreakStatus {
  is_on_break: boolean;
  current_break: BreakEntryResponse | null;
  elapsed_minutes: number | null;
  policy: EffectiveBreakPolicy | null;
}

export interface EffectiveBreakPolicy {
  policy: BreakPolicyResponse | null;
  source_level: 'user' | 'team' | 'organization' | 'default';
}

export interface BreakDeduction {
  total_minutes: number;
  source: 'auto_deduct' | 'tracked' | 'none';
  entries: BreakEntryResponse[];
}

// ============================================================================
// UI Helper Types
// ============================================================================

export const DAYS_OF_WEEK = [
  { value: 0, label: 'Sunday' },
  { value: 1, label: 'Monday' },
  { value: 2, label: 'Tuesday' },
  { value: 3, label: 'Wednesday' },
  { value: 4, label: 'Thursday' },
  { value: 5, label: 'Friday' },
  { value: 6, label: 'Saturday' },
] as const;

export const TRACKING_MODE_OPTIONS = [
  {
    value: 'auto_deduct' as BreakTrackingMode,
    label: 'Auto Deduct',
    description: 'Breaks are automatically deducted from worked hours based on policy windows',
  },
  {
    value: 'explicit_tracking' as BreakTrackingMode,
    label: 'Explicit Tracking',
    description: 'Users must explicitly start and end their breaks',
  },
] as const;

export function getDayLabel(dayOfWeek: number): string {
  return DAYS_OF_WEEK.find((d) => d.value === dayOfWeek)?.label ?? 'Unknown';
}

export function getTrackingModeLabel(mode: BreakTrackingMode): string {
  return TRACKING_MODE_OPTIONS.find((m) => m.value === mode)?.label ?? mode;
}

export function formatBreakDuration(minutes: number | null): string {
  if (minutes === null) return '-';
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  if (hours === 0) return `${mins}min`;
  if (mins === 0) return `${hours}h`;
  return `${hours}h ${mins}min`;
}
