/**
 * OrgTeamFilter Component
 *
 * Reusable organization and team filter component for admin pages.
 * - Organization filter: SuperAdmin only
 * - Team filter: Admin/Manager+ (optional)
 */

import { useState, useEffect, useCallback } from 'react';
import { Building2, Users, Filter } from 'lucide-react';
import { Button } from '../ui/button';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../ui/select';
import { useCurrentUser } from '../../hooks/useAuth';
import { organizationsApi } from '../../api/organizations';
import { teamsApi } from '../../api/teams';
import type { OrganizationResponse } from '../../types/organization';
import type { TeamResponse } from '../../types/team';
import { UserRole } from '../../types/auth';

export interface OrgTeamFilterProps {
  /** Whether to show the team filter (default: true for applicable pages) */
  showTeamFilter?: boolean;
  /** Currently selected organization ID */
  selectedOrgId: string;
  /** Currently selected team ID */
  selectedTeamId: string;
  /** Callback when organization changes */
  onOrgChange: (orgId: string) => void;
  /** Callback when team changes */
  onTeamChange: (teamId: string) => void;
  /** Optional className for the container */
  className?: string;
}

export function OrgTeamFilter({
  showTeamFilter = true,
  selectedOrgId,
  selectedTeamId,
  onOrgChange,
  onTeamChange,
  className = '',
}: OrgTeamFilterProps) {
  const user = useCurrentUser();

  // Data state
  const [organizations, setOrganizations] = useState<OrganizationResponse[]>([]);
  const [teams, setTeams] = useState<TeamResponse[]>([]);
  const [loadingOrgs, setLoadingOrgs] = useState(false);
  const [loadingTeams, setLoadingTeams] = useState(false);

  // Role checks
  const isSuperAdmin = user?.role === UserRole.SuperAdmin;
  const isAdminOrHigher = user?.role === UserRole.SuperAdmin || user?.role === UserRole.Admin;
  const canFilterByTeam = showTeamFilter && (isAdminOrHigher || user?.role === UserRole.Manager);

  // Load organizations for SuperAdmin
  useEffect(() => {
    if (isSuperAdmin) {
      setLoadingOrgs(true);
      organizationsApi
        .list({ per_page: 100 })
        .then((response) => setOrganizations(response.data))
        .catch(() => setOrganizations([]))
        .finally(() => setLoadingOrgs(false));
    }
  }, [isSuperAdmin]);

  // Load teams for Admin/Manager/SuperAdmin
  useEffect(() => {
    if (canFilterByTeam) {
      setLoadingTeams(true);
      const params: { per_page: number; organization_id?: string } = { per_page: 100 };
      // If SuperAdmin has selected an org, filter teams by that org
      if (isSuperAdmin && selectedOrgId) {
        params.organization_id = selectedOrgId;
      }
      teamsApi
        .list(params)
        .then((response) => setTeams(response.teams))
        .catch(() => setTeams([]))
        .finally(() => setLoadingTeams(false));
    }
  }, [canFilterByTeam, isSuperAdmin, selectedOrgId]);

  // Handle filter changes
  const handleOrgChange = useCallback(
    (value: string) => {
      const newOrgId = value === 'all' ? '' : value;
      onOrgChange(newOrgId);
      // Reset team filter when org changes
      if (newOrgId !== selectedOrgId) {
        onTeamChange('');
      }
    },
    [onOrgChange, onTeamChange, selectedOrgId]
  );

  const handleTeamChange = useCallback(
    (value: string) => {
      onTeamChange(value === 'all' ? '' : value);
    },
    [onTeamChange]
  );

  const hasActiveFilters = selectedOrgId !== '' || selectedTeamId !== '';

  const clearFilters = useCallback(() => {
    onOrgChange('');
    onTeamChange('');
  }, [onOrgChange, onTeamChange]);

  // Don't render if user doesn't have filter permissions
  if (!isSuperAdmin && !canFilterByTeam) {
    return null;
  }

  return (
    <div className={`flex flex-wrap items-center gap-3 ${className}`}>
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
        <Button variant="ghost" size="sm" onClick={clearFilters} className="h-9">
          Clear filters
        </Button>
      )}
    </div>
  );
}

/**
 * Hook to manage org/team filter state
 */
export function useOrgTeamFilter() {
  const [selectedOrgId, setSelectedOrgId] = useState<string>('');
  const [selectedTeamId, setSelectedTeamId] = useState<string>('');

  const resetFilters = useCallback(() => {
    setSelectedOrgId('');
    setSelectedTeamId('');
  }, []);

  return {
    selectedOrgId,
    selectedTeamId,
    setSelectedOrgId,
    setSelectedTeamId,
    resetFilters,
  };
}
