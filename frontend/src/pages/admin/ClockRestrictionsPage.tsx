/**
 * Clock Restrictions Management Page (Admin)
 *
 * Admin page to manage clock-in/out restrictions at organization, team, or user level.
 */

import { useState, useCallback, useEffect } from 'react';
import { toast } from 'sonner';
import { Plus, Loader2, Pencil, Trash2, Clock, Users, Building2, User } from 'lucide-react';
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../components/ui/select';
import { clockRestrictionsApi } from '../../api/clockRestrictions';
import { teamsApi } from '../../api/teams';
import { usersApi } from '../../api/users';
import { organizationsApi } from '../../api/organizations';
import { OrgTeamFilter, useOrgTeamFilter } from '../../components/filters';
import { useCurrentUser } from '../../hooks/useAuth';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type {
  ClockRestrictionResponse,
  CreateClockRestrictionRequest,
  UpdateClockRestrictionRequest,
  ClockRestrictionMode,
} from '../../types/clockRestriction';
import { RESTRICTION_MODE_CONFIG } from '../../types/clockRestriction';
import type { TeamResponse } from '../../types/team';
import type { UserResponse } from '../../types/user';
import type { OrganizationResponse } from '../../types/organization';
import { UserRole } from '../../types/auth';

interface FormData {
  organization_id: string;
  scope: 'organization' | 'team' | 'user';
  team_id: string;
  user_id: string;
  mode: ClockRestrictionMode;
  clock_in_earliest: string;
  clock_in_latest: string;
  clock_out_earliest: string;
  clock_out_latest: string;
  enforce_schedule: boolean;
  require_manager_approval: boolean;
  max_daily_clock_events: string;
}

