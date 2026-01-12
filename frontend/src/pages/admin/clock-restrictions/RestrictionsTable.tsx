/**
 * Restrictions Table Component
 *
 * Displays clock restrictions in a table format with scope and mode info.
 */

import { Pencil, Trash2, Users, Building2, User } from 'lucide-react';
import { Button } from '../../../components/ui/button';
import { Badge } from '../../../components/ui/badge';
import { RESTRICTION_MODE_CONFIG } from '../../../types/clockRestriction';
import type { ClockRestrictionResponse } from '../../../types/clockRestriction';

interface RestrictionsTableProps {
  restrictions: ClockRestrictionResponse[];
  onEdit: (restriction: ClockRestrictionResponse) => void;
  onDelete: (restriction: ClockRestrictionResponse) => void;
}

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

export function RestrictionsTable({
  restrictions,
  onEdit,
  onDelete,
}: RestrictionsTableProps) {
  if (restrictions.length === 0) {
    return null;
  }

  return (
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
                      onClick={() => onEdit(restriction)}
                    >
                      <Pencil className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="ghost"
                      size="icon"
                      onClick={() => onDelete(restriction)}
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
  );
}

export default RestrictionsTable;
