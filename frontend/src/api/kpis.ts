/**
 * KPIs API Client
 *
 * API methods for KPI and statistics operations.
 */

import { apiRequest } from './client';
import { KPI_ENDPOINTS } from '../config/constants';
import type {
  UserKPIs,
  TeamKPIs,
  OrgKPIs,
  PresenceOverview,
  ChartData,
  KPIQueryParams,
  ChartQueryParams,
} from '../types/kpi';

/**
 * KPIs API methods
 */
export const kpisApi = {
  /**
   * Get current user's KPIs
   *
   * @param params - Query parameters for date range
   * @returns User's KPI data
   */
  getMyKpis: async (params: KPIQueryParams = {}): Promise<UserKPIs> => {
    const queryParams = new URLSearchParams();

    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${KPI_ENDPOINTS.MY_KPIS}?${queryString}` : KPI_ENDPOINTS.MY_KPIS;

    return apiRequest<UserKPIs>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get a specific user's KPIs (Manager+)
   *
   * @param userId - User ID
   * @param params - Query parameters for date range
   * @returns User's KPI data
   */
  getUserKpis: async (userId: string, params: KPIQueryParams = {}): Promise<UserKPIs> => {
    const queryParams = new URLSearchParams();

    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }

    const queryString = queryParams.toString();
    const baseUrl = KPI_ENDPOINTS.USER_KPIS(userId);
    const url = queryString ? `${baseUrl}?${queryString}` : baseUrl;

    return apiRequest<UserKPIs>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get a team's KPIs (Manager+)
   *
   * @param teamId - Team ID
   * @param params - Query parameters for date range
   * @returns Team's KPI data
   */
  getTeamKpis: async (teamId: string, params: KPIQueryParams = {}): Promise<TeamKPIs> => {
    const queryParams = new URLSearchParams();

    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }

    const queryString = queryParams.toString();
    const baseUrl = KPI_ENDPOINTS.TEAM_KPIS(teamId);
    const url = queryString ? `${baseUrl}?${queryString}` : baseUrl;

    return apiRequest<TeamKPIs>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get organization-wide KPIs (Admin+)
   *
   * @param params - Query parameters for date range
   * @returns Organization's KPI data
   */
  getOrgKpis: async (params: KPIQueryParams = {}): Promise<OrgKPIs> => {
    const queryParams = new URLSearchParams();

    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${KPI_ENDPOINTS.ORG_KPIS}?${queryString}` : KPI_ENDPOINTS.ORG_KPIS;

    return apiRequest<OrgKPIs>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get real-time presence overview (Manager+)
   *
   * @returns Presence overview data
   */
  getPresence: async (): Promise<PresenceOverview> => {
    return apiRequest<PresenceOverview>({
      method: 'GET',
      url: KPI_ENDPOINTS.PRESENCE,
    });
  },

  /**
   * Get chart data for hours worked
   *
   * @param params - Query parameters for chart data
   * @returns Chart data
   */
  getCharts: async (params: ChartQueryParams = {}): Promise<ChartData> => {
    const queryParams = new URLSearchParams();

    if (params.start_date) {
      queryParams.set('start_date', params.start_date);
    }
    if (params.end_date) {
      queryParams.set('end_date', params.end_date);
    }
    if (params.user_id) {
      queryParams.set('user_id', params.user_id);
    }
    if (params.granularity) {
      queryParams.set('granularity', params.granularity);
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${KPI_ENDPOINTS.CHARTS}?${queryString}` : KPI_ENDPOINTS.CHARTS;

    return apiRequest<ChartData>({
      method: 'GET',
      url,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  getMyKpis,
  getUserKpis,
  getTeamKpis,
  getOrgKpis,
  getPresence,
  getCharts,
} = kpisApi;
