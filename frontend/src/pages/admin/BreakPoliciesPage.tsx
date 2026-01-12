/**
 * Break Policies Management Page (Admin)
 *
 * Admin page to manage break policies at organization, team, or user level.
 */

import { useState, useCallback, useEffect } from 'react';
import { toast } from 'sonner';
import { Plus, Loader2, Pencil, Trash2, Coffee, Users, Building2, User, Clock } from 'lucide-react';
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
import { Input } from '../../components/ui/input';
import { Label } from '../../components/ui/label';
import { Switch } from '../../components/ui/switch';
import { Badge } from '../../components/ui/badge';
import { Textarea } from '../../components/ui/textarea';
import { Checkbox } from '../../components/ui/checkbox';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../components/ui/select';
import {
  listBreakPolicies,
  createBreakPolicy,
  updateBreakPolicy,
  deleteBreakPolicy,
  addBreakWindow,
  deleteBreakWindow,
} from '../../api/breaks';
import { teamsApi } from '../../api/teams';
import { usersApi } from '../../api/users';
import { OrgTeamFilter, useOrgTeamFilter } from '../../components/filters';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type {
  BreakPolicyResponse,
  CreateBreakPolicyRequest,
  UpdateBreakPolicyRequest,
  BreakTrackingMode,
  CreateBreakWindowRequest,
  BreakWindowResponse,
} from '../../types/break';
import { TRACKING_MODE_OPTIONS, DAYS_OF_WEEK, getDayLabel, formatBreakDuration } from '../../types/break';
import type { TeamResponse } from '../../types/team';
import type { UserResponse } from '../../types/user';

interface FormData {
  scope: 'organization' | 'team' | 'user';
  team_id: string;
  user_id: string;
  name: string;
  description: string;
  tracking_mode: BreakTrackingMode;
  notify_missing_break: boolean;
}

interface WindowFormData {
  selectedDays: number[];
  window_start: string;
  window_end: string;
  min_duration_minutes: number;
  max_duration_minutes: number;
  is_mandatory: boolean;
}

const initialFormData: FormData = {
  scope: 'organization',
  team_id: '',
  user_id: '',
  name: '',
  description: '',
  tracking_mode: 'auto_deduct',
  notify_missing_break: false,
};

