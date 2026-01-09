/**
 * Organizations Page
 *
 * Super Admin page to manage organizations with infinite scroll.
 * Supports search filtering and create/edit via side sheets.
 */

import { useState, useCallback, useMemo } from 'react';
import { toast } from 'sonner';
import { Loader2, Search } from 'lucide-react';
import { Button } from '../../components/ui/button';
import { Input } from '../../components/ui/input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../components/ui/card';
import { ConfirmDialog } from '../../components/ui/confirm-dialog';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../../components/ui/sheet';
import { OrganizationsTable, OrganizationForm } from '../../components/admin';
import { organizationsApi } from '../../api/organizations';
import { useInfiniteScroll } from '../../hooks/useInfiniteScroll';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type {
  OrganizationResponse,
  CreateOrganizationRequest,
} from '../../types/organization';

export function OrganizationsPage() {
  // Filter state
  const [search, setSearch] = useState('');

  // Build fetch params from filters
  const fetchParams = useMemo(() => {
    const params: Record<string, string | undefined> = {};
    if (search) params.search = search;
    return params;
  }, [search]);

  // Fetch function for infinite scroll
  const fetchOrganizations = useCallback(
    async (params: { page: number; per_page: number }) => {
      const response = await organizationsApi.list({
        ...fetchParams,
        page: params.page,
        per_page: params.per_page,
      });
      return response;
    },
    [fetchParams]
  );

  // Use infinite scroll hook
  const {
    items: organizations,
    isLoading,
    isInitialLoading,
    hasMore,
    total,
    error,
    sentinelRef,
    reset,
  } = useInfiniteScroll<OrganizationResponse>({
    fetchFn: fetchOrganizations,
    params: fetchParams,
    perPage: 20,
  });

  // Track removed organization IDs for optimistic updates
  const [removedIds, setRemovedIds] = useState<Set<string>>(new Set());
  const displayedOrganizations = organizations.filter((o) => !removedIds.has(o.id));
  const displayTotal = Math.max(0, total - removedIds.size);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    organization: OrganizationResponse | null;
    loading: boolean;
  }>({ open: false, organization: null, loading: false });

  // Create drawer state
  const [createDrawer, setCreateDrawer] = useState<{
    open: boolean;
    loading: boolean;
    error: string;
  }>({ open: false, loading: false, error: '' });

  // Edit drawer state
  const [editDrawer, setEditDrawer] = useState<{
    open: boolean;
    organization: OrganizationResponse | null;
    loading: boolean;
    error: string;
  }>({ open: false, organization: null, loading: false, error: '' });

  const hasActiveFilters = search !== '';

  const handleSearchChange = (value: string) => {
    setSearch(value);
    setRemovedIds(new Set());
  };

  // Create handlers
  const handleCreateClick = () => {
    setCreateDrawer({ open: true, loading: false, error: '' });
  };

  const handleCreateSubmit = async (data: CreateOrganizationRequest) => {
    setCreateDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await organizationsApi.create(data);
      toast.success(`Organization "${data.name}" has been created`);
      setCreateDrawer({ open: false, loading: false, error: '' });
      setRemovedIds(new Set());
      reset();
    } catch (err) {
      setCreateDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleCreateCancel = () => {
    setCreateDrawer({ open: false, loading: false, error: '' });
  };

  // Edit handlers
  const handleEdit = (organization: OrganizationResponse) => {
    setEditDrawer({ open: true, organization, loading: false, error: '' });
  };

  const handleEditSubmit = async (data: CreateOrganizationRequest) => {
    if (!editDrawer.organization) return;

    setEditDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await organizationsApi.update(editDrawer.organization.id, {
        name: data.name,
        timezone: data.timezone,
      });
      toast.success(`Organization "${data.name}" has been updated`);
      setEditDrawer({ open: false, organization: null, loading: false, error: '' });
      setRemovedIds(new Set());
      reset();
    } catch (err) {
      setEditDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleEditCancel = () => {
    setEditDrawer({ open: false, organization: null, loading: false, error: '' });
  };

  // Delete handlers
  const handleDeleteClick = (organization: OrganizationResponse) => {
    setDeleteDialog({ open: true, organization, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.organization) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await organizationsApi.delete(deleteDialog.organization.id);
      toast.success(`Organization "${deleteDialog.organization.name}" has been deleted`);
      setRemovedIds((prev) => new Set(prev).add(deleteDialog.organization!.id));
      setDeleteDialog({ open: false, organization: null, loading: false });
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle className="flex items-center justify-between">
              <span>Organizations</span>
              {displayTotal > 0 && (
                <span className="text-sm font-normal text-muted-foreground ml-4">
                  {displayedOrganizations.length} of {displayTotal}{' '}
                  {hasActiveFilters && '(filtered)'}
                </span>
              )}
            </CardTitle>
            <CardDescription>Manage multi-tenant organizations</CardDescription>
          </div>
          <Button onClick={handleCreateClick}>Add Organization</Button>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
              {error.message}
              <Button variant="outline" size="sm" className="ml-2" onClick={reset}>
                Try again
              </Button>
            </div>
          )}

          {/* Search Filter */}
          <div className="mb-4">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder="Search organizations..."
                value={search}
                onChange={(e) => handleSearchChange(e.target.value)}
                className="pl-9"
              />
            </div>
          </div>

          <OrganizationsTable
            organizations={displayedOrganizations}
            onEdit={handleEdit}
            onDelete={handleDeleteClick}
            isLoading={isInitialLoading}
          />

          {/* Infinite scroll elements */}
          {!isInitialLoading && displayedOrganizations.length > 0 && (
            <>
              {/* Sentinel element for intersection observer */}
              <div ref={sentinelRef} className="h-4" />

              {/* Loading more indicator */}
              {isLoading && (
                <div className="flex items-center justify-center py-4">
                  <Loader2 className="h-5 w-5 animate-spin text-muted-foreground" />
                </div>
              )}

              {/* End of list indicator */}
              {!hasMore && (
                <p className="text-center text-sm text-muted-foreground py-4">
                  All organizations loaded
                </p>
              )}
            </>
          )}

          <ConfirmDialog
            open={deleteDialog.open}
            onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
            title="Delete Organization"
            description={
              deleteDialog.organization
                ? `Are you sure you want to delete "${deleteDialog.organization.name}"? This action cannot be undone.`
                : ''
            }
            confirmText="Delete"
            variant="destructive"
            onConfirm={handleDeleteConfirm}
            loading={deleteDialog.loading}
          />

          {/* Create Organization Drawer */}
          <Sheet open={createDrawer.open} onOpenChange={(open) => !open && handleCreateCancel()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>Add Organization</SheetTitle>
                <SheetDescription>Create a new organization for multi-tenant management.</SheetDescription>
              </SheetHeader>
              <OrganizationForm
                onSubmit={handleCreateSubmit}
                onCancel={handleCreateCancel}
                isLoading={createDrawer.loading}
                error={createDrawer.error}
                variant="sheet"
              />
            </SheetContent>
          </Sheet>

          {/* Edit Organization Drawer */}
          <Sheet open={editDrawer.open} onOpenChange={(open) => !open && handleEditCancel()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>Edit Organization</SheetTitle>
                <SheetDescription>Update organization information</SheetDescription>
              </SheetHeader>
              <OrganizationForm
                organization={editDrawer.organization}
                onSubmit={handleEditSubmit}
                onCancel={handleEditCancel}
                isLoading={editDrawer.loading}
                error={editDrawer.error}
                variant="sheet"
              />
            </SheetContent>
          </Sheet>
        </CardContent>
      </Card>
    </div>
  );
}
