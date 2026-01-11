/**
 * Clock Entry Types
 *
 * TypeScript type definitions for clock in/out operations.
 */

/**
 * Clock entry status enumeration
 */
export type ClockEntryStatus = 'pending' | 'approved' | 'rejected';

/**
 * Clock history view mode
 */
export type ClockViewMode = 'list' | 'calendar';

/**
 * Clock history filter state
 */
export interface ClockFilterState {
  startDate: string | null;
  endDate: string | null;
  status: ClockEntryStatus | 'all';
}

/**
 * Clock entry from API
 */
export interface ClockEntry {
  id: string;
  organization_id: string;
  user_id: string;
  clock_in: string;
  clock_out: string | null;
  status: ClockEntryStatus;
  approved_by: string | null;
  approved_at: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

/**
 * Clock entry response with user info
 */
export interface ClockEntryResponse {
  id: string;
  organization_id: string;
  organization_name: string;
  user_id: string;
  user_name: string;
  user_email: string;
  team_id: string | null;
  team_name: string | null;
  clock_in: string;
  clock_out: string | null;
  status: ClockEntryStatus;
  approved_by: string | null;
  approver_name: string | null;
  approved_at: string | null;
  notes: string | null;
  duration_minutes: number | null;
  created_at: string;
}

/**
 * Current clock status
 */
export interface ClockStatus {
  is_clocked_in: boolean;
  current_entry: ClockEntry | null;
  elapsed_minutes: number | null;
}

/**
 * Clock in request
 */
export interface ClockInRequest {
  notes?: string;
}

/**
 * Clock out request
 */
export interface ClockOutRequest {
  notes?: string;
}

/**
 * Reject entry request
 */
export interface RejectEntryRequest {
  reason?: string;
}

/**
 * Clock history query parameters
 */
export interface ClockHistoryParams {
  page?: number;
  per_page?: number;
  start_date?: string;
  end_date?: string;
  status?: ClockEntryStatus;
}

/**
 * Paginated clock history response
 */
export interface PaginatedClockHistoryResponse {
  data: ClockEntryResponse[];
  total: number;
  page: number;
  per_page: number;
}

/**
 * Pending entries query parameters
 */
export interface PendingEntriesParams {
  page?: number;
  per_page?: number;
  /** Filter by organization (SuperAdmin only) */
  organization_id?: string;
  /** Filter by team (Admin/Manager) */
  team_id?: string;
}

/**
 * Paginated pending entries response
 */
export interface PaginatedPendingResponse {
  data: ClockEntryResponse[];
  total: number;
  page: number;
  per_page: number;
}
