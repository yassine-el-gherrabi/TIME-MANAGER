/**
 * Pending Absences Page (Manager)
 *
 * Page for managers to review and approve/reject absence requests.
 */

import { useState, useCallback, useEffect, useMemo } from 'react';
import { toast } from 'sonner';
import { Loader2, CheckCircle, AlertCircle } from 'lucide-react';
import { logger } from '../utils/logger';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { Badge } from '../components/ui/badge';
import { PendingAbsenceCard, RejectReasonModal } from '../components/absences';
import { useInfiniteScroll } from '../hooks/useInfiniteScroll';
import { absencesApi } from '../api/absences';
import { absenceTypesApi } from '../api/absenceTypes';
import { mapErrorToMessage } from '../utils/errorHandling';
import type { Absence, AbsenceType } from '../types/absence';

export function PendingAbsencesPage() {
  // Reference data
  const [absenceTypes, setAbsenceTypes] = useState<AbsenceType[]>([]);

  // Load absence types on mount
  useEffect(() => {
    const loadTypes = async () => {
      try {
        const types = await absenceTypesApi.list();
        setAbsenceTypes(types);
      } catch (err) {
        logger.error('Failed to load absence types', err, { component: 'PendingAbsencesPage', action: 'loadTypes' });
      }
    };
    loadTypes();
  }, []);

  // Create type lookup map
  const typeMap = useMemo(() => {
    const map: Record<string, AbsenceType> = {};
    absenceTypes.forEach((t) => {
      map[t.id] = t;
    });
    return map;
  }, [absenceTypes]);

  // Fetch function for infinite scroll
  const fetchPending = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await absencesApi.listPending(params.page, params.per_page);
      return {
        data: response.data,
        total: response.total,
        page: response.page,
        per_page: response.per_page,
      };
    },
    []
  );

  // Use infinite scroll hook
  const {
    items: allAbsences,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<Absence>({
    fetchFn: fetchPending,
    perPage: 20,
  });

  // Track removed IDs for optimistic updates
  const [removedIds, setRemovedIds] = useState<Set<string>>(new Set());
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  // Filter out removed entries
  const absences = allAbsences.filter((a) => !removedIds.has(a.id));
  const displayTotal = Math.max(0, total - removedIds.size);

  // Reject dialog state
  const [rejectDialog, setRejectDialog] = useState<{
    open: boolean;
    absence: Absence | null;
    loading: boolean;
  }>({ open: false, absence: null, loading: false });

  // Handlers
  const handleApprove = async (absenceId: string) => {
    setActionLoading(absenceId);
    try {
      await absencesApi.approve(absenceId);
      toast.success('Absence request approved');
      setRemovedIds((prev) => new Set(prev).add(absenceId));
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setActionLoading(null);
    }
  };

  const handleRejectClick = (absenceId: string) => {
    const absence = absences.find((a) => a.id === absenceId);
    if (absence) {
      setRejectDialog({ open: true, absence, loading: false });
    }
  };

  const handleRejectConfirm = async (reason: string) => {
    if (!rejectDialog.absence) return;

    setRejectDialog((prev) => ({ ...prev, loading: true }));
    try {
      await absencesApi.reject(rejectDialog.absence.id, { reason });
      toast.success('Absence request rejected');
      setRemovedIds((prev) => new Set(prev).add(rejectDialog.absence!.id));
      setRejectDialog({ open: false, absence: null, loading: false });
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setRejectDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  if (isInitialLoading) {
    return (
      <div className="space-y-6">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Pending Absences</h1>
          <p className="text-muted-foreground">
            Review and approve employee absence requests
          </p>
        </div>
        <Card>
          <CardContent className="flex items-center justify-center py-8">
            <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
          </CardContent>
        </Card>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Pending Absences</h1>
        <p className="text-muted-foreground">
          Review and approve employee absence requests
        </p>
      </div>

      {/* Pending Absences List */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between text-lg">
            <span className="flex items-center gap-2">
              <AlertCircle className="h-5 w-5 text-amber-500" />
              Pending Approvals
            </span>
            {displayTotal > 0 && (
              <Badge variant="secondary">{displayTotal} pending</Badge>
            )}
          </CardTitle>
          <CardDescription>
            Review and approve or reject absence requests from your team
          </CardDescription>
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

          {absences.length === 0 ? (
            <div className="text-center py-8">
              <CheckCircle className="h-12 w-12 text-green-500 mx-auto mb-3" />
              <p className="text-sm text-muted-foreground">
                No pending absence requests
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {absences.map((absence) => (
                <PendingAbsenceCard
                  key={absence.id}
                  absence={absence}
                  absenceType={typeMap[absence.type_id]}
                  userName={absence.user_name || 'Unknown User'}
                  onApprove={handleApprove}
                  onReject={handleRejectClick}
                  isApproving={actionLoading === absence.id}
                  isRejecting={false}
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
              {!hasMore && absences.length > 0 && (
                <p className="text-center text-sm text-muted-foreground py-4">
                  All pending requests loaded
                </p>
              )}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Reject Reason Modal */}
      <RejectReasonModal
        open={rejectDialog.open}
        onOpenChange={(open) => setRejectDialog((prev) => ({ ...prev, open }))}
        onConfirm={handleRejectConfirm}
        loading={rejectDialog.loading}
        userName={rejectDialog.absence?.user_name}
        absenceType={rejectDialog.absence ? typeMap[rejectDialog.absence.type_id]?.name : undefined}
      />
    </div>
  );
}
