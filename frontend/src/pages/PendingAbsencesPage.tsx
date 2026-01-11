/**
 * Pending Absences Page (Manager)
 *
 * Page for managers to review and approve/reject absence requests.
 * Includes filtering by organization (SuperAdmin) and team (Admin/Manager).
 */

import { useState, useCallback, useEffect, useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'sonner';
import { Loader2, CheckCircle, AlertCircle, Building2, Users, Filter } from 'lucide-react';
import { logger } from '../utils/logger';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { Badge } from '../components/ui/badge';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../components/ui/select';
import { PendingAbsenceCard, RejectReasonModal } from '../components/absences';
import { useInfiniteScroll } from '../hooks/useInfiniteScroll';
import { useCurrentUser } from '../hooks/useAuth';
import { absencesApi } from '../api/absences';
import { absenceTypesApi } from '../api/absenceTypes';
import { organizationsApi } from '../api/organizations';
import { teamsApi } from '../api/teams';
import { mapErrorToMessage } from '../utils/errorHandling';
import type { Absence, AbsenceType, PendingAbsenceFilter } from '../types/absence';
import type { OrganizationResponse } from '../types/organization';
import type { TeamResponse } from '../types/team';
import { UserRole } from '../types/auth';

export function PendingAbsencesPage() {
  const { t } = useTranslation();
  const user = useCurrentUser();

  // Reference data
  const [absenceTypes, setAbsenceTypes] = useState<AbsenceType[]>([]);

  // Filter state
  const [selectedOrgId, setSelectedOrgId] = useState<string>('');
  const [selectedTeamId, setSelectedTeamId] = useState<string>('');
  const [organizations, setOrganizations] = useState<OrganizationResponse[]>([]);
  const [teams, setTeams] = useState<TeamResponse[]>([]);
  const [loadingOrgs, setLoadingOrgs] = useState(false);
  const [loadingTeams, setLoadingTeams] = useState(false);

  // Check user role for filter visibility
  const isSuperAdmin = user?.role === UserRole.SuperAdmin;
  const isAdminOrHigher = user?.role === UserRole.SuperAdmin || user?.role === UserRole.Admin;
  const canFilterByTeam = isAdminOrHigher || user?.role === UserRole.Manager;

  // Load organizations for SuperAdmin
  useEffect(() => {
    if (isSuperAdmin) {
      setLoadingOrgs(true);
      organizationsApi.list({ per_page: 100 })
        .then((response) => setOrganizations(response.data))
        .catch(() => setOrganizations([]))
        .finally(() => setLoadingOrgs(false));
    }
  }, [isSuperAdmin]);

  // Load teams for Admin/Manager/SuperAdmin
  useEffect(() => {
    if (canFilterByTeam) {
      setLoadingTeams(true);
      teamsApi.list({ per_page: 100 })
        .then((response) => setTeams(response.teams))
        .catch(() => setTeams([]))
        .finally(() => setLoadingTeams(false));
    }
  }, [canFilterByTeam]);

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

  // Fetch function for infinite scroll with filters
  const fetchPending = useCallback(
    async (params: { page: number; per_page: number }) => {
      const filter: PendingAbsenceFilter = {
        page: params.page,
        per_page: params.per_page,
      };
      if (selectedOrgId) {
        filter.organization_id = selectedOrgId;
      }
      if (selectedTeamId) {
        filter.team_id = selectedTeamId;
      }
      const response = await absencesApi.listPending(filter);
      return {
        data: response.data,
        total: response.total,
        page: response.page,
        per_page: response.per_page,
      };
    },
    [selectedOrgId, selectedTeamId]
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

  // Reset when filters change
  useEffect(() => {
    setRemovedIds(new Set());
    reset();
  }, [selectedOrgId, selectedTeamId, reset]);

  // Filter out removed entries
  const absences = allAbsences.filter((a) => !removedIds.has(a.id));
  const displayTotal = Math.max(0, total - removedIds.size);

  // Handle filter changes
  const handleOrgChange = (value: string) => {
    setSelectedOrgId(value === 'all' ? '' : value);
  };

  const handleTeamChange = (value: string) => {
    setSelectedTeamId(value === 'all' ? '' : value);
  };

  const hasActiveFilters = selectedOrgId !== '' || selectedTeamId !== '';

  const clearFilters = () => {
    setSelectedOrgId('');
    setSelectedTeamId('');
  };

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
      toast.success(t('success.approved'));
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
      toast.success(t('success.rejected'));
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
          <h1 className="text-2xl font-bold tracking-tight">{t('pendingAbsences.title')}</h1>
          <p className="text-muted-foreground">
            {t('pendingAbsences.description')}
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
        <h1 className="text-2xl font-bold tracking-tight">{t('pendingAbsences.title')}</h1>
        <p className="text-muted-foreground">
          {t('pendingAbsences.description')}
        </p>
      </div>

      {/* Pending Absences List */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between text-lg">
            <span className="flex items-center gap-2">
              <AlertCircle className="h-5 w-5 text-amber-500" />
              {t('pendingAbsences.pendingApprovals')}
            </span>
            {displayTotal > 0 && (
              <Badge variant="secondary">{t('pendingAbsences.pendingCount', { count: displayTotal })}</Badge>
            )}
          </CardTitle>
          <CardDescription>
            {t('pendingAbsences.reviewDescription')}
          </CardDescription>

          {/* Filters */}
          {(isSuperAdmin || canFilterByTeam) && (
            <div className="flex flex-wrap items-center gap-3 mt-4 pt-4 border-t">
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Filter className="h-4 w-4" />
                <span>{t('pendingAbsences.filterBy')}</span>
              </div>

              {/* Organization filter - SuperAdmin only */}
              {isSuperAdmin && (
                <Select
                  value={selectedOrgId || 'all'}
                  onValueChange={handleOrgChange}
                  disabled={loadingOrgs}
                >
                  <SelectTrigger className="w-[180px] h-9">
                    <Building2 className="h-4 w-4 mr-2" />
                    <SelectValue placeholder="All Organizations" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="all">{t('pendingAbsences.allOrganizations')}</SelectItem>
                    {organizations.map((org) => (
                      <SelectItem key={org.id} value={org.id}>
                        {org.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              )}

              {/* Team filter - Admin/Manager/SuperAdmin */}
              {canFilterByTeam && (
                <Select
                  value={selectedTeamId || 'all'}
                  onValueChange={handleTeamChange}
                  disabled={loadingTeams}
                >
                  <SelectTrigger className="w-[180px] h-9">
                    <Users className="h-4 w-4 mr-2" />
                    <SelectValue placeholder="All Teams" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="all">{t('pendingAbsences.allTeams')}</SelectItem>
                    {teams.map((team) => (
                      <SelectItem key={team.id} value={team.id}>
                        {team.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              )}

              {/* Clear filters button */}
              {hasActiveFilters && (
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={clearFilters}
                  className="h-9"
                >
                  {t('common.clearFilters')}
                </Button>
              )}
            </div>
          )}
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
              {error.message}
              <Button variant="outline" size="sm" className="ml-2" onClick={reset}>
                {t('common.tryAgain')}
              </Button>
            </div>
          )}

          {absences.length === 0 ? (
            <div className="text-center py-8">
              <CheckCircle className="h-12 w-12 text-green-500 mx-auto mb-3" />
              <p className="text-sm text-muted-foreground">
                {t('pendingAbsences.noPendingRequests')}
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
                  {t('pendingAbsences.allLoaded')}
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
