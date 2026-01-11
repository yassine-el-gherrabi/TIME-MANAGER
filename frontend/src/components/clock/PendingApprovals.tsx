/**
 * Pending Approvals Component
 *
 * Lists pending clock entries for manager approval/rejection.
 * Uses infinite scroll for seamless loading.
 * Includes filtering by organization (SuperAdmin) and team (Admin/Manager).
 */

import { useState, useCallback, useEffect, type FC, type ChangeEvent } from 'react';
import { format } from 'date-fns';
import { Loader2, Clock, CheckCircle, XCircle, AlertCircle, Building2, Users, Filter } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../ui/card';
import { Button } from '../ui/button';
import { Badge } from '../ui/badge';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '../ui/dialog';
import { Textarea } from '../ui/textarea';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../ui/select';
import { useInfiniteScroll } from '../../hooks/useInfiniteScroll';
import { useCurrentUser } from '../../hooks/useAuth';
import { clocksApi } from '../../api/clocks';
import { organizationsApi } from '../../api/organizations';
import { teamsApi } from '../../api/teams';
import type { ClockEntryResponse, PendingEntriesParams } from '../../types/clock';
import type { OrganizationResponse } from '../../types/organization';
import type { TeamResponse } from '../../types/team';
import { UserRole } from '../../types/auth';

interface PendingApprovalsProps {
  className?: string;
}

/**
 * Calculate duration between two timestamps
 */
const calculateDuration = (clockIn: string, clockOut: string | null): string => {
  if (!clockOut) return 'In progress';

  const start = new Date(clockIn).getTime();
  const end = new Date(clockOut).getTime();
  const diffMs = end - start;

  const hours = Math.floor(diffMs / (1000 * 60 * 60));
  const minutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));

  if (hours > 0) {
    return `${hours}h ${minutes}m`;
  }
  return `${minutes}m`;
};

