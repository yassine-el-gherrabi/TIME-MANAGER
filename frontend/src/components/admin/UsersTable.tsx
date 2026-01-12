import React from 'react';
import { useTranslation } from 'react-i18next';
import { Button } from '../ui/button';
import { Badge } from '../ui/badge';
import { InviteStatusBadge } from './InviteStatusBadge';
import type { UserResponse } from '../../types/user';
import { UserRole } from '../../types/auth';

export interface UsersTableProps {
  users: UserResponse[];
  currentUserId?: string;
  currentUserRole?: UserRole;
  onEdit: (user: UserResponse) => void;
  onDelete: (user: UserResponse) => void;
  onResendInvite: (user: UserResponse) => void;
  onRestore?: (user: UserResponse) => void;
  isLoading?: boolean;
  showOrganization?: boolean;
}

// Role hierarchy for comparison (higher value = more privileged)
const roleHierarchy: Record<UserRole, number> = {
  [UserRole.Employee]: 0,
  [UserRole.Manager]: 1,
  [UserRole.Admin]: 2,
  [UserRole.SuperAdmin]: 3,
};

const getRoleBadgeClass = (role: UserRole): string => {
  switch (role) {
    case UserRole.SuperAdmin:
      return 'bg-amber-100 text-amber-800 border-amber-200';
    case UserRole.Admin:
      return 'bg-purple-100 text-purple-800 border-purple-200';
    case UserRole.Manager:
      return 'bg-blue-100 text-blue-800 border-blue-200';
    case UserRole.Employee:
      return 'bg-gray-100 text-gray-800 border-gray-200';
    default:
      return 'bg-gray-100 text-gray-800 border-gray-200';
  }
};

export const UsersTable: React.FC<UsersTableProps> = ({
  users,
  currentUserId,
  currentUserRole,
  onEdit,
  onDelete,
  onResendInvite,
  onRestore,
  isLoading,
  showOrganization = false,
}) => {
  const { t } = useTranslation();

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">{t('users.loadingUsers')}</div>
      </div>
    );
  }

  if (users.length === 0) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">{t('users.noUsers')}</div>
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
              {t('common.email')}
            </th>
            {showOrganization && (
              <th className="hidden md:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
                {t('users.organization')}
              </th>
            )}
            <th className="hidden sm:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('users.role')}
            </th>
            <th className="hidden md:table-cell px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              {t('common.status')}
            </th>
            <th className="px-4 py-3 text-right text-sm font-medium text-muted-foreground">
              {t('common.actions')}
            </th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => {
            const isDeleted = !!user.deleted_at;

            return (
              <tr
                key={user.id}
                className={`border-b transition-colors ${
                  isDeleted
                    ? 'bg-muted/30 opacity-60'
                    : 'hover:bg-muted/25'
                }`}
              >
                <td className="px-4 py-3 text-sm">
                  <div className="font-medium">
                    {user.first_name} {user.last_name}
                  </div>
                  {/* Show email on mobile under name */}
                  <div className="sm:hidden text-xs text-muted-foreground truncate max-w-[150px]">
                    {user.email}
                  </div>
                </td>
                <td className="hidden sm:table-cell px-4 py-3 text-sm text-muted-foreground">{user.email}</td>
                {showOrganization && (
                  <td className="hidden md:table-cell px-4 py-3 text-sm text-muted-foreground">{user.organization_name}</td>
                )}
                <td className="hidden sm:table-cell px-4 py-3 text-sm">
                  <span
                    className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold border ${getRoleBadgeClass(user.role)}`}
                  >
                    {user.role}
                  </span>
                </td>
                <td className="hidden md:table-cell px-4 py-3 text-sm">
                  <div className="flex items-center gap-2">
                    {isDeleted ? (
                      <Badge variant="destructive">{t('users.deleted')}</Badge>
                    ) : (
                      <InviteStatusBadge hasPassword={user.has_password} />
                    )}
                  </div>
                </td>
                <td className="px-4 py-3 text-sm text-right">
                  <div className="flex items-center justify-end gap-2">
                    {isDeleted ? (
                      // Deleted user actions - only restore
                      onRestore && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => onRestore(user)}
                        >
                          {t('common.restore')}
                        </Button>
                      )
                    ) : (
                      // Active user actions
                      <>
                        <Button variant="outline" size="sm" onClick={() => onEdit(user)}>
                          {t('common.edit')}
                        </Button>
                        {!user.has_password && (
                          <Button variant="outline" size="sm" onClick={() => onResendInvite(user)}>
                            {t('users.resendInvite')}
                          </Button>
                        )}
                        {user.id !== currentUserId &&
                          currentUserRole &&
                          roleHierarchy[user.role] < roleHierarchy[currentUserRole] && (
                          <Button
                            variant="outline"
                            size="sm"
                            className="text-destructive hover:text-destructive hover:bg-destructive/10 border-destructive/50"
                            onClick={() => onDelete(user)}
                          >
                            {t('common.delete')}
                          </Button>
                        )}
                      </>
                    )}
                  </div>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};
