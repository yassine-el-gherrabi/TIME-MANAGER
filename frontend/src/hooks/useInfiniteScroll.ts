/**
 * useInfiniteScroll Hook
 *
 * Provides infinite scroll functionality with Intersection Observer.
 * Automatically loads more items when scrolling near the bottom.
 */

import { useState, useEffect, useRef, useCallback } from 'react';

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  per_page: number;
}

export interface UseInfiniteScrollOptions<T, P = Record<string, unknown>> {
  /** Function to fetch a page of data */
  fetchFn: (params: P & { page: number; per_page: number }) => Promise<PaginatedResponse<T>>;
  /** Additional params to pass to fetchFn (filters, etc.) */
  params?: P;
  /** Items per page */
  perPage?: number;
  /** Whether to start loading immediately */
  enabled?: boolean;
}

export interface UseInfiniteScrollResult<T> {
  /** All accumulated items */
  items: T[];
  /** Whether currently loading */
  isLoading: boolean;
  /** Whether initial load is happening */
  isInitialLoading: boolean;
  /** Whether there are more items to load */
  hasMore: boolean;
  /** Total number of items available */
  total: number;
  /** Any error that occurred */
  error: Error | null;
  /** Ref to attach to the sentinel element at the bottom */
  sentinelRef: React.RefObject<HTMLDivElement>;
  /** Manually trigger loading more */
  loadMore: () => void;
  /** Reset and reload from the beginning */
  reset: () => void;
}

export function useInfiniteScroll<T, P = Record<string, unknown>>({
  fetchFn,
  params = {} as P,
  perPage = 20,
  enabled = true,
}: UseInfiniteScrollOptions<T, P>): UseInfiniteScrollResult<T> {
  const [items, setItems] = useState<T[]>([]);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [isInitialLoading, setIsInitialLoading] = useState(true);
  const [total, setTotal] = useState(0);
  const [error, setError] = useState<Error | null>(null);

  const sentinelRef = useRef<HTMLDivElement>(null);
  const paramsRef = useRef(params);

  // Track if params changed to trigger reset
  const paramsKey = JSON.stringify(params);

  // Reset when params change
  useEffect(() => {
    paramsRef.current = params;
    setItems([]);
    setPage(1);
    setHasMore(true);
    setIsInitialLoading(true);
    setError(null);
  }, [paramsKey]);

  // Fetch function
  const fetchPage = useCallback(
    async (pageNum: number, isReset = false) => {
      if (!enabled) return;

      setIsLoading(true);
      setError(null);

      try {
        const response = await fetchFn({
          ...paramsRef.current,
          page: pageNum,
          per_page: perPage,
        });

        setItems((prev) => {
          if (isReset || pageNum === 1) {
            return response.data;
          }
          return [...prev, ...response.data];
        });

        setTotal(response.total);
        setHasMore(response.data.length === perPage && items.length + response.data.length < response.total);
        setPage(pageNum);
      } catch (err) {
        setError(err instanceof Error ? err : new Error('Failed to fetch'));
      } finally {
        setIsLoading(false);
        setIsInitialLoading(false);
      }
    },
    [enabled, fetchFn, perPage, items.length]
  );

  // Initial load
  useEffect(() => {
    if (enabled && isInitialLoading) {
      fetchPage(1, true);
    }
  }, [enabled, isInitialLoading, fetchPage]);

  // Load more function
  const loadMore = useCallback(() => {
    if (!isLoading && hasMore && enabled) {
      fetchPage(page + 1);
    }
  }, [isLoading, hasMore, enabled, fetchPage, page]);

  // Reset function
  const reset = useCallback(() => {
    setItems([]);
    setPage(1);
    setHasMore(true);
    setIsInitialLoading(true);
    setError(null);
  }, []);

  // Intersection Observer
  useEffect(() => {
    const sentinel = sentinelRef.current;
    if (!sentinel || !enabled) return;

    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && hasMore && !isLoading && !isInitialLoading) {
          loadMore();
        }
      },
      {
        root: null,
        rootMargin: '100px',
        threshold: 0.1,
      }
    );

    observer.observe(sentinel);

    return () => {
      observer.disconnect();
    };
  }, [hasMore, isLoading, isInitialLoading, loadMore, enabled]);

  return {
    items,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    loadMore,
    reset,
  };
}
