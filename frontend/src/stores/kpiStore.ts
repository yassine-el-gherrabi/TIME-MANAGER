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
 * Get Monday of the week containing the given date
 */
export const getMonday = (date: Date): Date => {
  const d = new Date(date);
  const day = d.getDay();
  const diff = d.getDate() - day + (day === 0 ? -6 : 1); // Adjust when day is Sunday
  d.setDate(diff);
  d.setHours(0, 0, 0, 0);
  return d;
};

/**
 * Get Sunday of the week containing the given date
 */
export const getSunday = (date: Date): Date => {
  const monday = getMonday(date);
  const sunday = new Date(monday);
  sunday.setDate(monday.getDate() + 6);
  sunday.setHours(23, 59, 59, 999);
  return sunday;
};

/**
 * Get the week range (Monday to Sunday) for a given date
 */
export const getWeekRange = (date: Date = new Date()): KPIQueryParams => {
  const monday = getMonday(date);
  const sunday = getSunday(date);
  return {
    start_date: monday.toISOString(),
    end_date: sunday.toISOString(),
  };
};

/**
 * Get the month range for a given date
 */
export const getMonthRange = (date: Date = new Date()): KPIQueryParams => {
  const start = new Date(date.getFullYear(), date.getMonth(), 1);
  const end = new Date(date.getFullYear(), date.getMonth() + 1, 0, 23, 59, 59, 999);
  return {
    start_date: start.toISOString(),
    end_date: end.toISOString(),
  };
};

/**
 * Navigate to previous/next period
 */
export const navigatePeriod = (
  currentDate: Date,
  period: 'week' | 'month',
  direction: 'prev' | 'next'
): Date => {
  const newDate = new Date(currentDate);
  if (period === 'week') {
    newDate.setDate(newDate.getDate() + (direction === 'next' ? 7 : -7));
  } else {
    newDate.setMonth(newDate.getMonth() + (direction === 'next' ? 1 : -1));
  }
  return newDate;
};

/**
 * Format period label
 */
export const formatPeriodLabel = (date: Date, period: 'week' | 'month'): string => {
  if (period === 'week') {
    const monday = getMonday(date);
    const sunday = getSunday(date);
    const formatDate = (d: Date) => d.toLocaleDateString('en-US', { month: 'short', day: 'numeric' });
    return `${formatDate(monday)} - ${formatDate(sunday)}`;
  } else {
    return date.toLocaleDateString('en-US', { month: 'long', year: 'numeric' });
  }
};

/**
 * Helper to get date range for common periods (legacy, kept for backwards compatibility)
 */
export const getDateRange = (period: 'week' | 'month' | 'quarter' | 'year'): KPIQueryParams => {
  const now = new Date();

  switch (period) {
    case 'week':
      return getWeekRange(now);
    case 'month':
      return getMonthRange(now);
    case 'quarter': {
      const start = new Date(now);
      start.setMonth(now.getMonth() - 3);
      return {
        start_date: start.toISOString(),
        end_date: now.toISOString(),
      };
    }
    case 'year': {
      const start = new Date(now);
      start.setFullYear(now.getFullYear() - 1);
      return {
        start_date: start.toISOString(),
        end_date: now.toISOString(),
      };
    }
  }
};
