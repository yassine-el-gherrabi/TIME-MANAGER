/**
 * Audit Logs Page
 *
 * Super Admin page to view and export system audit logs.
 * Features filtering by entity type, action, and date range.
 * Supports infinite scroll and CSV export.
 */

import { useState, useCallback, useMemo } from 'react';
import { toast } from 'sonner';
import { Loader2, Download, ScrollText } from 'lucide-react';
import { Button } from '../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../components/ui/card';
import {
  AuditLogFilters,
  AuditLogsTable,
  AuditLogDetailsSheet,
} from '../../components/admin';
import { auditLogsApi } from '../../api/auditLogs';
import { useInfiniteScroll } from '../../hooks/useInfiniteScroll';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { AuditLog, AuditLogFilter } from '../../types/audit';
import { AuditAction } from '../../types/audit';

export function AuditLogsPage() {
  // Filter state
  const [filters, setFilters] = useState({
    entityType: '',
    action: '' as AuditAction | '',
    startDate: '',
    endDate: '',
  });

  // Export loading state
  const [isExporting, setIsExporting] = useState(false);

  // Details sheet state
  const [detailsSheet, setDetailsSheet] = useState<{
    open: boolean;
    log: AuditLog | null;
  }>({ open: false, log: null });

  // Build fetch params from filters
  const fetchParams = useMemo((): AuditLogFilter => {
    const params: AuditLogFilter = {};
    if (filters.entityType) params.entity_type = filters.entityType;
    if (filters.action) params.action = filters.action;
    if (filters.startDate) params.start_date = filters.startDate;
    if (filters.endDate) params.end_date = filters.endDate;
    return params;
  }, [filters]);

  // Fetch function for infinite scroll
  const fetchLogs = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await auditLogsApi.list({
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
    items: logs,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<AuditLog>({
    fetchFn: fetchLogs,
    params: fetchParams,
    perPage: 50,
  });

  const hasActiveFilters =
    filters.entityType !== '' ||
    filters.action !== '' ||
    filters.startDate !== '' ||
    filters.endDate !== '';

  // Filter handlers
  const handleEntityTypeChange = (value: string) => {
    setFilters((prev) => ({ ...prev, entityType: value }));
  };

  const handleActionChange = (value: AuditAction | '') => {
    setFilters((prev) => ({ ...prev, action: value }));
  };

  const handleStartDateChange = (value: string) => {
    setFilters((prev) => ({ ...prev, startDate: value }));
  };

  const handleEndDateChange = (value: string) => {
    setFilters((prev) => ({ ...prev, endDate: value }));
  };

  const handleClearFilters = () => {
    setFilters({
      entityType: '',
      action: '',
      startDate: '',
      endDate: '',
    });
  };

  // Row click handler
  const handleRowClick = (log: AuditLog) => {
    setDetailsSheet({ open: true, log });
  };

  // Export handler
  const handleExport = async () => {
    setIsExporting(true);
    try {
      await auditLogsApi.exportCsv(fetchParams);
      toast.success('Export started - file download will begin shortly');
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <ScrollText className="h-5 w-5" />
              <span>Audit Logs</span>
              {total > 0 && (
                <span className="text-sm font-normal text-muted-foreground ml-2">
                  {logs.length} of {total} entries {hasActiveFilters && '(filtered)'}
                </span>
              )}
            </CardTitle>
            <CardDescription>
              System activity audit trail for compliance and security monitoring
            </CardDescription>
          </div>
          <Button
            variant="outline"
            onClick={handleExport}
            disabled={isExporting || total === 0}
          >
            {isExporting ? (
              <Loader2 className="h-4 w-4 animate-spin mr-2" />
            ) : (
              <Download className="h-4 w-4 mr-2" />
            )}
            Export CSV
          </Button>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
              {error.message}
              <Button variant="outline" size="sm" className="ml-2" onClick={reset}>
                Try again
              </Button>
            </div>
          )}

          <AuditLogFilters
            entityType={filters.entityType}
            action={filters.action}
            startDate={filters.startDate}
            endDate={filters.endDate}
            onEntityTypeChange={handleEntityTypeChange}
            onActionChange={handleActionChange}
            onStartDateChange={handleStartDateChange}
            onEndDateChange={handleEndDateChange}
            onClearFilters={handleClearFilters}
            hasActiveFilters={hasActiveFilters}
          />

          <AuditLogsTable
            logs={logs}
            isLoading={isInitialLoading}
            onRowClick={handleRowClick}
          />

          {/* Infinite scroll elements */}
          {!isInitialLoading && logs.length > 0 && (
            <>
              {/* Sentinel element for intersection observer */}
              <div ref={sentinelRef} className="h-4" />

              {/* Loading more indicator */}
              {isLoading && (
                <div className="flex items-center justify-center py-4">
                  <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
                </div>
              )}

              {/* End of list indicator */}
              {!hasMore && (
                <p className="text-center text-sm text-muted-foreground py-4">
                  All audit logs loaded
                </p>
              )}
            </>
          )}

          <AuditLogDetailsSheet
            log={detailsSheet.log}
            open={detailsSheet.open}
            onOpenChange={(open) => setDetailsSheet((prev) => ({ ...prev, open }))}
          />
        </CardContent>
      </Card>
    </div>
  );
}
