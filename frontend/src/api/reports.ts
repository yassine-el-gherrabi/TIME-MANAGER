/**
 * Reports API Client
 *
 * API methods for exporting reports (Admin+).
 */

import { apiClient } from './client';
import { REPORTS_ENDPOINTS } from '../config/constants';

/**
 * Export type options
 */
export type ExportType = 'clocks' | 'absences' | 'users';

/**
 * Export parameters
 */
export interface ExportParams {
  start_date?: string;
  end_date?: string;
  user_id?: string;
}

/**
 * Download blob as file
 */
const downloadBlob = (blob: Blob, filename: string) => {
  const url = window.URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.setAttribute('download', filename);
  document.body.appendChild(link);
  link.click();
  link.parentNode?.removeChild(link);
  window.URL.revokeObjectURL(url);
};

/**
 * Reports API methods
 */
export const reportsApi = {
  /**
   * Export data as CSV
   *
   * @param type - Type of export (clocks, absences, users)
   * @param params - Optional filter parameters
   */
  exportCsv: async (type: ExportType, params: ExportParams = {}): Promise<void> => {
    const queryParams = new URLSearchParams();
    queryParams.set('type', type);

    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }
    if (params.user_id) {
      queryParams.set('user_id', params.user_id);
    }

    const response = await apiClient.get(`${REPORTS_ENDPOINTS.EXPORT}?${queryParams.toString()}`, {
      responseType: 'blob',
    });

    // Generate filename with timestamp
    const timestamp = new Date().toISOString().replace(/[:.]/g, '-').slice(0, 19);
    const filename = `${type}_${timestamp}.csv`;

    downloadBlob(response.data, filename);
  },
};

/**
 * Export individual methods for convenience
 */
export const { exportCsv } = reportsApi;
