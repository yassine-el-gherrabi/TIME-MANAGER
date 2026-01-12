/**
 * Organizations Table Component
 *
 * Displays a list of organizations in a table format with actions.
 * Super Admin only.
 */

import type { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { Button } from '../ui/button';
import type { OrganizationResponse } from '../../types/organization';

export interface OrganizationsTableProps {
  organizations: OrganizationResponse[];
  onEdit: (organization: OrganizationResponse) => void;
  onDelete: (organization: OrganizationResponse) => void;
  isLoading?: boolean;
}

/**
 * Format date string for display
 */
const formatDate = (dateString: string): string => {
  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
};

export const OrganizationsTable: FC<OrganizationsTableProps> = ({
  organizations,
  onEdit,
  onDelete,
  isLoading,
}) => {
  const { t } = useTranslation();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">{t('organizations.loadingOrganizations')}</div>
      </div>
    );
  }

  if (organizations.length === 0) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">{t('organizations.noOrganizations')}</div>
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
            <th className="hidden sm:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('organizations.slug')}
            </th>
            <th className="hidden md:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('organizations.timezone')}
            </th>
            <th className="px-4 py-3 text-center text-sm font-medium text-muted-foreground">
              {t('organizations.users')}
            </th>
            <th className="hidden lg:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('common.created')}
            </th>
            <th className="px-4 py-3 text-right text-sm font-medium text-muted-foreground">
              {t('common.actions')}
            </th>
          </tr>
        </thead>
        <tbody>
          {organizations.map((organization) => (
            <tr
              key={organization.id}
              className="border-b hover:bg-muted/25 transition-colors"
            >
              <td className="px-4 py-3 text-sm">
                <div className="font-medium">{organization.name}</div>
                {/* Show slug on mobile under name */}
                <div className="sm:hidden">
                  <code className="px-1 py-0.5 rounded bg-muted text-xs">
                    {organization.slug}
                  </code>
                </div>
              </td>
              <td className="hidden sm:table-cell px-4 py-3 text-sm">
                <code className="px-2 py-0.5 rounded bg-muted text-xs">
                  {organization.slug}
                </code>
              </td>
              <td className="hidden md:table-cell px-4 py-3 text-sm text-muted-foreground">
                {organization.timezone}
              </td>
              <td className="px-4 py-3 text-sm text-center">
                <span
                  className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold border ${
                    (organization.user_count ?? 0) > 0
                      ? 'bg-blue-100 text-blue-800 border-blue-200'
                      : 'bg-gray-100 text-gray-600 border-gray-200'
                  }`}
                >
                  {organization.user_count ?? 0}
                </span>
              </td>
              <td className="hidden lg:table-cell px-4 py-3 text-sm text-muted-foreground">
                {formatDate(organization.created_at)}
              </td>
              <td className="px-4 py-3 text-sm text-right">
                <div className="flex items-center justify-end gap-2">
                  <Button variant="outline" size="sm" onClick={() => onEdit(organization)}>
                    {t('common.edit')}
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    className="text-destructive hover:text-destructive hover:bg-destructive/10 border-destructive/50"
                    onClick={() => onDelete(organization)}
                    disabled={(organization.user_count ?? 0) > 0}
                    title={
                      (organization.user_count ?? 0) > 0
                        ? t('organizations.cannotDeleteWithUsers')
                        : t('organizations.deleteOrganization')
                    }
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
