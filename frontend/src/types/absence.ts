/**
 * Absence Management Types
 *
 * TypeScript type definitions for absence/leave management.
 */

/**
 * Absence status enum
 */
export enum AbsenceStatus {
  Pending = 'pending',
  Approved = 'approved',
  Rejected = 'rejected',
  Cancelled = 'cancelled',
}

/**
 * Absence type configuration
 */
export interface AbsenceType {
  id: string;
  name: string;
  code: string;
  color: string;
  requires_approval: boolean;
  affects_balance: boolean;
  is_paid: boolean;
  created_at: string;
  updated_at: string;
}

/**
 * Absence record
 */
export interface Absence {
  id: string;
  organization_id: string;
  organization_name: string;
  user_id: string;
  user_name: string;
  user_email: string;
  team_id: string | null;
  team_name: string | null;
  type_id: string;
  type_name: string;
  type_code: string;
  type_color: string;
  start_date: string;
  end_date: string;
  days_count: number;
  status: AbsenceStatus;
  reason: string | null;
  rejection_reason: string | null;
  approved_by: string | null;
  approver_name: string | null;
  approved_at: string | null;
  created_at: string;
}

/**
 * Leave balance for a specific absence type
 */
export interface LeaveBalance {
  id: string;
  user_id: string;
  absence_type_id: string;
  type_name: string;
  type_code: string;
  type_color: string;
  year: number;
  initial_balance: number;
  used: number;
  adjustment: number;
  remaining: number;
}

/**
 * Closed day definition (company holidays, office closures)
 */
export interface ClosedDay {
  id: string;
  name: string;
  date: string;
  is_recurring: boolean;
  created_at: string;
}

/**
 * Paginated absences response
 */
export interface PaginatedAbsences {
  data: Absence[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

/**
 * Create absence type request
 */
export interface CreateAbsenceTypeRequest {
  name: string;
  code: string;
  color?: string;
  requires_approval?: boolean;
  affects_balance?: boolean;
  is_paid?: boolean;
}

/**
 * Update absence type request
 */
export interface UpdateAbsenceTypeRequest {
  name?: string;
  code?: string;
  color?: string;
  requires_approval?: boolean;
  affects_balance?: boolean;
  is_paid?: boolean;
}

/**
 * Create absence request
 */
export interface CreateAbsenceRequest {
  type_id: string;
  start_date: string;
  end_date: string;
  reason?: string;
}

/**
 * Reject absence request
 */
export interface RejectAbsenceRequest {
  reason?: string;
}

/**
 * Set balance request
 */
export interface SetBalanceRequest {
  absence_type_id: string;
  year: number;
  initial_balance: number;
}

/**
 * Adjust balance request
 */
export interface AdjustBalanceRequest {
  adjustment: number;
  reason?: string;
}

/**
 * Create closed day request
 */
export interface CreateClosedDayRequest {
  name: string;
  date: string;
  is_recurring?: boolean;
}

/**
 * Update closed day request
 */
export interface UpdateClosedDayRequest {
  name?: string;
  date?: string;
  is_recurring?: boolean;
}

/**
 * Absence filter options
 */
export interface AbsenceFilter {
  user_id?: string;
  type_id?: string;
  status?: AbsenceStatus;
  start_date?: string;
  end_date?: string;
  page?: number;
  per_page?: number;
}

/**
 * Pending absence filter options (for approval pages)
 */
export interface PendingAbsenceFilter {
  page?: number;
  per_page?: number;
  /** Filter by organization (SuperAdmin only) */
  organization_id?: string;
  /** Filter by team (Admin/Manager) */
  team_id?: string;
}

/**
 * Closed day filter options
 */
export interface ClosedDayFilter {
  start_date?: string;
  end_date?: string;
  is_recurring?: boolean;
  /** Filter by organization (SuperAdmin only) */
  organization_id?: string;
}

/**
 * Balance filter options
 */
export interface BalanceFilter {
  user_id?: string;
  absence_type_id?: string;
  year?: number;
}

/**
 * Status badge colors
 */
export const STATUS_COLORS: Record<AbsenceStatus, string> = {
  [AbsenceStatus.Pending]: 'bg-yellow-100 text-yellow-800',
  [AbsenceStatus.Approved]: 'bg-green-100 text-green-800',
  [AbsenceStatus.Rejected]: 'bg-red-100 text-red-800',
  [AbsenceStatus.Cancelled]: 'bg-gray-100 text-gray-800',
};

/**
 * Status labels
 */
export const STATUS_LABELS: Record<AbsenceStatus, string> = {
  [AbsenceStatus.Pending]: 'En attente',
  [AbsenceStatus.Approved]: 'Approuvée',
  [AbsenceStatus.Rejected]: 'Refusée',
  [AbsenceStatus.Cancelled]: 'Annulée',
};
