/**
 * Clock Restrictions Management Page (Admin)
 *
 * Admin page to manage clock-in/out restrictions at organization, team, or user level.
 * Drawers and table are lazy-loaded for code splitting.
 */

import { useState, useCallback, useEffect, lazy, Suspense } from 'react';
import { toast } from 'sonner';
import { Plus, Loader2, Clock } from 'lucide-react';
import { Button } from '../../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../../components/ui/card';
import { ConfirmDialog } from '../../../components/ui/confirm-dialog';
import { clockRestrictionsApi } from '../../../api/clockRestrictions';
import { teamsApi } from '../../../api/teams';
import { usersApi } from '../../../api/users';
import { organizationsApi } from '../../../api/organizations';
import { OrgTeamFilter, useOrgTeamFilter } from '../../../components/filters';
import { useCurrentUser } from '../../../hooks/useAuth';
import { mapErrorToMessage } from '../../../utils/errorHandling';
import type {
  ClockRestrictionResponse,
  CreateClockRestrictionRequest,
  UpdateClockRestrictionRequest,
  ClockRestrictionMode,
} from '../../../types/clockRestriction';
import type { TeamResponse } from '../../../types/team';
import type { UserResponse } from '../../../types/user';
import type { OrganizationResponse } from '../../../types/organization';
import { UserRole } from '../../../types/auth';
import type { FormData, FormDrawerState, DeleteDialogState } from './types';
import { initialFormData } from './types';

// Lazy loaded components for code splitting
const RestrictionFormDrawer = lazy(() => import('./RestrictionFormDrawer'));
const RestrictionsTable = lazy(() => import('./RestrictionsTable'));

// Loading fallback for lazy components
const DrawerFallback = () => (
  <div className="flex items-center justify-center p-8">
    <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
  </div>
);

