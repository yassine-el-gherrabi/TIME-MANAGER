/**
 * Clock Restriction Types
 *
 * TypeScript type definitions for clock restriction configuration.
 */

/**
 * Clock restriction mode enumeration
 */
export type ClockRestrictionMode = 'strict' | 'flexible' | 'unrestricted';

/**
 * Clock override request status
 */
export type ClockOverrideStatus = 'pending' | 'approved' | 'rejected' | 'auto_approved';

/**
 * Mode display configuration
 */
export const RESTRICTION_MODE_CONFIG = {
  strict: {
    label: 'Strict',
    description: 'No override possible - clock blocked outside allowed window',
    color: 'red',
  },
  flexible: {
    label: 'Flexible',
    description: 'Override possible with justification',
    color: 'yellow',
  },
  unrestricted: {
    label: 'Unrestricted',
    description: 'No time restrictions',
    color: 'green',
  },
} as const;

/**
 * Override status display configuration
 */
export const OVERRIDE_STATUS_CONFIG = {
  pending: {
    label: 'Pending',
    color: 'yellow',
  },
  approved: {
    label: 'Approved',
    color: 'green',
  },
  rejected: {
    label: 'Rejected',
    color: 'red',
  },
  auto_approved: {
    label: 'Auto-Approved',
    color: 'blue',
  },
} as const;

/**
 * Clock restriction entity from API
 */
export interface ClockRestriction {
  id: string;
  organization_id: string;
  team_id: string | null;
  user_id: string | null;
  mode: ClockRestrictionMode;
  clock_in_earliest: string | null;
  clock_in_latest: string | null;
  clock_out_earliest: string | null;
  clock_out_latest: string | null;
  enforce_schedule: boolean;
  require_manager_approval: boolean;
  is_active: boolean;
  max_daily_clock_events: number | null;
  created_at: string;
  updated_at: string;
}

/**
 * Clock restriction response with context
 */
export interface ClockRestrictionResponse {
  id: string;
  organization_id: string;
  organization_name: string;
  team_id: string | null;
  team_name: string | null;
  user_id: string | null;
  user_name: string | null;
  mode: ClockRestrictionMode;
  clock_in_earliest: string | null;
  clock_in_latest: string | null;
  clock_out_earliest: string | null;
  clock_out_latest: string | null;
  enforce_schedule: boolean;
  require_manager_approval: boolean;
  is_active: boolean;
  max_daily_clock_events: number | null;
  created_at: string;
  updated_at: string;
}

/**
 * Effective restriction (result of cascade resolution)
 */
export interface EffectiveRestriction {
  restriction: ClockRestrictionResponse | null;
  source_level: 'user' | 'team' | 'organization' | 'default';
}

/**
 * Create clock restriction request
 */
export interface CreateClockRestrictionRequest {
  organization_id?: string; // Required for SuperAdmin
  team_id?: string | null;
  user_id?: string | null;
  mode: ClockRestrictionMode;
  clock_in_earliest?: string | null;
  clock_in_latest?: string | null;
  clock_out_earliest?: string | null;
  clock_out_latest?: string | null;
  enforce_schedule?: boolean;
  require_manager_approval?: boolean;
  max_daily_clock_events?: number | null;
}

/**
 * Update clock restriction request
 */
export interface UpdateClockRestrictionRequest {
  mode?: ClockRestrictionMode;
  clock_in_earliest?: string | null;
  clock_in_latest?: string | null;
  clock_out_earliest?: string | null;
  clock_out_latest?: string | null;
  enforce_schedule?: boolean;
  require_manager_approval?: boolean;
  is_active?: boolean;
  max_daily_clock_events?: number | null;
}

/**
 * Clock validation result
 */
export interface ClockValidationResult {
  allowed: boolean;
  mode: ClockRestrictionMode;
  message: string | null;
  can_request_override: boolean;
  restriction_source: 'user' | 'team' | 'organization' | 'default';
}

/**
 * Clock override request entity
 */
export interface ClockOverrideRequest {
  id: string;
  organization_id: string;
  user_id: string;
  clock_entry_id: string | null;
  requested_action: string;
  reason: string;
  status: ClockOverrideStatus;
  reviewed_by: string | null;
  reviewed_at: string | null;
  review_notes: string | null;
  created_at: string;
}

/**
 * Clock override request response with context
 */
export interface ClockOverrideRequestResponse {
  id: string;
  organization_id: string;
  organization_name: string;
  user_id: string;
  user_name: string;
  user_email: string;
  team_id: string | null;
  team_name: string | null;
  clock_entry_id: string | null;
  requested_action: string;
  reason: string;
  status: ClockOverrideStatus;
  reviewed_by: string | null;
  reviewer_name: string | null;
  reviewed_at: string | null;
  review_notes: string | null;
  created_at: string;
}

/**
 * Create override request
 */
export interface CreateOverrideRequest {
  requested_action: 'clock_in' | 'clock_out';
  reason: string;
}

/**
 * Review override request
 */
export interface ReviewOverrideRequest {
  approved: boolean;
  notes?: string;
}

/**
 * Clock restriction filter params
 */
export interface ClockRestrictionFilter {
  page?: number;
  per_page?: number;
  team_id?: string;
  user_id?: string;
  mode?: ClockRestrictionMode;
  is_active?: boolean;
}

/**
 * Override request filter params
 */
export interface ClockOverrideFilter {
  page?: number;
  per_page?: number;
  status?: ClockOverrideStatus;
  requested_action?: string;
}

/**
 * Paginated clock restrictions response
 */
export interface PaginatedClockRestrictions {
  data: ClockRestrictionResponse[];
  total: number;
  page: number;
  per_page: number;
}

/**
 * Paginated override requests response
 */
export interface PaginatedOverrideRequests {
  data: ClockOverrideRequestResponse[];
  total: number;
  page: number;
  per_page: number;
}
