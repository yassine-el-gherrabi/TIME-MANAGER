/**
 * Break Policies Management Page (Admin)
 *
 * Admin page to manage break policies at organization, team, or user level.
 * Drawer components are lazy-loaded to reduce initial bundle size.
 */

import { useState, useCallback, useEffect, lazy, Suspense } from 'react';
import { toast } from 'sonner';
import { Plus, Loader2, Pencil, Trash2, Coffee, Users, Building2, User, Clock } from 'lucide-react';
import { Button } from '../../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../../components/ui/card';
import { ConfirmDialog } from '../../../components/ui/confirm-dialog';
import { Badge } from '../../../components/ui/badge';
import {
  listBreakPolicies,
  createBreakPolicy,
  updateBreakPolicy,
  deleteBreakPolicy,
  addBreakWindow,
  deleteBreakWindow,
} from '../../../api/breaks';
import { teamsApi } from '../../../api/teams';
import { usersApi } from '../../../api/users';
import { OrgTeamFilter, useOrgTeamFilter } from '../../../components/filters';
import { mapErrorToMessage } from '../../../utils/errorHandling';
import type {
  BreakPolicyResponse,
  CreateBreakPolicyRequest,
  UpdateBreakPolicyRequest,
  BreakTrackingMode,
  CreateBreakWindowRequest,
} from '../../../types/break';
import { TRACKING_MODE_OPTIONS } from '../../../types/break';
import type { TeamResponse } from '../../../types/team';
import type { UserResponse } from '../../../types/user';
import type {
  FormData,
  WindowFormData,
  FormDrawerState,
  WindowDrawerState,
  DeleteDialogState,
} from './types';
import { initialFormData, initialWindowFormData } from './types';

// Lazy-load drawer components for code splitting
const PolicyFormDrawer = lazy(() => import('./PolicyFormDrawer'));
const WindowManagementDrawer = lazy(() => import('./WindowManagementDrawer'));

// Drawer loading fallback
const DrawerFallback = () => (
  <div className="flex items-center justify-center p-8">
    <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
  </div>
);

