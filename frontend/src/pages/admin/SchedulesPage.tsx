/**
 * Schedules Management Page
 *
 * Admin page for managing work schedules.
 * Supports create, edit, delete, and assign operations.
 */

import { useState, useEffect, useCallback } from 'react';
import { toast } from 'sonner';
import { Loader2 } from 'lucide-react';
import { Button } from '../../components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '../../components/ui/card';
import { ConfirmDialog } from '../../components/ui/confirm-dialog';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../../components/ui/sheet';
import { SchedulesTable } from '../../components/admin/SchedulesTable';
import { ScheduleForm } from '../../components/admin/ScheduleForm';
import { ScheduleAssignPanel } from '../../components/admin/ScheduleAssignPanel';
import { schedulesApi } from '../../api/schedules';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { WorkScheduleWithDays, CreateScheduleRequest } from '../../types/schedule';

export function SchedulesPage() {
  // Data state
  const [schedules, setSchedules] = useState<WorkScheduleWithDays[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  // Create drawer state
  const [createDrawer, setCreateDrawer] = useState<{
    open: boolean;
    loading: boolean;
    error: string;
  }>({ open: false, loading: false, error: '' });

  // Edit drawer state
  const [editDrawer, setEditDrawer] = useState<{
    open: boolean;
    schedule: WorkScheduleWithDays | null;
    loading: boolean;
    error: string;
  }>({ open: false, schedule: null, loading: false, error: '' });

  // Assign drawer state
  const [assignDrawer, setAssignDrawer] = useState<{
    open: boolean;
    schedule: WorkScheduleWithDays | null;
  }>({ open: false, schedule: null });

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    schedule: WorkScheduleWithDays | null;
    loading: boolean;
  }>({ open: false, schedule: null, loading: false });

  // Load schedules
  const loadSchedules = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await schedulesApi.list();
      setSchedules(data);
    } catch (err) {
      setError(err as Error);
      toast.error(mapErrorToMessage(err));
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    loadSchedules();
  }, [loadSchedules]);

  // Create handlers
  const handleCreateClick = () => {
    setCreateDrawer({ open: true, loading: false, error: '' });
  };

  const handleCreateSubmit = async (data: CreateScheduleRequest) => {
    setCreateDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      await schedulesApi.create(data);
      toast.success(`Schedule "${data.name}" has been created`);
      setCreateDrawer({ open: false, loading: false, error: '' });
      loadSchedules();
    } catch (err) {
      setCreateDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleCreateCancel = () => {
    setCreateDrawer({ open: false, loading: false, error: '' });
  };

  // Edit handlers
  const handleEdit = (schedule: WorkScheduleWithDays) => {
    setEditDrawer({ open: true, schedule, loading: false, error: '' });
  };

  const handleEditSubmit = async (data: CreateScheduleRequest) => {
    if (!editDrawer.schedule) return;

    setEditDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      // Update schedule metadata
      await schedulesApi.update(editDrawer.schedule.schedule.id, {
        name: data.name,
        description: data.description,
        is_default: data.is_default,
      });

      // Handle day changes: delete existing days and recreate
      // First, remove all existing days
      for (const existingDay of editDrawer.schedule.days) {
        await schedulesApi.removeDay(existingDay.id);
      }

      // Then add new days
      for (const newDay of data.days) {
        await schedulesApi.addDay(editDrawer.schedule.schedule.id, {
          day_of_week: newDay.day_of_week,
          start_time: newDay.start_time,
          end_time: newDay.end_time,
          break_minutes: newDay.break_minutes,
        });
      }

      toast.success(`Schedule "${data.name}" has been updated`);
      setEditDrawer({ open: false, schedule: null, loading: false, error: '' });
      loadSchedules();
    } catch (err) {
      setEditDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleEditCancel = () => {
    setEditDrawer({ open: false, schedule: null, loading: false, error: '' });
  };

  // Assign handlers
  const handleAssign = (schedule: WorkScheduleWithDays) => {
    setAssignDrawer({ open: true, schedule });
  };

  const handleAssignClose = () => {
    setAssignDrawer({ open: false, schedule: null });
  };

  // Delete handlers
  const handleDeleteClick = (schedule: WorkScheduleWithDays) => {
    if (schedule.schedule.is_default) {
      toast.error('Cannot delete the default schedule');
      return;
    }
    setDeleteDialog({ open: true, schedule, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.schedule) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await schedulesApi.delete(deleteDialog.schedule.schedule.id);
      toast.success(`Schedule "${deleteDialog.schedule.schedule.name}" has been deleted`);
      setDeleteDialog({ open: false, schedule: null, loading: false });
      loadSchedules();
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
              <span>Work Schedules</span>
              {schedules.length > 0 && (
                <span className="text-sm font-normal text-muted-foreground ml-4">
                  {schedules.length} schedule{schedules.length !== 1 ? 's' : ''}
                </span>
              )}
            </CardTitle>
            <CardDescription>Manage work schedule templates for your organization</CardDescription>
          </div>
          <Button onClick={handleCreateClick}>Add Schedule</Button>
        </CardHeader>
        <CardContent>
          {error && (
            <div className="mb-4 p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
              {error.message}
              <Button variant="outline" size="sm" className="ml-2" onClick={loadSchedules}>
                Try again
              </Button>
            </div>
          )}

          {isLoading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
            </div>
          ) : (
            <SchedulesTable
              schedules={schedules}
              onEdit={handleEdit}
              onAssign={handleAssign}
              onDelete={handleDeleteClick}
            />
          )}

          {/* Delete confirmation dialog */}
          <ConfirmDialog
            open={deleteDialog.open}
            onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
            title="Delete Schedule"
            description={
              deleteDialog.schedule
                ? `Are you sure you want to delete "${deleteDialog.schedule.schedule.name}"? Users assigned to this schedule will need a new assignment.`
                : ''
            }
            confirmText="Delete"
            variant="destructive"
            onConfirm={handleDeleteConfirm}
            loading={deleteDialog.loading}
          />

          {/* Create Schedule Drawer */}
          <Sheet open={createDrawer.open} onOpenChange={(open) => !open && handleCreateCancel()}>
            <SheetContent className="overflow-y-auto sm:max-w-xl">
              <SheetHeader>
                <SheetTitle>Create Schedule</SheetTitle>
                <SheetDescription>
                  Create a new work schedule template.
                </SheetDescription>
              </SheetHeader>
              <ScheduleForm
                onSubmit={handleCreateSubmit}
                onCancel={handleCreateCancel}
                isLoading={createDrawer.loading}
                error={createDrawer.error}
                variant="sheet"
              />
            </SheetContent>
          </Sheet>

          {/* Edit Schedule Drawer */}
          <Sheet open={editDrawer.open} onOpenChange={(open) => !open && handleEditCancel()}>
            <SheetContent className="overflow-y-auto sm:max-w-xl">
              <SheetHeader>
                <SheetTitle>Edit Schedule</SheetTitle>
                <SheetDescription>
                  Update schedule configuration.
                </SheetDescription>
              </SheetHeader>
              <ScheduleForm
                schedule={editDrawer.schedule}
                onSubmit={handleEditSubmit}
                onCancel={handleEditCancel}
                isLoading={editDrawer.loading}
                error={editDrawer.error}
                variant="sheet"
              />
            </SheetContent>
          </Sheet>

          {/* Assign Schedule Drawer */}
          <Sheet open={assignDrawer.open} onOpenChange={(open) => !open && handleAssignClose()}>
            <SheetContent className="overflow-y-auto">
              <SheetHeader>
                <SheetTitle>Assign Schedule</SheetTitle>
                <SheetDescription>
                  Assign "{assignDrawer.schedule?.schedule.name}" to users.
                </SheetDescription>
              </SheetHeader>
              <ScheduleAssignPanel
                schedule={assignDrawer.schedule}
                onClose={handleAssignClose}
              />
            </SheetContent>
          </Sheet>
        </CardContent>
      </Card>
    </div>
  );
}
