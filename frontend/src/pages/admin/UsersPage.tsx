/**
 * Users Page
 *
 * Admin page to manage organization users with infinite scroll.
 * Supports filtering by search and role, soft delete, and restore.
 * User creation and editing are done via side sheets.
 */

import { useState, useCallback, useMemo, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { toast } from 'sonner';
import { Loader2, Download } from 'lucide-react';
import { reportsApi } from '../../api/reports';
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
import { UsersTable, UserFilters, UserForm } from '../../components/admin';
import { OrgTeamFilter, useOrgTeamFilter } from '../../components/filters';
import { usersApi } from '../../api/users';
import { schedulesApi } from '../../api/schedules';
import { useAuthStore } from '../../stores/authStore';
import { useInfiniteScroll } from '../../hooks/useInfiniteScroll';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { UserResponse, CreateUserRequest } from '../../types/user';
import type { ScheduleOption } from '../../components/admin/UserForm';
import { UserRole } from '../../types/auth';

export function UsersPage() {
  const { t } = useTranslation();
  const currentUser = useAuthStore((state) => state.user);

  // Org/Team filter state
  const {
    selectedOrgId,
    selectedTeamId,
    setSelectedOrgId,
    setSelectedTeamId,
  } = useOrgTeamFilter();

  // Schedules for assignment dropdown
  const [schedules, setSchedules] = useState<ScheduleOption[]>([]);

  // Load schedules on mount (Admin+ can assign schedules)
  useEffect(() => {
    const loadSchedules = async () => {
      try {
        const data = await schedulesApi.list();
        setSchedules(data.map((s) => ({ id: s.id, name: s.name })));
      } catch {
        // Silently fail - schedule dropdown will just be empty
      }
    };
    loadSchedules();
  }, []);

  // Filter state
  const [filters, setFilters] = useState({
    search: '',
    role: '' as UserRole | '',
    showDeleted: false,
  });

  // Build fetch params from filters
  const fetchParams = useMemo(() => {
    const params: Record<string, string | boolean | undefined> = {};
    if (filters.search) params.search = filters.search;
    if (filters.role) params.role = filters.role;
    if (filters.showDeleted) params.include_deleted = true;
    if (selectedOrgId) params.organization_id = selectedOrgId;
    if (selectedTeamId) params.team_id = selectedTeamId;
    return params;
  }, [filters, selectedOrgId, selectedTeamId]);

  // Fetch function for infinite scroll
  const fetchUsers = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await usersApi.list({
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
    items: users,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<UserResponse>({
    fetchFn: fetchUsers,
    params: fetchParams,
    perPage: 20,
  });

  // Track removed user IDs for optimistic updates
  const [removedIds, setRemovedIds] = useState<Set<string>>(new Set());
  const displayedUsers = users.filter((u) => !removedIds.has(u.id));
  const displayTotal = Math.max(0, total - removedIds.size);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    user: UserResponse | null;
    loading: boolean;
  }>({ open: false, user: null, loading: false });

  // Restore dialog state
  const [restoreDialog, setRestoreDialog] = useState<{
    open: boolean;
    user: UserResponse | null;
    loading: boolean;
  }>({ open: false, user: null, loading: false });

  // Create drawer state
  const [createDrawer, setCreateDrawer] = useState<{
    open: boolean;
    loading: boolean;
    error: string;
  }>({ open: false, loading: false, error: '' });

  // Edit drawer state
  const [editDrawer, setEditDrawer] = useState<{
    open: boolean;
    user: UserResponse | null;
    loading: boolean;
    error: string;
  }>({ open: false, user: null, loading: false, error: '' });

  const hasActiveFilters = filters.search !== '' || filters.role !== '' || filters.showDeleted || selectedOrgId !== '' || selectedTeamId !== '';

  const handleSearchChange = (value: string) => {
    setFilters((prev) => ({ ...prev, search: value }));
    setRemovedIds(new Set());
  };

  const handleRoleChange = (value: UserRole | '') => {
    setFilters((prev) => ({ ...prev, role: value }));
    setRemovedIds(new Set());
  };

  const handleShowDeletedChange = (value: boolean) => {
    setFilters((prev) => ({ ...prev, showDeleted: value }));
    setRemovedIds(new Set());
  };

  // Create handlers
  const handleCreateClick = () => {
    setCreateDrawer({ open: true, loading: false, error: '' });
  };

  const handleCreateSubmit = async (data: CreateUserRequest) => {
    setCreateDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await usersApi.create(data);
      toast.success(t('success.created'));
      setCreateDrawer({ open: false, loading: false, error: '' });
      // Reset the list to show the new user
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
  const handleEdit = (user: UserResponse) => {
    setEditDrawer({ open: true, user, loading: false, error: '' });
  };

  const handleEditSubmit = async (data: CreateUserRequest) => {
    if (!editDrawer.user) return;

    setEditDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await usersApi.update(editDrawer.user.id, data);
      toast.success(t('success.saved'));
      setEditDrawer({ open: false, user: null, loading: false, error: '' });
      // Reset the list to refresh with updated data
      setRemovedIds(new Set());
      reset();
    } catch (err) {
      setEditDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleEditCancel = () => {
    setEditDrawer({ open: false, user: null, loading: false, error: '' });
  };

  // Delete handlers
  const handleDeleteClick = (user: UserResponse) => {
    setDeleteDialog({ open: true, user, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.user) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await usersApi.delete(deleteDialog.user.id);
      toast.success(t('success.deleted'));
      // Optimistic removal from local list (unless showing deleted)
      if (!filters.showDeleted) {
        setRemovedIds((prev) => new Set(prev).add(deleteDialog.user!.id));
      } else {
        // Refresh to show updated deleted_at status
        reset();
      }
      setDeleteDialog({ open: false, user: null, loading: false });
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  // Restore handlers
  const handleRestoreClick = (user: UserResponse) => {
    setRestoreDialog({ open: true, user, loading: false });
  };

  const handleRestoreConfirm = async () => {
    if (!restoreDialog.user) return;

    setRestoreDialog((prev) => ({ ...prev, loading: true }));
    try {
      await usersApi.restore(restoreDialog.user.id);
      toast.success(t('success.restored'));
      // Reset the list to refresh with restored user
      setRemovedIds(new Set());
      reset();
      setRestoreDialog({ open: false, user: null, loading: false });
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setRestoreDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const handleResendInvite = async (user: UserResponse) => {
    try {
      await usersApi.resendInvite(user.id);
      toast.success(t('success.inviteResent'));
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    }
  };

  // Schedule assignment callback
  const handleScheduleAssign = async (userId: string, scheduleId: string | null) => {
    try {
      if (scheduleId === null) {
        await schedulesApi.unassignFromUser(userId);
        toast.success(t('success.saved'));
      } else {
        await schedulesApi.assignToUser(userId, { schedule_id: scheduleId });
        toast.success(t('success.saved'));
      }
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    }
  };

  // Export state
  const [exporting, setExporting] = useState(false);

  const handleExport = async () => {
    setExporting(true);
    try {
      await reportsApi.exportCsv('users');
      toast.success(t('success.exported'));
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setExporting(false);
    }
  };

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle className="flex items-center justify-between">
              <span>{t('users.title')}</span>
              {displayTotal > 0 && (
                <span className="text-sm font-normal text-muted-foreground ml-4">
                  {hasActiveFilters ? t('users.usersCountFiltered', { count: displayTotal }) : t('users.usersCount', { count: displayTotal })}
                </span>
              )}
            </CardTitle>
            <CardDescription>{t('users.description')}</CardDescription>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" onClick={handleExport} disabled={exporting}>
              {exporting ? <Loader2 className="h-4 w-4 animate-spin" /> : <Download className="h-4 w-4" />}
              <span className="ml-2">{t('common.export')}</span>
            </Button>
            <Button onClick={handleCreateClick}>{t('users.addUser')}</Button>
          </div>
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

          <OrgTeamFilter
            showTeamFilter={true}
            selectedOrgId={selectedOrgId}
            selectedTeamId={selectedTeamId}
            onOrgChange={setSelectedOrgId}
            onTeamChange={setSelectedTeamId}
            className="mb-4 pb-4 border-b"
          />

          <UserFilters
            search={filters.search}
            role={filters.role}
            showDeleted={filters.showDeleted}
            onSearchChange={handleSearchChange}
            onRoleChange={handleRoleChange}
            onShowDeletedChange={handleShowDeletedChange}
          />

          <UsersTable
            users={displayedUsers}
            currentUserId={currentUser?.id}
            currentUserRole={currentUser?.role}
            onEdit={handleEdit}
            onDelete={handleDeleteClick}
            onResendInvite={handleResendInvite}
            onRestore={handleRestoreClick}
            isLoading={isInitialLoading}
            showOrganization={currentUser?.role === UserRole.SuperAdmin}
          />

          {/* Infinite scroll elements */}
          {!isInitialLoading && displayedUsers.length > 0 && (
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
                  {t('users.allUsersLoaded')}
                </p>
              )}
            </>
          )}

          {/* Delete Confirmation Dialog */}
          <ConfirmDialog
            open={deleteDialog.open}
            onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
            title={t('users.deleteUser')}
            description={
              deleteDialog.user
                ? t('users.deleteConfirmation', { name: `${deleteDialog.user.first_name} ${deleteDialog.user.last_name}` })
                : ''
            }
            confirmText={t('common.delete')}
            variant="destructive"
            onConfirm={handleDeleteConfirm}
            loading={deleteDialog.loading}
          />

          {/* Restore Confirmation Dialog */}
          <ConfirmDialog
            open={restoreDialog.open}
            onOpenChange={(open) => setRestoreDialog((prev) => ({ ...prev, open }))}
            title={t('users.restoreUser')}
            description={
              restoreDialog.user
                ? t('users.restoreConfirmation', { name: `${restoreDialog.user.first_name} ${restoreDialog.user.last_name}` })
                : ''
            }
            confirmText={t('common.restore')}
            onConfirm={handleRestoreConfirm}
            loading={restoreDialog.loading}
          />

          {/* Create User Drawer */}
          <Sheet open={createDrawer.open} onOpenChange={(open) => !open && handleCreateCancel()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>{t('users.addUser')}</SheetTitle>
                <SheetDescription>
                  {t('users.addUserDescription')}
                </SheetDescription>
              </SheetHeader>
              <UserForm
                onSubmit={handleCreateSubmit}
                onCancel={handleCreateCancel}
                isLoading={createDrawer.loading}
                error={createDrawer.error}
                variant="sheet"
              />
            </SheetContent>
          </Sheet>

          {/* Edit User Drawer */}
          <Sheet open={editDrawer.open} onOpenChange={(open) => !open && handleEditCancel()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>{t('users.editUser')}</SheetTitle>
                <SheetDescription>
                  {t('users.editUserDescription')}
                </SheetDescription>
              </SheetHeader>
              <UserForm
                user={editDrawer.user}
                onSubmit={handleEditSubmit}
                onCancel={handleEditCancel}
                isLoading={editDrawer.loading}
                error={editDrawer.error}
                variant="sheet"
                schedules={schedules}
                onScheduleAssign={handleScheduleAssign}
              />
            </SheetContent>
          </Sheet>
        </CardContent>
      </Card>
    </div>
  );
}