export function BreakPoliciesPage() {
  // Org filter state
  const { selectedOrgId, setSelectedOrgId, setSelectedTeamId } = useOrgTeamFilter();

  const [policies, setPolicies] = useState<BreakPolicyResponse[]>([]);
  const [teams, setTeams] = useState<TeamResponse[]>([]);
  const [users, setUsers] = useState<UserResponse[]>([]);
  const [loading, setLoading] = useState(true);

  // Filter state
  const [filterMode, setFilterMode] = useState<BreakTrackingMode | 'all'>('all');

  // Form drawer state
  const [formDrawer, setFormDrawer] = useState<FormDrawerState>({
    open: false,
    policy: null,
    loading: false,
    error: '',
  });
  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Window form state
  const [windowDrawer, setWindowDrawer] = useState<WindowDrawerState>({
    open: false,
    policy: null,
    loading: false,
  });
  const [windowFormData, setWindowFormData] = useState<WindowFormData>(initialWindowFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<DeleteDialogState>({
    open: false,
    policy: null,
    loading: false,
  });

  // Load policies
  const loadPolicies = useCallback(async () => {
    setLoading(true);
    try {
      const data = await listBreakPolicies({
        tracking_mode: filterMode !== 'all' ? filterMode : undefined,
      });
      setPolicies(data.data);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, [filterMode]);

  // Load teams and users for form dropdowns
  const loadFormData = useCallback(async () => {
    try {
      const [teamsData, usersData] = await Promise.all([
        teamsApi.list({ per_page: 100 }),
        usersApi.list({ per_page: 100 }),
      ]);
      setTeams(teamsData.teams);
      setUsers(usersData.data);
    } catch {
      // Silently fail - form will work without team/user selection
    }
  }, []);

  useEffect(() => {
    loadPolicies();
    loadFormData();
  }, [loadPolicies, loadFormData]);

  // Handlers
  const handleCreateClick = () => {
    setFormData(initialFormData);
    setFormDrawer({ open: true, policy: null, loading: false, error: '' });
  };

  const handleEditClick = (policy: BreakPolicyResponse) => {
    const scope: FormData['scope'] = policy.user_id
      ? 'user'
      : policy.team_id
        ? 'team'
        : 'organization';
    setFormData({
      scope,
      team_id: policy.team_id || '',
      user_id: policy.user_id || '',
      name: policy.name,
      description: policy.description || '',
      tracking_mode: policy.tracking_mode,
      notify_missing_break: policy.notify_missing_break,
    });
    setFormDrawer({ open: true, policy, loading: false, error: '' });
  };

  const handleFormSubmit = async () => {
    setFormDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      if (formDrawer.policy) {
        const payload: UpdateBreakPolicyRequest = {
          name: formData.name,
          description: formData.description || null,
          tracking_mode: formData.tracking_mode,
          notify_missing_break: formData.notify_missing_break,
        };
        await updateBreakPolicy(formDrawer.policy.id, payload);
        toast.success('Break policy updated');
      } else {
        const payload: CreateBreakPolicyRequest = {
          name: formData.name,
          description: formData.description || null,
          tracking_mode: formData.tracking_mode,
          notify_missing_break: formData.notify_missing_break,
        };
        if (formData.scope === 'team' && formData.team_id) {
          payload.team_id = formData.team_id;
        } else if (formData.scope === 'user' && formData.user_id) {
          payload.user_id = formData.user_id;
        }
        await createBreakPolicy(payload);
        toast.success('Break policy created');
      }
      setFormDrawer({ open: false, policy: null, loading: false, error: '' });
      loadPolicies();
    } catch (err) {
      setFormDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleFormCancel = () => {
    setFormDrawer({ open: false, policy: null, loading: false, error: '' });
  };

  const handleDeleteClick = (policy: BreakPolicyResponse) => {
    setDeleteDialog({ open: true, policy, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.policy) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await deleteBreakPolicy(deleteDialog.policy.id);
      toast.success('Break policy deleted');
      setDeleteDialog({ open: false, policy: null, loading: false });
      loadPolicies();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  // Window management handlers
  const handleManageWindows = (policy: BreakPolicyResponse) => {
    setWindowDrawer({ open: true, policy, loading: false });
    setWindowFormData(initialWindowFormData);
  };

  const handleAddWindow = async () => {
    if (!windowDrawer.policy || windowFormData.selectedDays.length === 0) {
      toast.error('Please select at least one day');
      return;
    }

    setWindowDrawer((prev) => ({ ...prev, loading: true }));
    try {
      const promises = windowFormData.selectedDays.map((day) => {
        const payload: CreateBreakWindowRequest = {
          day_of_week: day,
          window_start: windowFormData.window_start,
          window_end: windowFormData.window_end,
          min_duration_minutes: windowFormData.min_duration_minutes,
          max_duration_minutes: windowFormData.max_duration_minutes,
          is_mandatory: windowFormData.is_mandatory,
        };
        return addBreakWindow(windowDrawer.policy!.id, payload);
      });

      await Promise.all(promises);
      toast.success(`Break window${windowFormData.selectedDays.length > 1 ? 's' : ''} added`);
      setWindowFormData(initialWindowFormData);
      loadPolicies();

      const updatedData = await listBreakPolicies({ page: 1, per_page: 100 });
      const updatedPolicy = updatedData.data.find((p) => p.id === windowDrawer.policy?.id);
      if (updatedPolicy) {
        setWindowDrawer((prev) => ({ ...prev, policy: updatedPolicy, loading: false }));
      }
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setWindowDrawer((prev) => ({ ...prev, loading: false }));
    }
  };

  const handleDeleteWindow = async (windowId: string) => {
    if (!windowDrawer.policy) return;

    try {
      await deleteBreakWindow(windowDrawer.policy.id, windowId);
      toast.success('Break window deleted');
      loadPolicies();

      const updatedData = await listBreakPolicies({ page: 1, per_page: 100 });
      const updatedPolicy = updatedData.data.find((p) => p.id === windowDrawer.policy?.id);
      if (updatedPolicy) {
        setWindowDrawer((prev) => ({ ...prev, policy: updatedPolicy }));
      }
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    }
  };

  // Get scope label and icon for a policy
  const getScopeInfo = (policy: BreakPolicyResponse) => {
    if (policy.user_id) {
      return { label: policy.user_name || 'User', icon: User, scope: 'User' };
    }
    if (policy.team_id) {
      return { label: policy.team_name || 'Team', icon: Users, scope: 'Team' };
    }
    return { label: policy.organization_name || 'Organization', icon: Building2, scope: 'Organization' };
  };

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <Coffee className="h-5 w-5" />
              Break Policies
            </CardTitle>
            <CardDescription>
              Configure break tracking modes and windows (Organization → Team → User cascade)
            </CardDescription>
          </div>
          <Button onClick={handleCreateClick}>
            <Plus className="h-4 w-4 mr-2" />
            Add Policy
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
              <label className="text-sm text-muted-foreground">Tracking Mode</label>
              <select
                value={filterMode}
                onChange={(e) => setFilterMode(e.target.value as BreakTrackingMode | 'all')}
                className="flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              >
                <option value="all">All modes</option>
                <option value="auto_deduct">Auto Deduct</option>
                <option value="explicit_tracking">Explicit Tracking</option>
              </select>
            </div>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
          ) : policies.length === 0 ? (
            <div className="text-center py-8">
              <Coffee className="h-12 w-12 text-muted-foreground mx-auto mb-3" />
              <p className="text-sm text-muted-foreground mb-4">No break policies configured</p>
              <Button onClick={handleCreateClick}>
                <Plus className="h-4 w-4 mr-2" />
                Add Policy
              </Button>
            </div>
          ) : (
            <div className="rounded-md border">
              <table className="w-full">
                <thead>
                  <tr className="border-b bg-muted/50">
                    <th className="px-4 py-3 text-left text-sm font-medium">Scope</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Name</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Mode</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Windows</th>
                    <th className="px-4 py-3 text-right text-sm font-medium">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {policies.map((policy) => {
                    const scopeInfo = getScopeInfo(policy);
                    const ScopeIcon = scopeInfo.icon;
                    const modeOption = TRACKING_MODE_OPTIONS.find((m) => m.value === policy.tracking_mode);

                    return (
                      <tr key={policy.id} className="border-b last:border-b-0">
                        <td className="px-4 py-3">
                          <div className="flex items-center gap-2">
                            <ScopeIcon className="h-4 w-4 text-muted-foreground" />
                            <div>
                              <span className="font-medium">{scopeInfo.label}</span>
                              <span className="text-xs text-muted-foreground ml-2">({scopeInfo.scope})</span>
                            </div>
                          </div>
                        </td>
                        <td className="px-4 py-3">
                          <span className="font-medium">{policy.name}</span>
                          {policy.description && (
                            <p className="text-xs text-muted-foreground truncate max-w-[200px]">
                              {policy.description}
                            </p>
                          )}
                        </td>
                        <td className="px-4 py-3">
                          <div className="flex items-center gap-2 flex-wrap">
                            <Badge variant={policy.tracking_mode === 'auto_deduct' ? 'secondary' : 'outline'}>
                              {modeOption?.label || policy.tracking_mode}
                            </Badge>
                            {policy.notify_missing_break && (
                              <Badge variant="outline" className="text-xs">Notify</Badge>
                            )}
                            {!policy.is_active && (
                              <Badge variant="destructive" className="text-xs">Inactive</Badge>
                            )}
                          </div>
                        </td>
                        <td className="px-4 py-3">
                          <Button
                            variant="outline"
                            size="sm"
                            onClick={() => handleManageWindows(policy)}
                            className="gap-1.5"
                          >
                            <Clock className="h-3.5 w-3.5" />
                            {policy.windows.length > 0
                              ? `${policy.windows.length} window${policy.windows.length !== 1 ? 's' : ''}`
                              : 'Add windows'}
                          </Button>
                        </td>
                        <td className="px-4 py-3 text-right">
                          <div className="flex justify-end gap-2">
                            <Button variant="ghost" size="icon" onClick={() => handleEditClick(policy)}>
                              <Pencil className="h-4 w-4" />
                            </Button>
                            <Button variant="ghost" size="icon" onClick={() => handleDeleteClick(policy)}>
                              <Trash2 className="h-4 w-4 text-destructive" />
                            </Button>
                          </div>
                        </td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Lazy-loaded Form Drawer */}
      {formDrawer.open && (
        <Suspense fallback={<DrawerFallback />}>
          <PolicyFormDrawer
            formDrawer={formDrawer}
            formData={formData}
            teams={teams}
            users={users}
            onFormDataChange={setFormData}
            onSubmit={handleFormSubmit}
            onCancel={handleFormCancel}
          />
        </Suspense>
      )}

      {/* Lazy-loaded Window Management Drawer */}
      {windowDrawer.open && (
        <Suspense fallback={<DrawerFallback />}>
          <WindowManagementDrawer
            windowDrawer={windowDrawer}
            windowFormData={windowFormData}
            onWindowDrawerChange={(state) => setWindowDrawer((prev) => ({ ...prev, ...state }))}
            onWindowFormDataChange={setWindowFormData}
            onAddWindow={handleAddWindow}
            onDeleteWindow={handleDeleteWindow}
          />
        </Suspense>
      )}

      {/* Delete Confirmation */}
      <ConfirmDialog
        open={deleteDialog.open}
        onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
        title="Delete Break Policy"
        description={
          deleteDialog.policy
            ? `Are you sure you want to delete "${deleteDialog.policy.name}"? This will also delete all associated break windows.`
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

export default BreakPoliciesPage;
