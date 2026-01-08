import { useState, useEffect, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { Button } from '../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../components/ui/card';
import { ConfirmDialog } from '../../components/ui/confirm-dialog';
import { UsersTable, UserFilters } from '../../components/admin';
import { usersApi } from '../../api/users';
import { useAuthStore } from '../../stores/authStore';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { UserResponse, PaginatedUsersResponse } from '../../types/user';
import { UserRole } from '../../types/auth';

export function UsersPage() {
  const navigate = useNavigate();
  const currentUser = useAuthStore((state) => state.user);

  const [users, setUsers] = useState<UserResponse[]>([]);
  const [pagination, setPagination] = useState({
    page: 1,
    perPage: 10,
    total: 0,
    totalPages: 0,
  });
  const [filters, setFilters] = useState({
    search: '',
    role: '' as UserRole | '',
  });
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState('');
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    user: UserResponse | null;
    loading: boolean;
  }>({ open: false, user: null, loading: false });

  const fetchUsers = useCallback(async () => {
    setIsLoading(true);
    setError('');
    try {
      const response: PaginatedUsersResponse = await usersApi.list({
        page: pagination.page,
        per_page: pagination.perPage,
        search: filters.search || undefined,
        role: filters.role || undefined,
      });
      setUsers(response.data);
      setPagination((prev) => ({
        ...prev,
        total: response.total,
        totalPages: response.total_pages,
      }));
    } catch (err) {
      setError(mapErrorToMessage(err));
    } finally {
      setIsLoading(false);
    }
  }, [pagination.page, pagination.perPage, filters.search, filters.role]);

  useEffect(() => {
    fetchUsers();
  }, [fetchUsers]);

  const handleSearchChange = (value: string) => {
    setFilters((prev) => ({ ...prev, search: value }));
    setPagination((prev) => ({ ...prev, page: 1 }));
  };

  const handleRoleChange = (value: UserRole | '') => {
    setFilters((prev) => ({ ...prev, role: value }));
    setPagination((prev) => ({ ...prev, page: 1 }));
  };

  const handleEdit = (user: UserResponse) => {
    navigate(`/admin/users/${user.id}/edit`);
  };

  const handleDeleteClick = (user: UserResponse) => {
    setDeleteDialog({ open: true, user, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.user) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await usersApi.delete(deleteDialog.user.id);
      toast.success(`${deleteDialog.user.first_name} ${deleteDialog.user.last_name} has been deleted`);
      setDeleteDialog({ open: false, user: null, loading: false });
      await fetchUsers();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const handleResendInvite = async (user: UserResponse) => {
    try {
      await usersApi.resendInvite(user.id);
      toast.success(`Invitation resent to ${user.email}`);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    }
  };

  const handlePageChange = (newPage: number) => {
    setPagination((prev) => ({ ...prev, page: newPage }));
  };

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Users</CardTitle>
            <CardDescription>Manage users in your organization</CardDescription>
          </div>
          <Button onClick={() => navigate('/admin/users/new')}>Add User</Button>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
              {error}
            </div>
          )}

          <UserFilters
            search={filters.search}
            role={filters.role}
            onSearchChange={handleSearchChange}
            onRoleChange={handleRoleChange}
          />

          <UsersTable
            users={users}
            currentUserId={currentUser?.id}
            onEdit={handleEdit}
            onDelete={handleDeleteClick}
            onResendInvite={handleResendInvite}
            isLoading={isLoading}
          />

          <ConfirmDialog
            open={deleteDialog.open}
            onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
            title="Delete User"
            description={
              deleteDialog.user
                ? `Are you sure you want to delete ${deleteDialog.user.first_name} ${deleteDialog.user.last_name}? This action cannot be undone.`
                : ''
            }
            confirmText="Delete"
            variant="destructive"
            onConfirm={handleDeleteConfirm}
            loading={deleteDialog.loading}
          />

          {pagination.totalPages > 1 && (
            <div className="flex items-center justify-between mt-4 pt-4 border-t">
              <div className="text-sm text-muted-foreground">
                Showing {(pagination.page - 1) * pagination.perPage + 1} to{' '}
                {Math.min(pagination.page * pagination.perPage, pagination.total)} of{' '}
                {pagination.total} users
              </div>
              <div className="flex gap-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handlePageChange(pagination.page - 1)}
                  disabled={pagination.page === 1}
                >
                  Previous
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handlePageChange(pagination.page + 1)}
                  disabled={pagination.page === pagination.totalPages}
                >
                  Next
                </Button>
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
