/**
 * Teams Management Page
 *
 * Admin page to manage organization teams with infinite scroll.
 * Supports filtering by search and manager.
 * Team creation and editing are done via side sheets.
 */

import { useState, useCallback, useMemo, useEffect } from 'react';
import { toast } from 'sonner';
import { Loader2 } from 'lucide-react';
import { logger } from '../../utils/logger';
import { Button } from '../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../components/ui/card';
import { ConfirmDialog } from '../../components/ui/confirm-dialog';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../../components/ui/sheet';
import { TeamsTable } from '../../components/admin/TeamsTable';
import { TeamFilters } from '../../components/admin/TeamFilters';
import { TeamForm } from '../../components/admin/TeamForm';
import { TeamMembersPanel } from '../../components/admin/TeamMembersPanel';
import { teamsApi } from '../../api/teams';
import { usersApi } from '../../api/users';
import { schedulesApi } from '../../api/schedules';
import { useInfiniteScroll } from '../../hooks/useInfiniteScroll';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { TeamResponse, CreateTeamRequest } from '../../types/team';
import type { UserResponse } from '../../types/user';
import { UserRole } from '../../types/auth';

export function TeamsPage() {
  // Filter state
  const [filters, setFilters] = useState({
    search: '',
    manager_id: '',
  });

  // Managers list for filters and form dropdowns
  const [managers, setManagers] = useState<Array<{ id: string; name: string }>>([]);
  const [managerNames, setManagerNames] = useState<Record<string, string>>({});

  // Schedules list for form dropdowns
  const [schedules, setSchedules] = useState<Array<{ id: string; name: string }>>([]);

  // Load managers and schedules on mount
  useEffect(() => {
    const loadManagers = async () => {
      try {
        // Get users who can be managers (Manager, Admin, SuperAdmin roles)
        const response = await usersApi.list({ per_page: 100 });
        const managerUsers = response.data.filter(
          (u: UserResponse) =>
            u.role === UserRole.Manager ||
            u.role === UserRole.Admin ||
            u.role === UserRole.SuperAdmin
        );
        const managerList = managerUsers.map((u: UserResponse) => ({
          id: u.id,
          name: `${u.first_name} ${u.last_name}`,
        }));
        setManagers(managerList);

        // Build name lookup map
        const names: Record<string, string> = {};
        managerUsers.forEach((u: UserResponse) => {
          names[u.id] = `${u.first_name} ${u.last_name}`;
        });
        setManagerNames(names);
      } catch (err) {
        logger.error('Failed to load managers', err, { component: 'TeamsPage', action: 'loadManagers' });
      }
    };

    const loadSchedules = async () => {
      try {
        const data = await schedulesApi.list();
        setSchedules(data.map((s) => ({ id: s.id, name: s.name })));
      } catch {
        // Silently fail - schedule dropdown will just be empty
      }
    };

    loadManagers();
    loadSchedules();
  }, []);

  // Build fetch params from filters
  const fetchParams = useMemo(() => {
    const params: Record<string, string | undefined> = {};
    if (filters.search) params.search = filters.search;
    if (filters.manager_id) params.manager_id = filters.manager_id;
    return params;
  }, [filters]);

  // Fetch function for infinite scroll
  const fetchTeams = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await teamsApi.list({
        ...fetchParams,
        page: params.page,
        per_page: params.per_page,
      });
      return {
        data: response.teams,
        total: response.total,
        page: response.page,
        per_page: response.per_page,
      };
    },
    [fetchParams]
  );

  // Use infinite scroll hook
  const {
    items: teams,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<TeamResponse>({
    fetchFn: fetchTeams,
    params: fetchParams,
    perPage: 20,
  });

  // Track removed team IDs for optimistic updates
  const [removedIds, setRemovedIds] = useState<Set<string>>(new Set());
  const displayedTeams = teams.filter((t) => !removedIds.has(t.id));
  const displayTotal = Math.max(0, total - removedIds.size);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    team: TeamResponse | null;
    loading: boolean;
  }>({ open: false, team: null, loading: false });

  // Create drawer state
  const [createDrawer, setCreateDrawer] = useState<{
    open: boolean;
    loading: boolean;
    error: string;
  }>({ open: false, loading: false, error: '' });

  // Edit drawer state
  const [editDrawer, setEditDrawer] = useState<{
    open: boolean;
    team: TeamResponse | null;
    loading: boolean;
    error: string;
  }>({ open: false, team: null, loading: false, error: '' });

  // Members drawer state (placeholder for Phase 3)
  const [membersDrawer, setMembersDrawer] = useState<{
    open: boolean;
    team: TeamResponse | null;
    loading: boolean;
  }>({ open: false, team: null, loading: false });

  const hasActiveFilters = filters.search !== '' || filters.manager_id !== '';

  const handleSearchChange = (value: string) => {
    setFilters((prev) => ({ ...prev, search: value }));
    setRemovedIds(new Set());
  };

  const handleManagerChange = (value: string) => {
    setFilters((prev) => ({ ...prev, manager_id: value }));
    setRemovedIds(new Set());
  };

  // Create handlers
  const handleCreateClick = () => {
    setCreateDrawer({ open: true, loading: false, error: '' });
  };

  const handleCreateSubmit = async (data: CreateTeamRequest) => {
    setCreateDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await teamsApi.create(data);
      toast.success(`Team "${data.name}" has been created`);
      setCreateDrawer({ open: false, loading: false, error: '' });
      setRemovedIds(new Set());
      reset();
    } catch (err) {
      setCreateDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleCreateCancel = () => {
    setCreateDrawer({ open: false, loading: false, error: '' });
  };

  // Edit handlers
  const handleEdit = (team: TeamResponse) => {
    setEditDrawer({ open: true, team, loading: false, error: '' });
  };

  const handleEditSubmit = async (data: CreateTeamRequest) => {
    if (!editDrawer.team) return;

    setEditDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await teamsApi.update(editDrawer.team.id, data);
      toast.success(`Team "${data.name}" has been updated`);
      setEditDrawer({ open: false, team: null, loading: false, error: '' });
      setRemovedIds(new Set());
      reset();
    } catch (err) {
      setEditDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleEditCancel = () => {
    setEditDrawer({ open: false, team: null, loading: false, error: '' });
  };

  // Members handlers
  const handleManageMembers = (team: TeamResponse) => {
    setMembersDrawer({ open: true, team, loading: false });
  };

  const handleMembersClose = () => {
    setMembersDrawer({ open: false, team: null, loading: false });
  };

  // Delete handlers
  const handleDeleteClick = (team: TeamResponse) => {
    if (team.member_count > 0) {
      toast.error('Remove all members before deleting the team');
      return;
    }
    setDeleteDialog({ open: true, team, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.team) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await teamsApi.delete(deleteDialog.team.id);
      toast.success(`Team "${deleteDialog.team.name}" has been deleted`);
      setRemovedIds((prev) => new Set(prev).add(deleteDialog.team!.id));
      setDeleteDialog({ open: false, team: null, loading: false });
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle className="flex items-center justify-between">
              <span>Teams</span>
              {displayTotal > 0 && (
                <span className="text-sm font-normal text-muted-foreground ml-4">
                  {displayedTeams.length} of {displayTotal} {hasActiveFilters && '(filtered)'}
                </span>
              )}
            </CardTitle>
            <CardDescription>Manage teams in your organization</CardDescription>
          </div>
          <Button onClick={handleCreateClick}>Add Team</Button>
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

          <TeamFilters
            search={filters.search}
            managerId={filters.manager_id}
            onSearchChange={handleSearchChange}
            onManagerChange={handleManagerChange}
            managers={managers}
          />

          <TeamsTable
            teams={displayedTeams}
            onEdit={handleEdit}
            onManageMembers={handleManageMembers}
            onDelete={handleDeleteClick}
            isLoading={isInitialLoading}
            managerNames={managerNames}
          />

          {/* Infinite scroll elements */}
          {!isInitialLoading && displayedTeams.length > 0 && (
            <>
              <div ref={sentinelRef} className="h-4" />

              {isLoading && (
                <div className="flex items-center justify-center py-4">
                  <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
                </div>
              )}

              {!hasMore && (
                <p className="text-center text-sm text-muted-foreground py-4">
                  All teams loaded
                </p>
              )}
            </>
          )}

          <ConfirmDialog
            open={deleteDialog.open}
            onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
            title="Delete Team"
            description={
              deleteDialog.team
                ? `Are you sure you want to delete "${deleteDialog.team.name}"? This action cannot be undone.`
                : ''
            }
            confirmText="Delete"
            variant="destructive"
            onConfirm={handleDeleteConfirm}
            loading={deleteDialog.loading}
          />

          {/* Create Team Drawer */}
          <Sheet open={createDrawer.open} onOpenChange={(open) => !open && handleCreateCancel()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>Add Team</SheetTitle>
                <SheetDescription>
                  Create a new team for your organization.
                </SheetDescription>
              </SheetHeader>
              <TeamForm
                onSubmit={handleCreateSubmit}
                onCancel={handleCreateCancel}
                isLoading={createDrawer.loading}
                error={createDrawer.error}
                variant="sheet"
                managers={managers}
                schedules={schedules}
              />
            </SheetContent>
          </Sheet>

          {/* Edit Team Drawer */}
          <Sheet open={editDrawer.open} onOpenChange={(open) => !open && handleEditCancel()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>Edit Team</SheetTitle>
                <SheetDescription>
                  Update team information
                </SheetDescription>
              </SheetHeader>
              <TeamForm
                team={editDrawer.team}
                onSubmit={handleEditSubmit}
                onCancel={handleEditCancel}
                isLoading={editDrawer.loading}
                error={editDrawer.error}
                variant="sheet"
                managers={managers}
                schedules={schedules}
              />
            </SheetContent>
          </Sheet>

          {/* Members Drawer */}
          <Sheet open={membersDrawer.open} onOpenChange={(open) => !open && handleMembersClose()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>Team Members</SheetTitle>
                <SheetDescription>
                  Manage members of {membersDrawer.team?.name}
                </SheetDescription>
              </SheetHeader>
              <TeamMembersPanel
                team={membersDrawer.team}
                onClose={handleMembersClose}
                onMembersChanged={() => {
                  setRemovedIds(new Set());
                  reset();
                }}
              />
            </SheetContent>
          </Sheet>
        </CardContent>
      </Card>
    </div>
  );
}
