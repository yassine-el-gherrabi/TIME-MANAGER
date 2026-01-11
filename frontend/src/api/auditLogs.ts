/**
 * Audit Logs API Client
 *
 * API methods for audit log operations (Super Admin only).
 */

import { apiRequest } from './client';
import { AUDIT_ENDPOINTS, API_BASE_URL, API_VERSION } from '../config/constants';
import type { PaginatedAuditLogs, AuditLogFilter } from '../types/audit';
import { useAuthStore } from '../stores/authStore';

/**
 * Build query string from filter params
 */
const buildQueryString = (params: AuditLogFilter): string => {
  const queryParams = new URLSearchParams();

  if (params.page !== undefined) {
    queryParams.set('page', params.page.toString());
  }
  if (params.per_page !== undefined) {
    queryParams.set('per_page', params.per_page.toString());
  }
  if (params.entity_type !== undefined) {
    queryParams.set('entity_type', params.entity_type);
  }
  if (params.action !== undefined) {
    queryParams.set('action', params.action);
  }
  if (params.user_id !== undefined) {
    queryParams.set('user_id', params.user_id);
  }
  if (params.entity_id !== undefined) {
    queryParams.set('entity_id', params.entity_id);
  }
  if (params.start_date !== undefined) {
    queryParams.set('start_date', params.start_date);
  }
  if (params.end_date !== undefined) {
    queryParams.set('end_date', params.end_date);
  }
  if (params.organization_id !== undefined) {
    queryParams.set('organization_id', params.organization_id);
  }

  return queryParams.toString();
};

/**
 * Audit Logs API methods
 */
export const auditLogsApi = {
  /**
   * Get paginated audit logs with filters
   *
   * @param params - Filter and pagination parameters
   * @returns Paginated audit logs
   */
  list: async (params: AuditLogFilter = {}): Promise<PaginatedAuditLogs> => {
    const queryString = buildQueryString(params);
    const url = queryString
      ? `${AUDIT_ENDPOINTS.LIST}?${queryString}`
      : AUDIT_ENDPOINTS.LIST;

    return apiRequest<PaginatedAuditLogs>({
      method: 'GET',
      url,
    });
  },

  /**
   * Export audit logs as CSV
   *
   * @param params - Filter parameters (no pagination)
   * @returns Blob containing CSV data
   */
  exportCsv: async (params: Omit<AuditLogFilter, 'page' | 'per_page'> = {}): Promise<void> => {
    const queryString = buildQueryString(params);
    const url = queryString
      ? `${AUDIT_ENDPOINTS.EXPORT}?${queryString}`
      : AUDIT_ENDPOINTS.EXPORT;

    // Get access token for authorization
    const accessToken = useAuthStore.getState().accessToken;
    const fullUrl = `${API_BASE_URL}${API_VERSION}${url}`;

    const response = await fetch(fullUrl, {
      method: 'GET',
      headers: {
        ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
      },
      credentials: 'include',
    });

    if (!response.ok) {
      throw new Error(`Export failed: ${response.statusText}`);
    }

    // Get filename from Content-Disposition header or use default
    const contentDisposition = response.headers.get('Content-Disposition');
    let filename = 'audit_logs.csv';
    if (contentDisposition) {
      const match = contentDisposition.match(/filename="?([^"]+)"?/);
      if (match) {
        filename = match[1];
      }
    }

    // Create blob and trigger download
    const blob = await response.blob();
    const downloadUrl = window.URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = downloadUrl;
    link.download = filename;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    window.URL.revokeObjectURL(downloadUrl);
  },
};

/**
 * Export individual methods for convenience
 */
export const { list, exportCsv } = auditLogsApi;
