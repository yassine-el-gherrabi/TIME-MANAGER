/**
 * Restriction Form Drawer Component
 *
 * Handles create/edit operations for clock restrictions.
 */

import { Button } from '../../../components/ui/button';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../../../components/ui/sheet';
import { Input } from '../../../components/ui/input';
import { Label } from '../../../components/ui/label';
import { Switch } from '../../../components/ui/switch';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../../components/ui/select';
import { RESTRICTION_MODE_CONFIG } from '../../../types/clockRestriction';
import type { ClockRestrictionMode } from '../../../types/clockRestriction';
import type { TeamResponse } from '../../../types/team';
import type { UserResponse } from '../../../types/user';
import type { OrganizationResponse } from '../../../types/organization';
import type { FormData, FormDrawerState } from './types';

interface RestrictionFormDrawerProps {
  formDrawer: FormDrawerState;
  formData: FormData;
  organizations: OrganizationResponse[];
  teams: TeamResponse[];
  users: UserResponse[];
  isSuperAdmin: boolean;
  onFormDataChange: (data: FormData) => void;
  onSubmit: () => void;
  onCancel: () => void;
}

export function RestrictionFormDrawer({
  formDrawer,
  formData,
  organizations,
  teams,
  users,
  isSuperAdmin,
  onFormDataChange,
  onSubmit,
  onCancel,
}: RestrictionFormDrawerProps) {
  const isEditing = !!formDrawer.restriction;

  return (
    <Sheet open={formDrawer.open} onOpenChange={(open) => !open && onCancel()}>
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
                  onFormDataChange({ ...formData, organization_id: value, team_id: '', user_id: '' })
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
                  onFormDataChange({ ...formData, scope: value, team_id: '', user_id: '' })
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
                onValueChange={(value) => onFormDataChange({ ...formData, team_id: value })}
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
                onValueChange={(value) => onFormDataChange({ ...formData, user_id: value })}
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
                onFormDataChange({ ...formData, mode: value })
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
                      onFormDataChange({ ...formData, clock_in_earliest: e.target.value })
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
                      onFormDataChange({ ...formData, clock_in_latest: e.target.value })
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
                      onFormDataChange({ ...formData, clock_out_earliest: e.target.value })
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
                      onFormDataChange({ ...formData, clock_out_latest: e.target.value })
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
                  onFormDataChange({ ...formData, enforce_schedule: checked })
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
                    onFormDataChange({ ...formData, require_manager_approval: checked })
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
                  onFormDataChange({ ...formData, max_daily_clock_events: e.target.value })
                }
                disabled={formDrawer.loading}
              />
              <p className="text-xs text-muted-foreground">
                Maximum clock in/out entries per day (leave empty for unlimited)
              </p>
            </div>
          </div>

          <div className="flex justify-end gap-4 pt-4 border-t">
            <Button variant="outline" onClick={onCancel} disabled={formDrawer.loading}>
              Cancel
            </Button>
            <Button onClick={onSubmit} disabled={formDrawer.loading}>
              {formDrawer.loading ? 'Saving...' : isEditing ? 'Save' : 'Create'}
            </Button>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}

export default RestrictionFormDrawer;
