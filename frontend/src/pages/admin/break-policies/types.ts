/**
 * Shared types for Break Policies page components.
 */

import type {
  BreakPolicyResponse,
  BreakTrackingMode,
} from '../../../types/break';

export interface FormData {
  scope: 'organization' | 'team' | 'user';
  team_id: string;
  user_id: string;
  name: string;
  description: string;
  tracking_mode: BreakTrackingMode;
  notify_missing_break: boolean;
}

export interface WindowFormData {
  selectedDays: number[];
  window_start: string;
  window_end: string;
  min_duration_minutes: number;
  max_duration_minutes: number;
  is_mandatory: boolean;
}

export interface FormDrawerState {
  open: boolean;
  policy: BreakPolicyResponse | null;
  loading: boolean;
  error: string;
}

export interface WindowDrawerState {
  open: boolean;
  policy: BreakPolicyResponse | null;
  loading: boolean;
}

export interface DeleteDialogState {
  open: boolean;
  policy: BreakPolicyResponse | null;
  loading: boolean;
}

export const initialFormData: FormData = {
  scope: 'organization',
  team_id: '',
  user_id: '',
  name: '',
  description: '',
  tracking_mode: 'auto_deduct',
  notify_missing_break: false,
};

export const initialWindowFormData: WindowFormData = {
  selectedDays: [],
  window_start: '12:00',
  window_end: '14:00',
  min_duration_minutes: 30,
  max_duration_minutes: 60,
  is_mandatory: true,
};