export function ClockRestrictionsPage() {
  // Current user for role check
  const user = useCurrentUser();
  const isSuperAdmin = user?.role === UserRole.SuperAdmin;

  // Org filter state
  const { selectedOrgId, setSelectedOrgId, setSelectedTeamId } = useOrgTeamFilter();

  const [restrictions, setRestrictions] = useState<ClockRestrictionResponse[]>([]);
  const [organizations, setOrganizations] = useState<OrganizationResponse[]>([]);
  const [teams, setTeams] = useState<TeamResponse[]>([]);
  const [users, setUsers] = useState<UserResponse[]>([]);
  const [loading, setLoading] = useState(true);

  // Filter state
  const [filterMode, setFilterMode] = useState<ClockRestrictionMode | 'all'>('all');

  // Form drawer state
  const [formDrawer, setFormDrawer] = useState<FormDrawerState>({
    open: false,
    restriction: null,
    loading: false,
    error: '',
  });

  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<DeleteDialogState>({
    open: false,
    restriction: null,
    loading: false,
  });

  // Load restrictions
  const loadRestrictions = useCallback(async () => {
    setLoading(true);
    try {
      const data = await clockRestrictionsApi.list({
        mode: filterMode !== 'all' ? filterMode : undefined,
      });
      setRestrictions(data);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, [filterMode]);

  // Load organizations, teams and users for form dropdowns
  const loadFormData = useCallback(async () => {
    try {
      const promises: Promise<unknown>[] = [
        teamsApi.list({ per_page: 100 }),
        usersApi.list({ per_page: 100 }),
      ];

      // Load organizations only for SuperAdmin
      if (isSuperAdmin) {
        promises.push(organizationsApi.list({ per_page: 100 }));
      }

      const results = await Promise.all(promises);
      const teamsData = results[0] as { teams: TeamResponse[] };
      const usersData = results[1] as { data: UserResponse[] };

      setTeams(teamsData.teams);
      setUsers(usersData.data);

      if (isSuperAdmin && results[2]) {
        const orgsData = results[2] as { data: OrganizationResponse[] };
        setOrganizations(orgsData.data);
      }
    } catch {
      // Silently fail - form will work without team/user selection
    }
  }, [isSuperAdmin]);

  useEffect(() => {
    loadRestrictions();
    loadFormData();
  }, [loadRestrictions, loadFormData]);

  // Handlers
  const handleCreateClick = () => {
    setFormData(initialFormData);
    setFormDrawer({ open: true, restriction: null, loading: false, error: '' });
  };

  const handleEditClick = (restriction: ClockRestrictionResponse) => {
    const scope: FormData['scope'] = restriction.user_id
      ? 'user'
      : restriction.team_id
        ? 'team'
        : 'organization';
    setFormData({
      organization_id: restriction.organization_id || '',
      scope,
      team_id: restriction.team_id || '',
      user_id: restriction.user_id || '',
      mode: restriction.mode,
      clock_in_earliest: restriction.clock_in_earliest || '',
      clock_in_latest: restriction.clock_in_latest || '',
      clock_out_earliest: restriction.clock_out_earliest || '',
      clock_out_latest: restriction.clock_out_latest || '',
      enforce_schedule: restriction.enforce_schedule,
      require_manager_approval: restriction.require_manager_approval,
      max_daily_clock_events: restriction.max_daily_clock_events?.toString() || '',
    });
    setFormDrawer({ open: true, restriction, loading: false, error: '' });
  };

  const handleFormSubmit = async () => {
    setFormDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      const payload: CreateClockRestrictionRequest | UpdateClockRestrictionRequest = {
        mode: formData.mode,
        clock_in_earliest: formData.clock_in_earliest || null,
        clock_in_latest: formData.clock_in_latest || null,
        clock_out_earliest: formData.clock_out_earliest || null,
        clock_out_latest: formData.clock_out_latest || null,
        enforce_schedule: formData.enforce_schedule,
        require_manager_approval: formData.require_manager_approval,
        max_daily_clock_events: formData.max_daily_clock_events
          ? parseInt(formData.max_daily_clock_events, 10)
          : null,
      };

      if (formDrawer.restriction) {
        // Update
        await clockRestrictionsApi.update(formDrawer.restriction.id, payload);
        toast.success('Clock restriction updated');
      } else {
        // Create - add scope and organization fields
        const createPayload = payload as CreateClockRestrictionRequest;

        // Add organization_id for SuperAdmin
        if (isSuperAdmin && formData.organization_id) {
          createPayload.organization_id = formData.organization_id;
        }

        // Add team or user scope
        if (formData.scope === 'team' && formData.team_id) {
          createPayload.team_id = formData.team_id;
        } else if (formData.scope === 'user' && formData.user_id) {
          createPayload.user_id = formData.user_id;
        }
        await clockRestrictionsApi.create(createPayload);
        toast.success('Clock restriction created');
      }
      setFormDrawer({ open: false, restriction: null, loading: false, error: '' });
      loadRestrictions();
    } catch (err) {
      setFormDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleFormCancel = () => {
    setFormDrawer({ open: false, restriction: null, loading: false, error: '' });
  };

  const handleDeleteClick = (restriction: ClockRestrictionResponse) => {
    setDeleteDialog({ open: true, restriction, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.restriction) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await clockRestrictionsApi.delete(deleteDialog.restriction.id);
      toast.success('Clock restriction deleted');
      setDeleteDialog({ open: false, restriction: null, loading: false });
      loadRestrictions();
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
            <CardTitle className="flex items-center gap-2">
              <Clock className="h-5 w-5" />
              Clock Restrictions
            </CardTitle>
            <CardDescription>
              Configure when users can clock in/out (Organization → Team → User cascade)
            </CardDescription>
          </div>
          <Button onClick={handleCreateClick}>
            <Plus className="h-4 w-4 mr-2" />
            Add Restriction
          </Button>
        </CardHeader>
        <CardContent>
          <OrgTeamFilter
            showTeamFilter={false}
            selectedOrgId={selectedOrgId}
            selectedTeamId=""
            onOrgChange={setSelectedOrgId}
            onTeamChange={setSelectedTeamId}
            className="mb-4 pb-4 border-b"
          />

          {/* Filters */}
          <div className="flex flex-wrap items-center gap-4 mb-6">
            <div className="space-y-1">
              <label className="text-sm text-muted-foreground">Mode</label>
              <select
                value={filterMode}
                onChange={(e) => setFilterMode(e.target.value as ClockRestrictionMode | 'all')}
                className="flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              >
                <option value="all">All modes</option>
                <option value="strict">Strict</option>
                <option value="flexible">Flexible</option>
                <option value="unrestricted">Unrestricted</option>
              </select>
            </div>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
          ) : restrictions.length === 0 ? (
            <div className="text-center py-8">
              <Clock className="h-12 w-12 text-muted-foreground mx-auto mb-3" />
              <p className="text-sm text-muted-foreground mb-4">
                No clock restrictions configured
              </p>
              <Button onClick={handleCreateClick}>
                <Plus className="h-4 w-4 mr-2" />
                Add Restriction
              </Button>
            </div>
          ) : (
            <Suspense fallback={<DrawerFallback />}>
              <RestrictionsTable
                restrictions={restrictions}
                onEdit={handleEditClick}
                onDelete={handleDeleteClick}
              />
            </Suspense>
          )}
        </CardContent>
      </Card>

      {/* Form Drawer - Lazy loaded */}
      {formDrawer.open && (
        <Suspense fallback={<DrawerFallback />}>
          <RestrictionFormDrawer
            formDrawer={formDrawer}
            formData={formData}
            organizations={organizations}
            teams={teams}
            users={users}
            isSuperAdmin={isSuperAdmin}
            onFormDataChange={setFormData}
            onSubmit={handleFormSubmit}
            onCancel={handleFormCancel}
          />
        </Suspense>
      )}

      {/* Delete Confirmation */}
      <ConfirmDialog
        open={deleteDialog.open}
        onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
        title="Delete Clock Restriction"
        description={
          deleteDialog.restriction
            ? `Are you sure you want to delete this clock restriction?`
            : ''
        }
        confirmText="Delete"
        variant="destructive"
        onConfirm={handleDeleteConfirm}
        loading={deleteDialog.loading}
      />
    </div>
  );
}

export default ClockRestrictionsPage;
