/**
 * Clock Store (Zustand)
 *
 * Global state management for clock in/out operations.
 */

import { create } from 'zustand';
import { clocksApi } from '../api/clocks';
import type {
  ClockEntry,
  ClockStatus,
  ClockEntryResponse,
  ClockHistoryParams,
  PendingEntriesParams,
  ClockFilterState,
} from '../types/clock';

/**
 * Clock store state and actions
 */
interface ClockStore {
  // State
  status: ClockStatus | null;
  history: ClockEntryResponse[];
  historyTotal: number;
  historyPage: number;
  historyFilters: ClockFilterState;
  pendingEntries: ClockEntryResponse[];
  pendingTotal: number;
  pendingPage: number;
  isLoading: boolean;
  isClockingIn: boolean;
  isClockingOut: boolean;
  error: string | null;

  // Actions
  fetchStatus: () => Promise<void>;
  clockIn: () => Promise<ClockEntry>;
  clockOut: (notes?: string) => Promise<ClockEntry>;
  fetchHistory: (params?: ClockHistoryParams) => Promise<void>;
  fetchFilteredHistory: (page?: number) => Promise<void>;
  setHistoryPage: (page: number) => void;
  setHistoryFilters: (filters: Partial<ClockFilterState>) => void;
  resetHistoryFilters: () => void;
  fetchPendingEntries: (params?: PendingEntriesParams) => Promise<void>;
  setPendingPage: (page: number) => void;
  approveEntry: (id: string) => Promise<void>;
  rejectEntry: (id: string, reason?: string) => Promise<void>;
  clearError: () => void;
  reset: () => void;
}

const initialFilters: ClockFilterState = {
  startDate: null,
  endDate: null,
  status: 'all',
};

const initialState = {
  status: null,
  history: [],
  historyTotal: 0,
  historyPage: 1,
  historyFilters: initialFilters,
  pendingEntries: [],
  pendingTotal: 0,
  pendingPage: 1,
  isLoading: false,
  isClockingIn: false,
  isClockingOut: false,
  error: null,
};

/**
 * Zustand clock store
 */
export const useClockStore = create<ClockStore>()((set, get) => ({
  ...initialState,

  /**
   * Fetch current clock status
   */
  fetchStatus: async () => {
    set({ isLoading: true, error: null });
    try {
      const status = await clocksApi.getStatus();
      set({ status, isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch clock status';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Clock in (notes are only allowed on clock-out)
   */
  clockIn: async () => {
    set({ isClockingIn: true, error: null });
    try {
      const entry = await clocksApi.clockIn();
      // Refresh status after clocking in
      await get().fetchStatus();
      set({ isClockingIn: false });
      return entry;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to clock in';
      set({ error: message, isClockingIn: false });
      throw error;
    }
  },

  /**
   * Clock out with optional notes
   */
  clockOut: async (notes?: string) => {
    set({ isClockingOut: true, error: null });
    try {
      const entry = await clocksApi.clockOut({ notes });
      // Refresh status after clocking out
      await get().fetchStatus();
      set({ isClockingOut: false });
      return entry;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to clock out';
      set({ error: message, isClockingOut: false });
      throw error;
    }
  },

  /**
   * Fetch clock history
   */
  fetchHistory: async (params: ClockHistoryParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const response = await clocksApi.getHistory(params);
      set({
        history: response?.data ?? [],
        historyTotal: response?.total ?? 0,
        historyPage: response?.page ?? 1,
        isLoading: false,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch history';
      set({ error: message, isLoading: false, history: [] });
      throw error;
    }
  },

  /**
   * Set history page (for pagination)
   */
  setHistoryPage: (page: number) => {
    set({ historyPage: page });
  },

  /**
   * Fetch history with current filters applied
   */
  fetchFilteredHistory: async (page: number = 1) => {
    const { historyFilters } = get();
    const params: ClockHistoryParams = {
      page,
      per_page: 10,
    };

    if (historyFilters.startDate) {
      params.start_date = historyFilters.startDate;
    }
    if (historyFilters.endDate) {
      params.end_date = historyFilters.endDate;
    }
    if (historyFilters.status && historyFilters.status !== 'all') {
      params.status = historyFilters.status;
    }

    await get().fetchHistory(params);
  },

  /**
   * Set history filters
   */
  setHistoryFilters: (filters: Partial<ClockFilterState>) => {
    set((state) => ({
      historyFilters: { ...state.historyFilters, ...filters },
      historyPage: 1, // Reset to page 1 when filters change
    }));
  },

  /**
   * Reset history filters to defaults
   */
  resetHistoryFilters: () => {
    set({ historyFilters: initialFilters, historyPage: 1 });
  },

  /**
   * Fetch pending entries for approval (Manager+)
   */
  fetchPendingEntries: async (params: PendingEntriesParams = {}) => {
    set({ isLoading: true, error: null });
    try {
      const response = await clocksApi.getPending(params);
      set({
        pendingEntries: response?.data ?? [],
        pendingTotal: response?.total ?? 0,
        pendingPage: response?.page ?? 1,
        isLoading: false,
      });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to fetch pending entries';
      set({ error: message, isLoading: false, pendingEntries: [] });
      throw error;
    }
  },

  /**
   * Set pending page (for pagination)
   */
  setPendingPage: (page: number) => {
    set({ pendingPage: page });
  },

  /**
   * Approve a clock entry (Manager+)
   */
  approveEntry: async (id: string) => {
    set({ isLoading: true, error: null });
    try {
      await clocksApi.approve(id);
      // Refresh pending list
      await get().fetchPendingEntries({ page: get().pendingPage });
      set({ isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to approve entry';
      set({ error: message, isLoading: false });
      throw error;
    }
  },

  /**
   * Reject a clock entry (Manager+)
   */
  rejectEntry: async (id: string, reason?: string) => {
    set({ isLoading: true, error: null });
    try {
      await clocksApi.reject(id, { reason });
      // Refresh pending list
      await get().fetchPendingEntries({ page: get().pendingPage });
      set({ isLoading: false });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to reject entry';
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
 * Initialize clock store on dashboard load
 */
export const initializeClockStore = async (): Promise<void> => {
  const store = useClockStore.getState();
  await store.fetchStatus();
};
