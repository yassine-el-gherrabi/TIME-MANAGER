/**
 * Policy Form Drawer Component
 *
 * Handles create/edit operations for break policies.
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
import { Textarea } from '../../../components/ui/textarea';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../../components/ui/select';
import { TRACKING_MODE_OPTIONS } from '../../../types/break';
import type { BreakTrackingMode } from '../../../types/break';
import type { TeamResponse } from '../../../types/team';
import type { UserResponse } from '../../../types/user';
import type { FormData, FormDrawerState } from './types';

interface PolicyFormDrawerProps {
  formDrawer: FormDrawerState;
  formData: FormData;
  teams: TeamResponse[];
  users: UserResponse[];
  onFormDataChange: (data: FormData) => void;
  onSubmit: () => void;
  onCancel: () => void;
}

export function PolicyFormDrawer({
  formDrawer,
  formData,
  teams,
  users,
  onFormDataChange,
  onSubmit,
  onCancel,
}: PolicyFormDrawerProps) {
  const isEditing = !!formDrawer.policy;

  return (
    <Sheet open={formDrawer.open} onOpenChange={(open) => !open && onCancel()}>
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

          {/* Name */}
          <div className="space-y-2">
            <Label htmlFor="name">Policy Name</Label>
            <Input
              id="name"
              value={formData.name}
              onChange={(e) => onFormDataChange({ ...formData, name: e.target.value })}
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
              onChange={(e) => onFormDataChange({ ...formData, description: e.target.value })}
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
                onFormDataChange({ ...formData, tracking_mode: value })
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
                    onFormDataChange({ ...formData, notify_missing_break: checked })
                  }
                  disabled={formDrawer.loading}
                />
              </div>
            </div>
          )}

          <div className="flex justify-end gap-4 pt-4 border-t">
            <Button variant="outline" onClick={onCancel} disabled={formDrawer.loading}>
              Cancel
            </Button>
            <Button onClick={onSubmit} disabled={formDrawer.loading || !formData.name}>
              {formDrawer.loading ? 'Saving...' : isEditing ? 'Save' : 'Create'}
            </Button>
          </div>
        </div>
      </SheetContent>
    </Sheet>
  );
}

export default PolicyFormDrawer;
