/**
 * Teams Table Component
 *
 * Displays a list of teams in a table format with actions.
 */

import React from 'react';
import { useTranslation } from 'react-i18next';
import { Button } from '../ui/button';
import type { TeamResponse } from '../../types/team';

export interface TeamsTableProps {
  teams: TeamResponse[];
  onEdit: (team: TeamResponse) => void;
  onManageMembers: (team: TeamResponse) => void;
  onDelete: (team: TeamResponse) => void;
  isLoading?: boolean;
  /** Optional: Map of manager IDs to names for display */
  managerNames?: Record<string, string>;
}

export const TeamsTable: React.FC<TeamsTableProps> = ({
  teams,
  onEdit,
  onManageMembers,
  onDelete,
  isLoading,
  managerNames = {},
}) => {
  const { t } = useTranslation();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">{t('teams.loadingTeams')}</div>
      </div>
    );
  }

  if (teams.length === 0) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">{t('teams.noTeams')}</div>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b bg-muted/50">
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('common.name')}
            </th>
            <th className="hidden md:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('common.description')}
            </th>
            <th className="hidden sm:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('teams.manager')}
            </th>
            <th className="px-4 py-3 text-center text-sm font-medium text-muted-foreground">
              {t('teams.members')}
            </th>
            <th className="px-4 py-3 text-right text-sm font-medium text-muted-foreground">
              {t('common.actions')}
            </th>
          </tr>
        </thead>
        <tbody>
          {teams.map((team) => (
            <tr key={team.id} className="border-b hover:bg-muted/25 transition-colors">
              <td className="px-4 py-3 text-sm">
                <div className="font-medium">{team.name}</div>
                {/* Show manager on mobile under name */}
                <div className="sm:hidden text-xs text-muted-foreground">
                  {team.manager_id ? (
                    managerNames[team.manager_id] || t('common.loading')
                  ) : (
                    <span className="italic">{t('teams.noManager')}</span>
                  )}
                </div>
              </td>
              <td className="hidden md:table-cell px-4 py-3 text-sm text-muted-foreground">
                {team.description || <span className="italic">{t('teams.noDescription')}</span>}
              </td>
              <td className="hidden sm:table-cell px-4 py-3 text-sm text-muted-foreground">
                {team.manager_id ? (
                  managerNames[team.manager_id] || (
                    <span className="text-xs text-muted-foreground/70">{t('common.loading')}</span>
                  )
                ) : (
                  <span className="italic">{t('teams.noManager')}</span>
                )}
              </td>
              <td className="px-4 py-3 text-sm text-center">
                <span
                  className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold border ${
                    team.member_count > 0
                      ? 'bg-blue-100 text-blue-800 border-blue-200'
                      : 'bg-gray-100 text-gray-600 border-gray-200'
                  }`}
                >
                  {team.member_count}
                </span>
              </td>
              <td className="px-4 py-3 text-sm text-right">
                <div className="flex items-center justify-end gap-2">
                  <Button variant="outline" size="sm" onClick={() => onEdit(team)}>
                    {t('common.edit')}
                  </Button>
                  <Button variant="outline" size="sm" onClick={() => onManageMembers(team)}>
                    {t('teams.members')}
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    className="text-destructive hover:text-destructive hover:bg-destructive/10 border-destructive/50"
                    onClick={() => onDelete(team)}
                    disabled={team.member_count > 0}
                    title={team.member_count > 0 ? t('teams.removeMembersFirst') : t('teams.deleteTeam')}
                  >
                    {t('common.delete')}
                  </Button>
                </div>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};
