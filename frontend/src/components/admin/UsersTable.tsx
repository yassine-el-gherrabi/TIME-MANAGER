import React from 'react';
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
}) => {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">Loading users...</div>
      </div>
    );
  }

  if (users.length === 0) {
    return (
      <div className="flex items-center justify-center py-8">
        <div className="text-muted-foreground">No users found</div>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse">
        <thead>
          <tr className="border-b bg-muted/50">
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              Name
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              Email
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              Organization
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              Role
            </th>
            <th className="px-4 py-3 text-left text-sm font-medium text-muted-foreground">
              Status
            </th>
            <th className="px-4 py-3 text-right text-sm font-medium text-muted-foreground">
              Actions
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
                </td>
                <td className="px-4 py-3 text-sm text-muted-foreground">{user.email}</td>
                <td className="px-4 py-3 text-sm text-muted-foreground">{user.organization_name}</td>
                <td className="px-4 py-3 text-sm">
                  <span
                    className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold border ${getRoleBadgeClass(user.role)}`}
                  >
                    {user.role}
                  </span>
                </td>
                <td className="px-4 py-3 text-sm">
                  <div className="flex items-center gap-2">
                    {isDeleted ? (
                      <Badge variant="destructive">Deleted</Badge>
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
                          Restore
                        </Button>
                      )
                    ) : (
                      // Active user actions
                      <>
                        <Button variant="outline" size="sm" onClick={() => onEdit(user)}>
                          Edit
                        </Button>
                        {!user.has_password && (
                          <Button variant="outline" size="sm" onClick={() => onResendInvite(user)}>
                            Resend Invite
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
                            Delete
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