const initialWindowFormData: WindowFormData = {
  selectedDays: [],
  window_start: '12:00',
  window_end: '14:00',
  min_duration_minutes: 30,
  max_duration_minutes: 60,
  is_mandatory: true,
};

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
  const [formDrawer, setFormDrawer] = useState<{
    open: boolean;
    policy: BreakPolicyResponse | null;
    loading: boolean;
    error: string;
  }>({ open: false, policy: null, loading: false, error: '' });

  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Window form state (for managing break windows)
  const [windowDrawer, setWindowDrawer] = useState<{
    open: boolean;
    policy: BreakPolicyResponse | null;
    loading: boolean;
  }>({ open: false, policy: null, loading: false });
  const [windowFormData, setWindowFormData] = useState<WindowFormData>(initialWindowFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    policy: BreakPolicyResponse | null;
    loading: boolean;
  }>({ open: false, policy: null, loading: false });

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
        // Update
        const payload: UpdateBreakPolicyRequest = {
          name: formData.name,
          description: formData.description || null,
          tracking_mode: formData.tracking_mode,
          notify_missing_break: formData.notify_missing_break,
        };
        await updateBreakPolicy(formDrawer.policy.id, payload);
        toast.success('Break policy updated');
      } else {
        // Create
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
      // Create a window for each selected day
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
      toast.success(`Break window${windowFormData.selectedDays.length > 1 ? 's' : ''} added for ${windowFormData.selectedDays.length} day${windowFormData.selectedDays.length > 1 ? 's' : ''}`);
      setWindowFormData(initialWindowFormData);
      loadPolicies();
      // Update the policy in the drawer
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
      // Update the policy in the drawer
      const updatedData = await listBreakPolicies({ page: 1, per_page: 100 });
      const updatedPolicy = updatedData.data.find((p) => p.id === windowDrawer.policy?.id);
      if (updatedPolicy) {
        setWindowDrawer((prev) => ({ ...prev, policy: updatedPolicy }));
      }
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    }
  };

  const isEditing = !!formDrawer.policy;

  // Get scope label and icon for a policy
  const getScopeInfo = (policy: BreakPolicyResponse) => {
    if (policy.user_id) {
      return {
        label: policy.user_name || 'User',
        icon: User,
        scope: 'User',
      };
    }
    if (policy.team_id) {
      return {
        label: policy.team_name || 'Team',
        icon: Users,
        scope: 'Team',
      };
    }
    return {
      label: policy.organization_name || 'Organization',
      icon: Building2,
      scope: 'Organization',
    };
  };

  // Format time for display
  const formatTime = (time: string) => {
    return time.substring(0, 5); // "HH:MM:SS" -> "HH:MM"
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
              <p className="text-sm text-muted-foreground mb-4">
                No break policies configured
              </p>
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
                    const modeOption = TRACKING_MODE_OPTIONS.find(
                      (m) => m.value === policy.tracking_mode
                    );

                    return (
                      <tr key={policy.id} className="border-b last:border-b-0">
                        <td className="px-4 py-3">
                          <div className="flex items-center gap-2">
                            <ScopeIcon className="h-4 w-4 text-muted-foreground" />
                            <div>
                              <span className="font-medium">{scopeInfo.label}</span>
                              <span className="text-xs text-muted-foreground ml-2">
                                ({scopeInfo.scope})
                              </span>
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
                            <Badge
                              variant={
                                policy.tracking_mode === 'auto_deduct'
                                  ? 'secondary'
                                  : 'outline'
                              }
                            >
                              {modeOption?.label || policy.tracking_mode}
                            </Badge>
                            {policy.notify_missing_break && (
                              <Badge variant="outline" className="text-xs">
                                Notify
                              </Badge>
                            )}
                            {!policy.is_active && (
                              <Badge variant="destructive" className="text-xs">
                                Inactive
                              </Badge>
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
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => handleEditClick(policy)}
                            >
                              <Pencil className="h-4 w-4" />
                            </Button>
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => handleDeleteClick(policy)}
                            >
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

      {/* Form Drawer */}
      <Sheet open={formDrawer.open} onOpenChange={(open) => !open && handleFormCancel()}>
        <SheetContent className="overflow-y-auto sm:max-w-lg">
          <SheetHeader>
            <SheetTitle>{isEditing ? 'Edit Break Policy' : 'Add Break Policy'}</SheetTitle>
            <SheetDescription>
              {isEditing
                ? 'Update the break policy settings'
                : 'Configure how breaks are tracked and deducted'}
            </SheetDescription>
          </SheetHeader>

          <div className="space-y-4 py-4">
            {formDrawer.error && (
              <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
                {formDrawer.error}
              </div>
            )}

            {/* Scope Selection (only for create) */}
            {!isEditing && (
              <div className="space-y-2">
                <Label>Scope</Label>
                <Select
                  value={formData.scope}
                  onValueChange={(value: FormData['scope']) =>
                    setFormData((prev) => ({ ...prev, scope: value, team_id: '', user_id: '' }))
                  }
                  disabled={formDrawer.loading}
                >
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="organization">Organization-wide</SelectItem>
                    <SelectItem value="team">Specific Team</SelectItem>
                    <SelectItem value="user">Specific User</SelectItem>
                  </SelectContent>
                </Select>
                <p className="text-xs text-muted-foreground">
                  User settings override Team, Team overrides Organization
                </p>
              </div>
            )}

            {/* Team Selection */}
            {!isEditing && formData.scope === 'team' && (
              <div className="space-y-2">
                <Label>Team</Label>
                <Select
                  value={formData.team_id}
                  onValueChange={(value) => setFormData((prev) => ({ ...prev, team_id: value }))}
                  disabled={formDrawer.loading}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select a team" />
                  </SelectTrigger>
                  <SelectContent>
                    {teams.map((team) => (
                      <SelectItem key={team.id} value={team.id}>
                        {team.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            )}

            {/* User Selection */}
            {!isEditing && formData.scope === 'user' && (
              <div className="space-y-2">
                <Label>User</Label>
                <Select
                  value={formData.user_id}
                  onValueChange={(value) => setFormData((prev) => ({ ...prev, user_id: value }))}
                  disabled={formDrawer.loading}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select a user" />
                  </SelectTrigger>
                  <SelectContent>
                    {users.map((user) => (
                      <SelectItem key={user.id} value={user.id}>
                        {user.first_name} {user.last_name} ({user.email})
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </div>
            )}

            {/* Name */}
            <div className="space-y-2">
              <Label htmlFor="name">Policy Name</Label>
              <Input
                id="name"
                value={formData.name}
                onChange={(e) => setFormData((prev) => ({ ...prev, name: e.target.value }))}
                placeholder="e.g., Standard Lunch Break"
                disabled={formDrawer.loading}
              />
            </div>

            {/* Description */}
            <div className="space-y-2">
              <Label htmlFor="description">Description (optional)</Label>
              <Textarea
                id="description"
                value={formData.description}
                onChange={(e) => setFormData((prev) => ({ ...prev, description: e.target.value }))}
                placeholder="Optional description of this break policy..."
                disabled={formDrawer.loading}
                rows={3}
              />
            </div>

            {/* Mode Selection */}
            <div className="space-y-2">
              <Label>Tracking Mode</Label>
              <Select
                value={formData.tracking_mode}
                onValueChange={(value: BreakTrackingMode) =>
                  setFormData((prev) => ({ ...prev, tracking_mode: value }))
                }
                disabled={formDrawer.loading}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {TRACKING_MODE_OPTIONS.map((option) => (
                    <SelectItem key={option.value} value={option.value}>
                      <div>
                        <span className="font-medium">{option.label}</span>
                        <span className="text-xs text-muted-foreground ml-2">
                          - {option.description}
                        </span>
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            {/* Options - only show for explicit tracking mode */}
            {formData.tracking_mode === 'explicit_tracking' && (
              <div className="space-y-4 pt-4 border-t">
                <div className="flex items-center justify-between">
                  <div className="space-y-0.5">
                    <Label>Notify Missing Break</Label>
                    <p className="text-xs text-muted-foreground">
                      Alert manager if employee works without logging a break
                    </p>
                  </div>
                  <Switch
                    checked={formData.notify_missing_break}
                    onCheckedChange={(checked) =>
                      setFormData((prev) => ({ ...prev, notify_missing_break: checked }))
                    }
                    disabled={formDrawer.loading}
                  />
                </div>
              </div>
            )}

            <div className="flex justify-end gap-4 pt-4 border-t">
              <Button variant="outline" onClick={handleFormCancel} disabled={formDrawer.loading}>
                Cancel
              </Button>
              <Button onClick={handleFormSubmit} disabled={formDrawer.loading || !formData.name}>
                {formDrawer.loading ? 'Saving...' : isEditing ? 'Save' : 'Create'}
              </Button>
            </div>
          </div>
        </SheetContent>
      </Sheet>

      {/* Window Management Drawer */}
      <Sheet open={windowDrawer.open} onOpenChange={(open) => setWindowDrawer((prev) => ({ ...prev, open }))}>
        <SheetContent className="overflow-y-auto sm:max-w-xl">
          <SheetHeader>
            <SheetTitle>Manage Break Windows</SheetTitle>
            <SheetDescription>
              Configure when breaks must or can be taken for "{windowDrawer.policy?.name}".
              <br />
              <span className="text-xs">
                {windowDrawer.policy?.tracking_mode === 'explicit_tracking'
                  ? 'Employees will see break buttons during these windows.'
                  : 'Break time will be auto-deducted based on these windows.'}
              </span>
            </SheetDescription>
          </SheetHeader>

          <div className="space-y-6 py-4">
            {/* Empty State Guidance */}
            {windowDrawer.policy && windowDrawer.policy.windows.length === 0 && (
              <div className="text-center py-6 border border-dashed rounded-md">
                <Coffee className="h-8 w-8 mx-auto text-muted-foreground mb-2" />
                <p className="text-sm font-medium">No break windows configured</p>
                <p className="text-xs text-muted-foreground mt-1 max-w-[280px] mx-auto">
                  Add at least one break window to define when breaks should be taken. You can select multiple days at once.
                </p>
              </div>
            )}

            {/* Existing Windows */}
            {windowDrawer.policy && windowDrawer.policy.windows.length > 0 && (
              <div className="space-y-2">
                <Label>Current Windows ({windowDrawer.policy.windows.length})</Label>
                <div className="rounded-md border divide-y max-h-[200px] overflow-y-auto">
                  {windowDrawer.policy.windows.map((window: BreakWindowResponse) => (
                    <div key={window.id} className="flex items-center justify-between p-3">
                      <div className="space-y-1">
                        <div className="flex items-center gap-2">
                          <Badge variant="outline">{getDayLabel(window.day_of_week)}</Badge>
                          <span className="text-sm">
                            {formatTime(window.window_start)} - {formatTime(window.window_end)}
                          </span>
                        </div>
                        <div className="flex items-center gap-2 text-xs text-muted-foreground">
                          <span>
                            {formatBreakDuration(window.min_duration_minutes)} - {formatBreakDuration(window.max_duration_minutes)}
                          </span>
                          {window.is_mandatory && (
                            <Badge variant="secondary" className="text-xs">
                              Mandatory
                            </Badge>
                          )}
                        </div>
                      </div>
                      <Button
                        variant="ghost"
                        size="icon"
                        onClick={() => handleDeleteWindow(window.id)}
                      >
                        <Trash2 className="h-4 w-4 text-destructive" />
                      </Button>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {/* Add New Window Form */}
            <div className={`space-y-4 pt-4 ${windowDrawer.policy?.windows && windowDrawer.policy.windows.length > 0 ? 'border-t' : ''}`}>
              <Label className="text-base font-semibold">Add Break Window</Label>

              {/* Multi-Day Selection with Checkboxes */}
              <div className="space-y-2">
                <Label>Days of Week</Label>
                <p className="text-xs text-muted-foreground">
                  Select which days this break window applies to
                </p>
                <div className="grid grid-cols-2 gap-2 mt-2">
                  {DAYS_OF_WEEK.map((day) => (
                    <label key={day.value} className="flex items-center gap-2 cursor-pointer p-2 rounded-md hover:bg-muted/50">
                      <Checkbox
                        checked={windowFormData.selectedDays.includes(day.value)}
                        onCheckedChange={(checked) => {
                          if (checked) {
                            setWindowFormData((prev) => ({
                              ...prev,
                              selectedDays: [...prev.selectedDays, day.value].sort((a, b) => a - b),
                            }));
                          } else {
                            setWindowFormData((prev) => ({
                              ...prev,
                              selectedDays: prev.selectedDays.filter((d) => d !== day.value),
                            }));
                          }
                        }}
                        disabled={windowDrawer.loading}
                      />
                      <span className="text-sm">{day.label}</span>
                    </label>
                  ))}
                </div>
                {/* Quick select buttons */}
                <div className="flex gap-2 mt-2">
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => setWindowFormData((prev) => ({ ...prev, selectedDays: [1, 2, 3, 4, 5] }))}
                    disabled={windowDrawer.loading}
                  >
                    Mon-Fri
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => setWindowFormData((prev) => ({ ...prev, selectedDays: [0, 1, 2, 3, 4, 5, 6] }))}
                    disabled={windowDrawer.loading}
                  >
                    All Days
                  </Button>
                  <Button
                    type="button"
                    variant="outline"
                    size="sm"
                    onClick={() => setWindowFormData((prev) => ({ ...prev, selectedDays: [] }))}
                    disabled={windowDrawer.loading}
                  >
                    Clear
                  </Button>
                </div>
              </div>

              {/* Time Window */}
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="window_start">Window Start</Label>
                  <Input
                    id="window_start"
                    type="time"
                    value={windowFormData.window_start}
                    onChange={(e) =>
                      setWindowFormData((prev) => ({ ...prev, window_start: e.target.value }))
                    }
                    disabled={windowDrawer.loading}
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="window_end">Window End</Label>
                  <Input
                    id="window_end"
                    type="time"
                    value={windowFormData.window_end}
                    onChange={(e) =>
                      setWindowFormData((prev) => ({ ...prev, window_end: e.target.value }))
                    }
                    disabled={windowDrawer.loading}
                  />
                </div>
              </div>

              {/* Duration */}
              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="min_duration">Min Duration (minutes)</Label>
                  <Input
                    id="min_duration"
                    type="number"
                    min={1}
                    max={480}
                    value={windowFormData.min_duration_minutes}
                    onChange={(e) =>
                      setWindowFormData((prev) => ({
                        ...prev,
                        min_duration_minutes: Number(e.target.value),
                      }))
                    }
                    disabled={windowDrawer.loading}
                  />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="max_duration">Max Duration (minutes)</Label>
                  <Input
                    id="max_duration"
                    type="number"
                    min={1}
                    max={480}
                    value={windowFormData.max_duration_minutes}
                    onChange={(e) =>
                      setWindowFormData((prev) => ({
                        ...prev,
                        max_duration_minutes: Number(e.target.value),
                      }))
                    }
                    disabled={windowDrawer.loading}
                  />
                </div>
              </div>

              {/* Mandatory with clear explanation */}
              <div className="flex items-center justify-between p-3 bg-muted/30 rounded-md">
                <div className="space-y-0.5">
                  <Label>Mandatory Break</Label>
                  <p className="text-xs text-muted-foreground max-w-[280px]">
                    {windowDrawer.policy?.tracking_mode === 'auto_deduct'
                      ? 'Break time will be automatically deducted during this window'
                      : 'Employee must take a break during this window'}
                  </p>
                </div>
                <Switch
                  checked={windowFormData.is_mandatory}
                  onCheckedChange={(checked) =>
                    setWindowFormData((prev) => ({ ...prev, is_mandatory: checked }))
                  }
                  disabled={windowDrawer.loading}
                />
              </div>

              <Button
                onClick={handleAddWindow}
                disabled={windowDrawer.loading || windowFormData.selectedDays.length === 0}
                className="w-full"
              >
                <Plus className="h-4 w-4 mr-2" />
                {windowDrawer.loading
                  ? 'Adding...'
                  : windowFormData.selectedDays.length === 0
                    ? 'Select days first'
                    : `Add Window for ${windowFormData.selectedDays.length} day${windowFormData.selectedDays.length !== 1 ? 's' : ''}`}
              </Button>
            </div>
          </div>
        </SheetContent>
      </Sheet>

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
