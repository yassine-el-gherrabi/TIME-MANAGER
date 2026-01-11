/**
 * Clock Page
 *
 * Clock history page with statistics, filters, and calendar/list views.
 * Uses infinite scroll for seamless data loading.
 */

import { useState, useCallback, useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { List, Calendar, Loader2, Download } from 'lucide-react';
import { toast } from 'sonner';
import { reportsApi } from '../api/reports';
import { mapErrorToMessage } from '../utils/errorHandling';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import {
  ClockStatistics,
  ClockFilters,
  ClockEntryCard,
  ClockCalendarView,
} from '../components/clock';
import { useInfiniteScroll } from '../hooks/useInfiniteScroll';
import { clocksApi } from '../api/clocks';
import type { ClockViewMode, ClockFilterState, ClockEntryResponse } from '../types/clock';

const initialFilters: ClockFilterState = {
  startDate: null,
  endDate: null,
  status: 'all',
};

export function ClockPage() {
  const { t } = useTranslation();
  const [viewMode, setViewMode] = useState<ClockViewMode>('list');
  const [filters, setFilters] = useState<ClockFilterState>(initialFilters);

  // Build fetch params from filters
  const fetchParams = useMemo(() => {
    const params: Record<string, string | undefined> = {};
    if (filters.startDate) params.start_date = filters.startDate;
    if (filters.endDate) params.end_date = filters.endDate;
    if (filters.status && filters.status !== 'all') params.status = filters.status;
    return params;
  }, [filters]);

  // Fetch function for infinite scroll
  const fetchHistory = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await clocksApi.getHistory({
        ...fetchParams,
        page: params.page,
        per_page: params.per_page,
      });
      return response;
    },
    [fetchParams]
  );

  // Use infinite scroll hook
  const {
    items: entries,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    sentinelRef,
  } = useInfiniteScroll<ClockEntryResponse>({
    fetchFn: fetchHistory,
    params: fetchParams,
    perPage: 20,
  });

  const handleFiltersChange = (newFilters: Partial<ClockFilterState>) => {
    setFilters((prev) => ({ ...prev, ...newFilters }));
  };

  const handleResetFilters = () => {
    setFilters(initialFilters);
  };

  const hasActiveFilters =
    filters.startDate !== null ||
    filters.endDate !== null ||
    filters.status !== 'all';

  // Export state
  const [exporting, setExporting] = useState(false);

  const handleExport = async () => {
    setExporting(true);
    try {
      await reportsApi.exportCsv('clocks', {
        start_date: filters.startDate || undefined,
        end_date: filters.endDate || undefined,
      });
      toast.success(t('success.exported'));
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="space-y-6">
      {/* Header with View Toggle */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">{t('nav.clockHistory')}</h1>
          <p className="text-muted-foreground">
            {t('clock.historyDescription')}
          </p>
        </div>

        {/* View Toggle and Export */}
        <div className="flex items-center gap-2">
          <Button variant="outline" size="sm" onClick={handleExport} disabled={exporting}>
            {exporting ? <Loader2 className="h-4 w-4 animate-spin" /> : <Download className="h-4 w-4" />}
            <span className="ml-2">{t('common.export')}</span>
          </Button>
          <div className="flex items-center gap-1 bg-muted p-1 rounded-lg">
            <Button
              variant={viewMode === 'list' ? 'default' : 'ghost'}
              size="sm"
              onClick={() => setViewMode('list')}
              className="gap-2"
            >
              <List className="h-4 w-4" />
              {t('clock.list')}
            </Button>
            <Button
              variant={viewMode === 'calendar' ? 'default' : 'ghost'}
              size="sm"
              onClick={() => setViewMode('calendar')}
              className="gap-2"
            >
              <Calendar className="h-4 w-4" />
              {t('clock.calendar')}
            </Button>
          </div>
        </div>
      </div>

      {/* Statistics */}
      <ClockStatistics entries={entries} isLoading={isInitialLoading} />

      {/* Filters */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-base">{t('common.filters')}</CardTitle>
        </CardHeader>
        <CardContent>
          <ClockFilters
            filters={filters}
            onFiltersChange={handleFiltersChange}
            onReset={handleResetFilters}
          />
        </CardContent>
      </Card>

      {/* Content - List or Calendar View */}
      {viewMode === 'list' ? (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center justify-between text-base">
              <span>{t('clock.entries')}</span>
              <span className="text-sm font-normal text-muted-foreground">
                {t('clock.entriesCount', { total })} {hasActiveFilters && `(${t('common.filtered')})`}
              </span>
            </CardTitle>
          </CardHeader>
          <CardContent>
            {isInitialLoading ? (
              <div className="flex items-center justify-center py-8">
                <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
              </div>
            ) : entries.length === 0 ? (
              <p className="text-center text-sm text-muted-foreground py-8">
                {t('clock.noEntries')}
              </p>
            ) : (
              <div className="space-y-3">
                {entries.map((entry) => (
                  <ClockEntryCard key={entry.id} entry={entry} />
                ))}

                {/* Infinite scroll sentinel */}
                <div ref={sentinelRef} className="h-4" />

                {/* Loading indicator */}
                {isLoading && !isInitialLoading && (
                  <div className="flex items-center justify-center py-4">
                    <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
                  </div>
                )}

                {/* End of list indicator */}
                {!hasMore && entries.length > 0 && (
                  <p className="text-center text-sm text-muted-foreground py-4">
                    {t('common.allLoaded')}
                  </p>
                )}
              </div>
            )}
          </CardContent>
        </Card>
      ) : (
        <ClockCalendarView
          entries={entries}
          isLoading={isInitialLoading}
          filterStartDate={filters.startDate}
          filterEndDate={filters.endDate}
          filterStatus={filters.status}
        />
      )}
    </div>
  );
}
