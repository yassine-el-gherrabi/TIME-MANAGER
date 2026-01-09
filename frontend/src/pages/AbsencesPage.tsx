/**
 * Absences Page (Employee)
 *
 * Page for employees to view their absences and submit new requests.
 * Shows leave balances and absence history with filters.
 */

import { useState, useCallback, useMemo, useEffect } from 'react';
import { toast } from 'sonner';
import { Plus, Loader2, Calendar, Ban } from 'lucide-react';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { ConfirmDialog } from '../components/ui/confirm-dialog';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../components/ui/sheet';
import {
  AbsenceCard,
  AbsenceRequestForm,
  LeaveBalanceCard,
} from '../components/absences';
import { useInfiniteScroll } from '../hooks/useInfiniteScroll';
import { absencesApi } from '../api/absences';
import { absenceTypesApi } from '../api/absenceTypes';
import { balancesApi } from '../api/balances';
import { mapErrorToMessage } from '../utils/errorHandling';
import type { Absence, AbsenceType, LeaveBalance, CreateAbsenceRequest } from '../types/absence';
import { AbsenceStatus } from '../types/absence';

type FilterStatus = 'all' | AbsenceStatus;

export function AbsencesPage() {
  // Filter state
  const [filterStatus, setFilterStatus] = useState<FilterStatus>('all');
  const [filterTypeId, setFilterTypeId] = useState<string>('');

  // Reference data
  const [absenceTypes, setAbsenceTypes] = useState<AbsenceType[]>([]);
  const [balances, setBalances] = useState<LeaveBalance[]>([]);
  const [loadingRef, setLoadingRef] = useState(true);

  // Load absence types and balances on mount
  useEffect(() => {
    const loadReferenceData = async () => {
      try {
        const [types, userBalances] = await Promise.all([
          absenceTypesApi.list(),
          balancesApi.getMyBalances(),
        ]);
        setAbsenceTypes(types);
        setBalances(userBalances);
      } catch (err) {
        console.error('Failed to load reference data:', err);
        toast.error('Failed to load absence types');
      } finally {
        setLoadingRef(false);
      }
    };
    loadReferenceData();
  }, []);

  // Build fetch params from filters
  const fetchParams = useMemo(() => {
    const params: Record<string, string | undefined> = {};
    if (filterStatus !== 'all') params.status = filterStatus;
    if (filterTypeId) params.type_id = filterTypeId;
    return params;
  }, [filterStatus, filterTypeId]);

  // Fetch function for infinite scroll
  const fetchAbsences = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await absencesApi.list({
        ...fetchParams,
        page: params.page,
        per_page: params.per_page,
      });
      return {
        data: response.data,
        total: response.total,
        page: response.page,
        per_page: response.per_page,
      };
    },
    [fetchParams]
  );

  // Use infinite scroll hook
  const {
    items: absences,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<Absence>({
    fetchFn: fetchAbsences,
    params: fetchParams,
    perPage: 20,
  });

  // Track removed absence IDs for optimistic updates
  const [removedIds, setRemovedIds] = useState<Set<string>>(new Set());
  const displayedAbsences = absences.filter((a) => !removedIds.has(a.id));

  // Request drawer state
  const [requestDrawer, setRequestDrawer] = useState<{
    open: boolean;
    loading: boolean;
    error: string;
  }>({ open: false, loading: false, error: '' });

  // Cancel dialog state
  const [cancelDialog, setCancelDialog] = useState<{
    open: boolean;
    absence: Absence | null;
    loading: boolean;
  }>({ open: false, absence: null, loading: false });

  // Create type lookup map
  const typeMap = useMemo(() => {
    const map: Record<string, AbsenceType> = {};
    absenceTypes.forEach((t) => {
      map[t.id] = t;
    });
    return map;
  }, [absenceTypes]);

  // Handlers
  const handleRequestClick = () => {
    setRequestDrawer({ open: true, loading: false, error: '' });
  };

  const handleRequestSubmit = async (data: CreateAbsenceRequest) => {
    setRequestDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await absencesApi.create(data);
      toast.success('Absence request submitted successfully');
      setRequestDrawer({ open: false, loading: false, error: '' });
      // Refresh balances
      const newBalances = await balancesApi.getMyBalances();
      setBalances(newBalances);
      setRemovedIds(new Set());
      reset();
    } catch (err) {
      setRequestDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleRequestCancel = () => {
    setRequestDrawer({ open: false, loading: false, error: '' });
  };

  const handleCancelClick = (absence: Absence) => {
    setCancelDialog({ open: true, absence, loading: false });
  };

  const handleCancelConfirm = async () => {
    if (!cancelDialog.absence) return;

    setCancelDialog((prev) => ({ ...prev, loading: true }));
    try {
      await absencesApi.cancel(cancelDialog.absence.id);
      toast.success('Absence request cancelled');
      setRemovedIds((prev) => new Set(prev).add(cancelDialog.absence!.id));
      // Refresh balances
      const newBalances = await balancesApi.getMyBalances();
      setBalances(newBalances);
      setCancelDialog({ open: false, absence: null, loading: false });
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setCancelDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const hasActiveFilters = filterStatus !== 'all' || filterTypeId !== '';

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">My Absences</h1>
          <p className="text-muted-foreground">
            View your leave balances and absence requests
          </p>
        </div>
        <Button onClick={handleRequestClick} className="gap-2">
          <Plus className="h-4 w-4" />
          New Request
        </Button>
      </div>

      {/* Leave Balances */}
      {loadingRef ? (
        <div className="flex items-center justify-center py-8">
          <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
        </div>
      ) : balances.length > 0 ? (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          {balances.map((balance) => (
            <LeaveBalanceCard key={balance.id} balance={balance} />
          ))}
        </div>
      ) : null}

      {/* Filters */}
      <Card>
        <CardHeader className="pb-3">
          <CardTitle className="text-base">Filters</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-4">
            <div className="space-y-1">
              <label className="text-sm text-muted-foreground">Status</label>
              <select
                value={filterStatus}
                onChange={(e) => {
                  setFilterStatus(e.target.value as FilterStatus);
                  setRemovedIds(new Set());
                }}
                className="flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              >
                <option value="all">All statuses</option>
                <option value={AbsenceStatus.Pending}>Pending</option>
                <option value={AbsenceStatus.Approved}>Approved</option>
                <option value={AbsenceStatus.Rejected}>Rejected</option>
                <option value={AbsenceStatus.Cancelled}>Cancelled</option>
              </select>
            </div>

            <div className="space-y-1">
              <label className="text-sm text-muted-foreground">Type</label>
              <select
                value={filterTypeId}
                onChange={(e) => {
                  setFilterTypeId(e.target.value);
                  setRemovedIds(new Set());
                }}
                className="flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              >
                <option value="">All types</option>
                {absenceTypes.map((type) => (
                  <option key={type.id} value={type.id}>
                    {type.name}
                  </option>
                ))}
              </select>
            </div>

            {hasActiveFilters && (
              <Button
                variant="ghost"
                size="sm"
                className="self-end"
                onClick={() => {
                  setFilterStatus('all');
                  setFilterTypeId('');
                  setRemovedIds(new Set());
                }}
              >
                Clear filters
              </Button>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Absences List */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between text-base">
            <span className="flex items-center gap-2">
              <Calendar className="h-5 w-5" />
              Absence Requests
            </span>
            <span className="text-sm font-normal text-muted-foreground">
              {displayedAbsences.length} of {total} {hasActiveFilters && '(filtered)'}
            </span>
          </CardTitle>
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

          {isInitialLoading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
            </div>
          ) : displayedAbsences.length === 0 ? (
            <div className="text-center py-8">
              <Calendar className="h-12 w-12 text-muted-foreground mx-auto mb-3" />
              <p className="text-sm text-muted-foreground">
                No absence requests found
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {displayedAbsences.map((absence) => (
                <AbsenceCard
                  key={absence.id}
                  absence={absence}
                  absenceType={typeMap[absence.type_id]}
                  actions={
                    absence.status === AbsenceStatus.Pending && (
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleCancelClick(absence)}
                        className="gap-1 text-destructive hover:text-destructive"
                      >
                        <Ban className="h-4 w-4" />
                        Cancel
                      </Button>
                    )
                  }
                />
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
              {!hasMore && displayedAbsences.length > 0 && (
                <p className="text-center text-sm text-muted-foreground py-4">
                  All absences loaded
                </p>
              )}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Request Drawer */}
      <Sheet open={requestDrawer.open} onOpenChange={(open) => !open && handleRequestCancel()}>
        <SheetContent className="overflow-y-auto">
          <SheetHeader>
            <SheetTitle>Request Absence</SheetTitle>
            <SheetDescription>
              Submit a new absence request for approval.
            </SheetDescription>
          </SheetHeader>
          <AbsenceRequestForm
            absenceTypes={absenceTypes}
            balances={balances}
            onSubmit={handleRequestSubmit}
            onCancel={handleRequestCancel}
            isLoading={requestDrawer.loading}
            error={requestDrawer.error}
            variant="sheet"
          />
        </SheetContent>
      </Sheet>

      {/* Cancel Confirmation Dialog */}
      <ConfirmDialog
        open={cancelDialog.open}
        onOpenChange={(open) => setCancelDialog((prev) => ({ ...prev, open }))}
        title="Cancel Absence Request"
        description="Are you sure you want to cancel this absence request? This action cannot be undone."
        confirmText="Cancel Request"
        variant="destructive"
        onConfirm={handleCancelConfirm}
        loading={cancelDialog.loading}
      />
    </div>
  );
}
