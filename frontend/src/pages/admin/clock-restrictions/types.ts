/**
 * Clock Restrictions Module Types
 *
 * Shared interfaces for clock restriction management components.
 */

import type { ClockRestrictionMode } from '../../../types/clockRestriction';

export interface FormData {
  organization_id: string;
  scope: 'organization' | 'team' | 'user';
  team_id: string;
  user_id: string;
  mode: ClockRestrictionMode;
  clock_in_earliest: string;
  clock_in_latest: string;
  clock_out_earliest: string;
  clock_out_latest: string;
  enforce_schedule: boolean;
  require_manager_approval: boolean;
  max_daily_clock_events: string;
}

export interface FormDrawerState {
  open: boolean;
  restriction: import('../../../types/clockRestriction').ClockRestrictionResponse | null;
  loading: boolean;
  error: string;
}

export interface DeleteDialogState {
  open: boolean;
  restriction: import('../../../types/clockRestriction').ClockRestrictionResponse | null;
  loading: boolean;
}

export const initialFormData: FormData = {
  organization_id: '',
  scope: 'organization',
  team_id: '',
  user_id: '',
  mode: 'flexible',
  clock_in_earliest: '07:00',
  clock_in_latest: '10:00',
  clock_out_earliest: '16:00',
  clock_out_latest: '22:00',
  enforce_schedule: true,
  require_manager_approval: false,
  max_daily_clock_events: '',
};
