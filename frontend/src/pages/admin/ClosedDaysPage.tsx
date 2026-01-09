/**
 * Closed Days Management Page (Admin)
 *
 * Admin page to manage organization closed days (company holidays, office closures).
 */

import { useState, useCallback, useEffect } from 'react';
import { toast } from 'sonner';
import { format } from 'date-fns';
import { Plus, Loader2, Pencil, Trash2, Calendar, Repeat } from 'lucide-react';
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
import { Input } from '../../components/ui/input';
import { Label } from '../../components/ui/label';
import { Switch } from '../../components/ui/switch';
import { Badge } from '../../components/ui/badge';
import { closedDaysApi } from '../../api/closedDays';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { ClosedDay, CreateClosedDayRequest, UpdateClosedDayRequest } from '../../types/absence';

interface FormData {
  name: string;
  date: string;
  is_recurring: boolean;
}

const initialFormData: FormData = {
  name: '',
  date: format(new Date(), 'yyyy-MM-dd'),
  is_recurring: false,
};

export function ClosedDaysPage() {
  const [closedDays, setClosedDays] = useState<ClosedDay[]>([]);
  const [loading, setLoading] = useState(true);

  // Filter state
  const [filterYear, setFilterYear] = useState<string>(new Date().getFullYear().toString());
  const [showRecurring, setShowRecurring] = useState(true);

  // Form drawer state
  const [formDrawer, setFormDrawer] = useState<{
    open: boolean;
    closedDay: ClosedDay | null;
    loading: boolean;
    error: string;
  }>({ open: false, closedDay: null, loading: false, error: '' });

  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    closedDay: ClosedDay | null;
    loading: boolean;
  }>({ open: false, closedDay: null, loading: false });

  // Load closed days
  const loadClosedDays = useCallback(async () => {
    setLoading(true);
    try {
      const startDate = `${filterYear}-01-01`;
      const endDate = `${filterYear}-12-31`;
      const data = await closedDaysApi.list({
        start_date: startDate,
        end_date: endDate,
        is_recurring: showRecurring ? undefined : false,
      });
      setClosedDays(data);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, [filterYear, showRecurring]);

  useEffect(() => {
    loadClosedDays();
  }, [loadClosedDays]);

  // Handlers
  const handleCreateClick = () => {
    setFormData(initialFormData);
    setFormDrawer({ open: true, closedDay: null, loading: false, error: '' });
  };

  const handleEditClick = (closedDay: ClosedDay) => {
    setFormData({
      name: closedDay.name,
      date: closedDay.date,
      is_recurring: closedDay.is_recurring,
    });
    setFormDrawer({ open: true, closedDay, loading: false, error: '' });
  };

  const handleFormSubmit = async () => {
    if (!formData.name.trim() || !formData.date) {
      setFormDrawer((prev) => ({ ...prev, error: 'Name and date are required' }));
      return;
    }

    setFormDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      if (formDrawer.closedDay) {
        // Update
        await closedDaysApi.update(formDrawer.closedDay.id, formData as UpdateClosedDayRequest);
        toast.success(`Closed day "${formData.name}" updated`);
      } else {
        // Create
        await closedDaysApi.create(formData as CreateClosedDayRequest);
        toast.success(`Closed day "${formData.name}" created`);
      }
      setFormDrawer({ open: false, closedDay: null, loading: false, error: '' });
      loadClosedDays();
    } catch (err) {
      setFormDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleFormCancel = () => {
    setFormDrawer({ open: false, closedDay: null, loading: false, error: '' });
  };

  const handleDeleteClick = (closedDay: ClosedDay) => {
    setDeleteDialog({ open: true, closedDay, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.closedDay) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await closedDaysApi.delete(deleteDialog.closedDay.id);
      toast.success(`Closed day "${deleteDialog.closedDay.name}" deleted`);
      setDeleteDialog({ open: false, closedDay: null, loading: false });
      loadClosedDays();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const isEditing = !!formDrawer.closedDay;

  // Group closed days by month
  const closedDaysByMonth = closedDays.reduce<Record<string, ClosedDay[]>>((acc, closedDay) => {
    const month = format(new Date(closedDay.date), 'MMMM');
    if (!acc[month]) acc[month] = [];
    acc[month].push(closedDay);
    return acc;
  }, {});

  // Generate year options
  const currentYear = new Date().getFullYear();
  const yearOptions = Array.from({ length: 5 }, (_, i) => currentYear - 2 + i);

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle className="flex items-center gap-2">
              <Calendar className="h-5 w-5" />
              Closed Days
            </CardTitle>
            <CardDescription>
              Configure organization closed days (excluded from working days)
            </CardDescription>
          </div>
          <Button onClick={handleCreateClick}>
            <Plus className="h-4 w-4 mr-2" />
            Add Closed Day
          </Button>
        </CardHeader>
        <CardContent>
          {/* Filters */}
          <div className="flex flex-wrap items-center gap-4 mb-6">
            <div className="space-y-1">
              <label className="text-sm text-muted-foreground">Year</label>
              <select
                value={filterYear}
                onChange={(e) => setFilterYear(e.target.value)}
                className="flex h-9 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              >
                {yearOptions.map((year) => (
                  <option key={year} value={year}>
                    {year}
                  </option>
                ))}
              </select>
            </div>

            <div className="flex items-center gap-2">
              <Switch
                id="show-recurring"
                checked={showRecurring}
                onCheckedChange={setShowRecurring}
              />
              <Label htmlFor="show-recurring" className="text-sm">
                Include recurring
              </Label>
            </div>
          </div>

          {loading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
          ) : closedDays.length === 0 ? (
            <div className="text-center py-8">
              <Calendar className="h-12 w-12 text-muted-foreground mx-auto mb-3" />
              <p className="text-sm text-muted-foreground mb-4">
                No closed days configured for {filterYear}
              </p>
              <Button onClick={handleCreateClick}>
                <Plus className="h-4 w-4 mr-2" />
                Add Closed Day
              </Button>
            </div>
          ) : (
            <div className="space-y-6">
              {Object.entries(closedDaysByMonth).map(([month, monthClosedDays]) => (
                <div key={month}>
                  <h3 className="text-sm font-medium text-muted-foreground mb-3">
                    {month}
                  </h3>
                  <div className="rounded-md border">
                    <table className="w-full">
                      <tbody>
                        {monthClosedDays.map((closedDay) => (
                          <tr key={closedDay.id} className="border-b last:border-b-0">
                            <td className="px-4 py-3 w-[140px]">
                              <span className="text-sm text-muted-foreground">
                                {format(new Date(closedDay.date), 'EEE, MMM d')}
                              </span>
                            </td>
                            <td className="px-4 py-3">
                              <div className="flex items-center gap-2">
                                <span className="font-medium">{closedDay.name}</span>
                                {closedDay.is_recurring && (
                                  <Badge variant="secondary" className="gap-1">
                                    <Repeat className="h-3 w-3" />
                                    Recurring
                                  </Badge>
                                )}
                              </div>
                            </td>
                            <td className="px-4 py-3 text-right">
                              <div className="flex justify-end gap-2">
                                <Button
                                  variant="ghost"
                                  size="icon"
                                  onClick={() => handleEditClick(closedDay)}
                                >
                                  <Pencil className="h-4 w-4" />
                                </Button>
                                <Button
                                  variant="ghost"
                                  size="icon"
                                  onClick={() => handleDeleteClick(closedDay)}
                                >
                                  <Trash2 className="h-4 w-4 text-destructive" />
                                </Button>
                              </div>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                </div>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* Form Drawer */}
      <Sheet open={formDrawer.open} onOpenChange={(open) => !open && handleFormCancel()}>
        <SheetContent className="overflow-y-auto">
          <SheetHeader>
            <SheetTitle>{isEditing ? 'Edit Closed Day' : 'Add Closed Day'}</SheetTitle>
            <SheetDescription>
              {isEditing
                ? 'Update the closed day details'
                : 'Add a new closed day to the organization calendar'}
            </SheetDescription>
          </SheetHeader>

          <div className="space-y-4 py-4">
            {formDrawer.error && (
              <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
                {formDrawer.error}
              </div>
            )}

            <div className="space-y-2">
              <Label htmlFor="name">Name</Label>
              <Input
                id="name"
                value={formData.name}
                onChange={(e) => setFormData((prev) => ({ ...prev, name: e.target.value }))}
                placeholder="e.g., New Year's Day"
                disabled={formDrawer.loading}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="date">Date</Label>
              <Input
                id="date"
                type="date"
                value={formData.date}
                onChange={(e) => setFormData((prev) => ({ ...prev, date: e.target.value }))}
                disabled={formDrawer.loading}
              />
            </div>

            <div className="flex items-center justify-between pt-4">
              <div className="space-y-0.5">
                <Label>Recurring Yearly</Label>
                <p className="text-xs text-muted-foreground">
                  This closed day occurs every year on the same date
                </p>
              </div>
              <Switch
                checked={formData.is_recurring}
                onCheckedChange={(checked) =>
                  setFormData((prev) => ({ ...prev, is_recurring: checked }))
                }
                disabled={formDrawer.loading}
              />
            </div>

            <div className="flex justify-end gap-4 pt-4 border-t">
              <Button variant="outline" onClick={handleFormCancel} disabled={formDrawer.loading}>
                Cancel
              </Button>
              <Button onClick={handleFormSubmit} disabled={formDrawer.loading}>
                {formDrawer.loading ? 'Saving...' : isEditing ? 'Save' : 'Create'}
              </Button>
            </div>
          </div>
        </SheetContent>
      </Sheet>

      {/* Delete Confirmation */}
      <ConfirmDialog
        open={deleteDialog.open}
        onOpenChange={(open) => setDeleteDialog((prev) => ({ ...prev, open }))}
        title="Delete Closed Day"
        description={
          deleteDialog.closedDay
            ? `Are you sure you want to delete "${deleteDialog.closedDay.name}"?`
            : ''
        }
        confirmText="Delete"
        variant="destructive"
        onConfirm={handleDeleteConfirm}
        loading={deleteDialog.loading}
      />
    </div>
  );
}
