/**
 * Break System API Client
 *
 * Handles all break policy, window, and entry API operations.
 */

import { apiClient } from './client';
import { BREAK_ENDPOINTS } from '../config/constants';
import type {
  BreakPolicyResponse,
  CreateBreakPolicyRequest,
  UpdateBreakPolicyRequest,
  BreakPolicyFilter,
  PaginatedBreakPolicies,
  BreakWindowResponse,
  CreateBreakWindowRequest,
  BreakEntryResponse,
  StartBreakRequest,
  EndBreakRequest,
  BreakEntryFilter,
  PaginatedBreakEntries,
  BreakStatus,
  EffectiveBreakPolicy,
} from '../types';

// ============================================================================
// Break Policies API
// ============================================================================

interface ListPoliciesParams extends BreakPolicyFilter {
  page?: number;
  per_page?: number;
}

export async function listBreakPolicies(
  params: ListPoliciesParams = {}
): Promise<PaginatedBreakPolicies> {
  const response = await apiClient.get<PaginatedBreakPolicies>(BREAK_ENDPOINTS.POLICIES, {
    params: {
      ...(params.team_id && { team_id: params.team_id }),
      ...(params.user_id && { user_id: params.user_id }),
      ...(params.tracking_mode && { tracking_mode: params.tracking_mode }),
      ...(params.is_active !== undefined && { is_active: params.is_active }),
      page: params.page ?? 1,
      per_page: params.per_page ?? 20,
    },
  });
  return response.data;
}

export async function getBreakPolicy(id: string): Promise<BreakPolicyResponse> {
  const response = await apiClient.get<BreakPolicyResponse>(BREAK_ENDPOINTS.GET_POLICY(id));
  return response.data;
}

export async function createBreakPolicy(
  data: CreateBreakPolicyRequest
): Promise<BreakPolicyResponse> {
  const response = await apiClient.post<BreakPolicyResponse>(BREAK_ENDPOINTS.CREATE_POLICY, data);
  return response.data;
}

export async function updateBreakPolicy(
  id: string,
  data: UpdateBreakPolicyRequest
): Promise<BreakPolicyResponse> {
  const response = await apiClient.put<BreakPolicyResponse>(BREAK_ENDPOINTS.UPDATE_POLICY(id), data);
  return response.data;
}

export async function deleteBreakPolicy(id: string): Promise<void> {
  await apiClient.delete(BREAK_ENDPOINTS.DELETE_POLICY(id));
}

// ============================================================================
// Break Windows API
// ============================================================================

export async function getBreakWindows(policyId: string): Promise<BreakWindowResponse[]> {
  const response = await apiClient.get<BreakWindowResponse[]>(
    BREAK_ENDPOINTS.GET_WINDOWS(policyId)
  );
  return response.data;
}

export async function addBreakWindow(
  policyId: string,
  data: CreateBreakWindowRequest
): Promise<BreakWindowResponse> {
  const response = await apiClient.post<BreakWindowResponse>(
    BREAK_ENDPOINTS.ADD_WINDOW(policyId),
    data
  );
  return response.data;
}

export async function deleteBreakWindow(policyId: string, windowId: string): Promise<void> {
  await apiClient.delete(BREAK_ENDPOINTS.DELETE_WINDOW(policyId, windowId));
}

// ============================================================================
// Break Entries API (for explicit tracking)
// ============================================================================

interface ListEntriesParams extends BreakEntryFilter {
  page?: number;
  per_page?: number;
}

export async function listBreakEntries(
  params: ListEntriesParams = {}
): Promise<PaginatedBreakEntries> {
  const response = await apiClient.get<PaginatedBreakEntries>(BREAK_ENDPOINTS.ENTRIES, {
    params: {
      ...(params.user_id && { user_id: params.user_id }),
      ...(params.clock_entry_id && { clock_entry_id: params.clock_entry_id }),
      ...(params.start_date && { start_date: params.start_date }),
      ...(params.end_date && { end_date: params.end_date }),
      page: params.page ?? 1,
      per_page: params.per_page ?? 20,
    },
  });
  return response.data;
}

export async function startBreak(
  clockEntryId: string,
  data: StartBreakRequest = {}
): Promise<BreakEntryResponse> {
  const response = await apiClient.post<BreakEntryResponse>(
    BREAK_ENDPOINTS.START_BREAK(clockEntryId),
    data
  );
  return response.data;
}

export async function endBreak(data: EndBreakRequest = {}): Promise<BreakEntryResponse> {
  const response = await apiClient.post<BreakEntryResponse>(BREAK_ENDPOINTS.END_BREAK, data);
  return response.data;
}

// ============================================================================
// Break Status API
// ============================================================================

export async function getBreakStatus(): Promise<BreakStatus> {
  const response = await apiClient.get<BreakStatus>(BREAK_ENDPOINTS.STATUS);
  return response.data;
}

export async function getEffectiveBreakPolicy(): Promise<EffectiveBreakPolicy | null> {
  const response = await apiClient.get<EffectiveBreakPolicy | null>(BREAK_ENDPOINTS.EFFECTIVE);
  return response.data;
}