const initialFormData: FormData = {
  organization_id: '',
  scope: 'organization',
  team_id: '',
  user_id: '',
  mode: 'flexible',
  clock_in_earliest: '07:00',
  clock_in_latest: '10:00',
  clock_out_earliest: '16:00',
  clock_out_latest: '22:00',
  enforce_schedule: true,
  require_manager_approval: false,
  max_daily_clock_events: '',
};

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
  const [formDrawer, setFormDrawer] = useState<{
    open: boolean;
    restriction: ClockRestrictionResponse | null;
    loading: boolean;
    error: string;
  }>({ open: false, restriction: null, loading: false, error: '' });

  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    restriction: ClockRestrictionResponse | null;
    loading: boolean;
  }>({ open: false, restriction: null, loading: false });

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

  const isEditing = !!formDrawer.restriction;

  // Get scope label and icon for a restriction
  const getScopeInfo = (restriction: ClockRestrictionResponse) => {
    if (restriction.user_id) {
      return {
        label: restriction.user_name || 'User',
        icon: User,
        scope: 'User',
      };
    }
    if (restriction.team_id) {
      return {
        label: restriction.team_name || 'Team',
        icon: Users,
        scope: 'Team',
      };
    }
    return {
      label: restriction.organization_name || 'Organization',
      icon: Building2,
      scope: 'Organization',
    };
  };

  // Format time for display
  const formatTime = (time: string | null) => {
    if (!time) return '-';
    return time.substring(0, 5); // "HH:MM:SS" -> "HH:MM"
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
            <div className="rounded-md border">
              <table className="w-full">
                <thead>
                  <tr className="border-b bg-muted/50">
                    <th className="px-4 py-3 text-left text-sm font-medium">Scope</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Mode</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Clock In</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Clock Out</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Options</th>
                    <th className="px-4 py-3 text-right text-sm font-medium">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {restrictions.map((restriction) => {
                    const scopeInfo = getScopeInfo(restriction);
                    const ScopeIcon = scopeInfo.icon;
                    const modeConfig = RESTRICTION_MODE_CONFIG[restriction.mode];

                    return (
                      <tr key={restriction.id} className="border-b last:border-b-0">
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
                          <Badge
                            variant={
                              restriction.mode === 'strict'
                                ? 'destructive'
                                : restriction.mode === 'flexible'
                                  ? 'secondary'
                                  : 'outline'
                            }
                          >
                            {modeConfig.label}
                          </Badge>
                        </td>
                        <td className="px-4 py-3 text-sm">
                          {restriction.mode !== 'unrestricted' ? (
                            <span>
                              {formatTime(restriction.clock_in_earliest)} -{' '}
                              {formatTime(restriction.clock_in_latest)}
                            </span>
                          ) : (
                            <span className="text-muted-foreground">No restriction</span>
                          )}
                        </td>
                        <td className="px-4 py-3 text-sm">
                          {restriction.mode !== 'unrestricted' ? (
                            <span>
                              {formatTime(restriction.clock_out_earliest)} -{' '}
                              {formatTime(restriction.clock_out_latest)}
                            </span>
                          ) : (
                            <span className="text-muted-foreground">No restriction</span>
                          )}
                        </td>
                        <td className="px-4 py-3">
                          <div className="flex gap-2">
                            {restriction.enforce_schedule && (
                              <Badge variant="outline" className="text-xs">
                                Schedule
                              </Badge>
                            )}
                            {restriction.require_manager_approval && (
                              <Badge variant="outline" className="text-xs">
                                Approval
                              </Badge>
                            )}
                          </div>
                        </td>
                        <td className="px-4 py-3 text-right">
                          <div className="flex justify-end gap-2">
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => handleEditClick(restriction)}
                            >
                              <Pencil className="h-4 w-4" />
                            </Button>
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => handleDeleteClick(restriction)}
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
            <SheetTitle>{isEditing ? 'Edit Clock Restriction' : 'Add Clock Restriction'}</SheetTitle>
            <SheetDescription>
              {isEditing
                ? 'Update the clock restriction settings'
                : 'Configure when users can clock in/out'}
            </SheetDescription>
          </SheetHeader>

          <div className="space-y-4 py-4">
            {formDrawer.error && (
              <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
                {formDrawer.error}
              </div>
            )}

            {/* Organization Selection (SuperAdmin only, for create) */}
            {!isEditing && isSuperAdmin && (
              <div className="space-y-2">
                <Label>Organization</Label>
                <Select
                  value={formData.organization_id}
                  onValueChange={(value) =>
                    setFormData((prev) => ({ ...prev, organization_id: value, team_id: '', user_id: '' }))
                  }
                  disabled={formDrawer.loading}
                >
                  <SelectTrigger>
                    <SelectValue placeholder="Select organization..." />
                  </SelectTrigger>
                  <SelectContent>
                    {organizations.map((org) => (
                      <SelectItem key={org.id} value={org.id}>
                        {org.name}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <p className="text-xs text-muted-foreground">
                  Select which organization this restriction applies to
                </p>
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

            {/* Mode Selection */}
            <div className="space-y-2">
              <Label>Mode</Label>
              <Select
                value={formData.mode}
                onValueChange={(value: ClockRestrictionMode) =>
                  setFormData((prev) => ({ ...prev, mode: value }))
                }
                disabled={formDrawer.loading}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {Object.entries(RESTRICTION_MODE_CONFIG).map(([mode, config]) => (
                    <SelectItem key={mode} value={mode}>
                      <div>
                        <span className="font-medium">{config.label}</span>
                        <span className="text-xs text-muted-foreground ml-2">
                          - {config.description}
                        </span>
                      </div>
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>

            {/* Time Windows (only for non-unrestricted modes) */}
            {formData.mode !== 'unrestricted' && (
              <>
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="clock_in_earliest">Clock In Earliest</Label>
                    <Input
                      id="clock_in_earliest"
                      type="time"
                      value={formData.clock_in_earliest}
                      onChange={(e) =>
                        setFormData((prev) => ({ ...prev, clock_in_earliest: e.target.value }))
                      }
                      disabled={formDrawer.loading}
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="clock_in_latest">Clock In Latest</Label>
                    <Input
                      id="clock_in_latest"
                      type="time"
                      value={formData.clock_in_latest}
                      onChange={(e) =>
                        setFormData((prev) => ({ ...prev, clock_in_latest: e.target.value }))
                      }
                      disabled={formDrawer.loading}
                    />
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <Label htmlFor="clock_out_earliest">Clock Out Earliest</Label>
                    <Input
                      id="clock_out_earliest"
                      type="time"
                      value={formData.clock_out_earliest}
                      onChange={(e) =>
                        setFormData((prev) => ({ ...prev, clock_out_earliest: e.target.value }))
                      }
                      disabled={formDrawer.loading}
                    />
                  </div>
                  <div className="space-y-2">
                    <Label htmlFor="clock_out_latest">Clock Out Latest</Label>
                    <Input
                      id="clock_out_latest"
                      type="time"
                      value={formData.clock_out_latest}
                      onChange={(e) =>
                        setFormData((prev) => ({ ...prev, clock_out_latest: e.target.value }))
                      }
                      disabled={formDrawer.loading}
                    />
                  </div>
                </div>
              </>
            )}

            {/* Options */}
            <div className="space-y-4 pt-4 border-t">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Enforce Schedule</Label>
                  <p className="text-xs text-muted-foreground">
                    Only allow clocking on scheduled working days
                  </p>
                </div>
                <Switch
                  checked={formData.enforce_schedule}
                  onCheckedChange={(checked) =>
                    setFormData((prev) => ({ ...prev, enforce_schedule: checked }))
                  }
                  disabled={formDrawer.loading}
                />
              </div>

              {formData.mode === 'flexible' && (
                <div className="flex items-center justify-between">
                  <div className="space-y-0.5">
                    <Label>Require Manager Approval</Label>
                    <p className="text-xs text-muted-foreground">
                      Override requests need manager approval
                    </p>
                  </div>
                  <Switch
                    checked={formData.require_manager_approval}
                    onCheckedChange={(checked) =>
                      setFormData((prev) => ({ ...prev, require_manager_approval: checked }))
                    }
                    disabled={formDrawer.loading}
                  />
                </div>
              )}

              <div className="space-y-2">
                <Label htmlFor="max_daily_clock_events">Max Daily Clock Events</Label>
                <Input
                  id="max_daily_clock_events"
                  type="number"
                  min="1"
                  placeholder="Unlimited"
                  value={formData.max_daily_clock_events}
                  onChange={(e) =>
                    setFormData((prev) => ({ ...prev, max_daily_clock_events: e.target.value }))
                  }
                  disabled={formDrawer.loading}
                />
                <p className="text-xs text-muted-foreground">
                  Maximum clock in/out entries per day (leave empty for unlimited)
                </p>
              </div>
            </div>

            <div className="flex justify-end gap-4 pt-4 border-t">
              <Button variant="outline" onClick={handleFormCancel} disabled={formDrawer.loading}>
                Cancel
              </Button>
              <Button onClick={handleFormSubmit} disabled={formDrawer.loading}>
                {formDrawer.loading ? 'Saving...' : isEditing ? 'Save' : 'Create'}
              </Button>
            </div>
          </div>
        </SheetContent>
      </Sheet>

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