export const PendingApprovals: FC<PendingApprovalsProps> = ({ className }) => {
  const user = useCurrentUser();
  const [rejectDialogOpen, setRejectDialogOpen] = useState(false);
  const [selectedEntry, setSelectedEntry] = useState<ClockEntryResponse | null>(null);
  const [rejectReason, setRejectReason] = useState('');
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [removedIds, setRemovedIds] = useState<Set<string>>(new Set());

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

  // Fetch function for infinite scroll with filters
  const fetchPending = useCallback(
    async (params: { page: number; per_page: number }) => {
      const queryParams: PendingEntriesParams = {
        page: params.page,
        per_page: params.per_page,
      };
      if (selectedOrgId) {
        queryParams.organization_id = selectedOrgId;
      }
      if (selectedTeamId) {
        queryParams.team_id = selectedTeamId;
      }
      const response = await clocksApi.getPending(queryParams);
      return response;
    },
    [selectedOrgId, selectedTeamId]
  );

  // Use infinite scroll hook
  const {
    items: allEntries,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<ClockEntryResponse>({
    fetchFn: fetchPending,
    perPage: 20,
  });

  // Reset when filters change
  useEffect(() => {
    setRemovedIds(new Set());
    reset();
  }, [selectedOrgId, selectedTeamId, reset]);

  // Filter out removed entries
  const entries = allEntries.filter((e) => !removedIds.has(e.id));
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

  const handleApprove = async (entry: ClockEntryResponse) => {
    setActionLoading(entry.id);
    try {
      await clocksApi.approve(entry.id);
      // Remove from local list instead of refetching
      setRemovedIds((prev) => new Set(prev).add(entry.id));
    } catch {
      // Error handled silently for now
    } finally {
      setActionLoading(null);
    }
  };

  const handleRejectClick = (entry: ClockEntryResponse) => {
    setSelectedEntry(entry);
    setRejectReason('');
    setRejectDialogOpen(true);
  };

  const handleRejectConfirm = async () => {
    if (!selectedEntry || !rejectReason.trim()) return;

    setActionLoading(selectedEntry.id);
    try {
      await clocksApi.reject(selectedEntry.id, { reason: rejectReason.trim() });
      // Remove from local list
      setRemovedIds((prev) => new Set(prev).add(selectedEntry.id));
      setRejectDialogOpen(false);
      setSelectedEntry(null);
      setRejectReason('');
    } catch {
      // Error handled silently for now
    } finally {
      setActionLoading(null);
    }
  };

  if (isInitialLoading) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <AlertCircle className="h-5 w-5" />
            Pending Approvals
          </CardTitle>
        </CardHeader>
        <CardContent className="flex items-center justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <AlertCircle className="h-5 w-5" />
            Pending Approvals
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-destructive">{error.message}</p>
          <Button variant="outline" size="sm" className="mt-2" onClick={reset}>
            Try again
          </Button>
        </CardContent>
      </Card>
    );
  }

  return (
    <>
      <Card className={className}>
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
            Review and approve or reject clock entries
          </CardDescription>

          {/* Filters */}
          {(isSuperAdmin || canFilterByTeam) && (
            <div className="flex flex-wrap items-center gap-3 mt-4 pt-4 border-t">
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Filter className="h-4 w-4" />
                <span>Filter by:</span>
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
                    <SelectItem value="all">All Organizations</SelectItem>
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
                    <SelectItem value="all">All Teams</SelectItem>
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
                  Clear filters
                </Button>
              )}
            </div>
          )}
        </CardHeader>
        <CardContent className="space-y-4">
          {entries.length === 0 ? (
            <div className="text-center py-8">
              <CheckCircle className="h-12 w-12 text-green-500 mx-auto mb-3" />
              <p className="text-sm text-muted-foreground">
                No pending approvals
              </p>
            </div>
          ) : (
            <div className="space-y-3">
              {entries.map((entry) => (
                <div
                  key={entry.id}
                  className="flex items-center justify-between p-4 border rounded-lg"
                >
                  <div className="flex items-start gap-3">
                    <div className="rounded-full bg-primary/10 p-2">
                      <Clock className="h-4 w-4 text-primary" />
                    </div>
                    <div>
                      <p className="text-sm font-medium">
                        {entry.user_name}
                      </p>
                      <div className="flex items-center gap-2 text-xs text-muted-foreground">
                        {entry.organization_name && (
                          <span className="flex items-center gap-1">
                            <Building2 className="h-3 w-3" />
                            {entry.organization_name}
                          </span>
                        )}
                        {entry.team_name && (
                          <span className="flex items-center gap-1">
                            <Users className="h-3 w-3" />
                            {entry.team_name}
                          </span>
                        )}
                      </div>
                      <p className="text-xs text-muted-foreground mb-1">
                        {format(new Date(entry.clock_in), 'EEEE, MMM d')}
                      </p>
                      <div className="flex items-center gap-3 text-sm text-muted-foreground">
                        <span>
                          {format(new Date(entry.clock_in), 'HH:mm')}
                          {entry.clock_out && ` - ${format(new Date(entry.clock_out), 'HH:mm')}`}
                        </span>
                        <span className="font-medium">
                          {calculateDuration(entry.clock_in, entry.clock_out)}
                        </span>
                        {entry.theoretical_hours != null && entry.duration_minutes != null && (
                          <span className={`text-xs ${
                            (entry.duration_minutes / 60) >= entry.theoretical_hours
                              ? 'text-green-600'
                              : 'text-orange-600'
                          }`}>
                            ({entry.theoretical_hours.toFixed(1)}h expected,{' '}
                            {((entry.duration_minutes / 60) - entry.theoretical_hours) >= 0 ? '+' : ''}
                            {((entry.duration_minutes / 60) - entry.theoretical_hours).toFixed(1)}h)
                          </span>
                        )}
                      </div>
                      {entry.notes && (
                        <p className="text-xs text-muted-foreground mt-1">
                          {entry.notes}
                        </p>
                      )}
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    <Button
                      size="sm"
                      variant="outline"
                      onClick={() => handleRejectClick(entry)}
                      disabled={actionLoading === entry.id}
                    >
                      <XCircle className="h-4 w-4 mr-1 text-destructive" />
                      Reject
                    </Button>
                    <Button
                      size="sm"
                      onClick={() => handleApprove(entry)}
                      disabled={actionLoading === entry.id}
                    >
                      {actionLoading === entry.id ? (
                        <Loader2 className="h-4 w-4 mr-1 animate-spin" />
                      ) : (
                        <CheckCircle className="h-4 w-4 mr-1" />
                      )}
                      Approve
                    </Button>
                  </div>
                </div>
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
                  All pending entries loaded
                </p>
              )}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Reject Dialog */}
      <Dialog open={rejectDialogOpen} onOpenChange={setRejectDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Reject Clock Entry</DialogTitle>
            <DialogDescription>
              Please provide a reason for rejecting this clock entry.
            </DialogDescription>
          </DialogHeader>
          <div className="py-4">
            <Textarea
              placeholder="Reason for rejection..."
              value={rejectReason}
              onChange={(e: ChangeEvent<HTMLTextAreaElement>) => setRejectReason(e.target.value)}
              rows={3}
            />
          </div>
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setRejectDialogOpen(false)}
            >
              Cancel
            </Button>
            <Button
              variant="destructive"
              onClick={handleRejectConfirm}
              disabled={!rejectReason.trim() || actionLoading !== null}
            >
              {actionLoading ? (
                <Loader2 className="h-4 w-4 mr-1 animate-spin" />
              ) : (
                <XCircle className="h-4 w-4 mr-1" />
              )}
              Reject Entry
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};
