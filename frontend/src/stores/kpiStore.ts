/**
 * KPI Store (Zustand)
 *
 * Global state management for KPI and statistics.
 */

import { create } from 'zustand';
import { kpisApi } from '../api/kpis';
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
 * KPI store state and actions
 */
interface KPIStore {
  // State
  myKpis: UserKPIs | null;
  userKpis: Record<string, UserKPIs>;
  teamKpis: Record<string, TeamKPIs>;
  orgKpis: OrgKPIs | null;
  presence: PresenceOverview | null;
  charts: ChartData | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  fetchMyKpis: (params?: KPIQueryParams) => Promise<void>;
  fetchUserKpis: (userId: string, params?: KPIQueryParams) => Promise<void>;
  fetchTeamKpis: (teamId: string, params?: KPIQueryParams) => Promise<void>;
  fetchOrgKpis: (params?: KPIQueryParams) => Promise<void>;
  fetchPresence: () => Promise<void>;
  fetchCharts: (params?: ChartQueryParams) => Promise<void>;
  clearError: () => void;
  reset: () => void;
}

const initialState = {
  myKpis: null,
  userKpis: {},
  teamKpis: {},
  orgKpis: null,
  presence: null,
  charts: null,
  isLoading: false,
  error: null,
};

/**
 * Zustand KPI store
 */
export const useKPIStore = create<KPIStore>()((set) => ({
  ...initialState,

  /**
   * Fetch current user's KPIs
   */
  fetchMyKpis: async (params: KPIQueryParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const myKpis = await kpisApi.getMyKpis(params);
      set({ myKpis, isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch KPIs';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Fetch a specific user's KPIs (Manager+)
   */
  fetchUserKpis: async (userId: string, params: KPIQueryParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const kpis = await kpisApi.getUserKpis(userId, params);
      set((state) => ({
        userKpis: { ...state.userKpis, [userId]: kpis },
        isLoading: false,
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch user KPIs';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Fetch a team's KPIs (Manager+)
   */
  fetchTeamKpis: async (teamId: string, params: KPIQueryParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const kpis = await kpisApi.getTeamKpis(teamId, params);
      set((state) => ({
        teamKpis: { ...state.teamKpis, [teamId]: kpis },
        isLoading: false,
      }));
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch team KPIs';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Fetch organization-wide KPIs (Admin+)
   */
  fetchOrgKpis: async (params: KPIQueryParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const orgKpis = await kpisApi.getOrgKpis(params);
      set({ orgKpis, isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch organization KPIs';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Fetch real-time presence (Manager+)
   */
  fetchPresence: async () => {
    set({ isLoading: true, error: null });
    try {
      const presence = await kpisApi.getPresence();
      set({ presence, isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch presence';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Fetch chart data
   */
  fetchCharts: async (params: ChartQueryParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const charts = await kpisApi.getCharts(params);
      set({ charts, isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch charts';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Clear error message
   */
  clearError: () => {
    set({ error: null });
  },

  /**
   * Reset store to initial state
   */
  reset: () => {
    set(initialState);
  },
}));

/**
 * Initialize KPI store with user's KPIs
 */
export const initializeKPIStore = async (): Promise<void> => {
  const store = useKPIStore.getState();
  await store.fetchMyKpis();
};

/**
 * Helper to get date range for common periods
 */
export const getDateRange = (period: 'week' | 'month' | 'quarter' | 'year'): KPIQueryParams => {
  const now = new Date();
  let start: Date;

  switch (period) {
    case 'week':
      start = new Date(now);
      start.setDate(now.getDate() - 7);
      break;
    case 'month':
      start = new Date(now);
      start.setMonth(now.getMonth() - 1);
      break;
    case 'quarter':
      start = new Date(now);
      start.setMonth(now.getMonth() - 3);
      break;
    case 'year':
      start = new Date(now);
      start.setFullYear(now.getFullYear() - 1);
      break;
  }

  return {
    start_date: start.toISOString(),
    end_date: now.toISOString(),
  };
};
