/**
 * Holidays Management Page (Admin)
 *
 * Admin page to manage organization holidays.
 */

import { useState, useCallback, useEffect } from 'react';
import { toast } from 'sonner';
import { format } from 'date-fns';
import { Plus, Loader2, Pencil, Trash2, Sparkles, Calendar, Repeat } from 'lucide-react';
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
import { holidaysApi } from '../../api/holidays';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { Holiday, CreateHolidayRequest, UpdateHolidayRequest } from '../../types/absence';

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

export function HolidaysPage() {
  const [holidays, setHolidays] = useState<Holiday[]>([]);
  const [loading, setLoading] = useState(true);
  const [seedLoading, setSeedLoading] = useState(false);

  // Filter state
  const [filterYear, setFilterYear] = useState<string>(new Date().getFullYear().toString());
  const [showRecurring, setShowRecurring] = useState(true);

  // Form drawer state
  const [formDrawer, setFormDrawer] = useState<{
    open: boolean;
    holiday: Holiday | null;
    loading: boolean;
    error: string;
  }>({ open: false, holiday: null, loading: false, error: '' });

  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    holiday: Holiday | null;
    loading: boolean;
  }>({ open: false, holiday: null, loading: false });

  // Load holidays
  const loadHolidays = useCallback(async () => {
    setLoading(true);
    try {
      const startDate = `${filterYear}-01-01`;
      const endDate = `${filterYear}-12-31`;
      const data = await holidaysApi.list({
        start_date: startDate,
        end_date: endDate,
        is_recurring: showRecurring ? undefined : false,
      });
      setHolidays(data);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, [filterYear, showRecurring]);

  useEffect(() => {
    loadHolidays();
  }, [loadHolidays]);

  // Handlers
  const handleCreateClick = () => {
    setFormData(initialFormData);
    setFormDrawer({ open: true, holiday: null, loading: false, error: '' });
  };

  const handleEditClick = (holiday: Holiday) => {
    setFormData({
      name: holiday.name,
      date: holiday.date,
      is_recurring: holiday.is_recurring,
    });
    setFormDrawer({ open: true, holiday, loading: false, error: '' });
  };

  const handleFormSubmit = async () => {
    if (!formData.name.trim() || !formData.date) {
      setFormDrawer((prev) => ({ ...prev, error: 'Name and date are required' }));
      return;
    }

    setFormDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      if (formDrawer.holiday) {
        // Update
        await holidaysApi.update(formDrawer.holiday.id, formData as UpdateHolidayRequest);
        toast.success(`Holiday "${formData.name}" updated`);
      } else {
        // Create
        await holidaysApi.create(formData as CreateHolidayRequest);
        toast.success(`Holiday "${formData.name}" created`);
      }
      setFormDrawer({ open: false, holiday: null, loading: false, error: '' });
      loadHolidays();
    } catch (err) {
      setFormDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleFormCancel = () => {
    setFormDrawer({ open: false, holiday: null, loading: false, error: '' });
  };

  const handleDeleteClick = (holiday: Holiday) => {
    setDeleteDialog({ open: true, holiday, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.holiday) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await holidaysApi.delete(deleteDialog.holiday.id);
      toast.success(`Holiday "${deleteDialog.holiday.name}" deleted`);
      setDeleteDialog({ open: false, holiday: null, loading: false });
      loadHolidays();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const handleSeed = async () => {
    setSeedLoading(true);
    try {
      await holidaysApi.seed();
      toast.success('Default French holidays created');
      loadHolidays();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setSeedLoading(false);
    }
  };

  const isEditing = !!formDrawer.holiday;

  // Group holidays by month
  const holidaysByMonth = holidays.reduce<Record<string, Holiday[]>>((acc, holiday) => {
    const month = format(new Date(holiday.date), 'MMMM');
    if (!acc[month]) acc[month] = [];
    acc[month].push(holiday);
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
              Holidays
            </CardTitle>
            <CardDescription>
              Configure organization holidays (excluded from working days)
            </CardDescription>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" onClick={handleSeed} disabled={seedLoading}>
              {seedLoading ? (
                <Loader2 className="h-4 w-4 mr-2 animate-spin" />
              ) : (
                <Sparkles className="h-4 w-4 mr-2" />
              )}
              Seed French Holidays
            </Button>
            <Button onClick={handleCreateClick}>
              <Plus className="h-4 w-4 mr-2" />
              Add Holiday
            </Button>
          </div>
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
          ) : holidays.length === 0 ? (
            <div className="text-center py-8">
              <Calendar className="h-12 w-12 text-muted-foreground mx-auto mb-3" />
              <p className="text-sm text-muted-foreground mb-4">
                No holidays configured for {filterYear}
              </p>
              <Button onClick={handleSeed} disabled={seedLoading}>
                <Sparkles className="h-4 w-4 mr-2" />
                Create French Holidays
              </Button>
            </div>
          ) : (
            <div className="space-y-6">
              {Object.entries(holidaysByMonth).map(([month, monthHolidays]) => (
                <div key={month}>
                  <h3 className="text-sm font-medium text-muted-foreground mb-3">
                    {month}
                  </h3>
                  <div className="rounded-md border">
                    <table className="w-full">
                      <tbody>
                        {monthHolidays.map((holiday) => (
                          <tr key={holiday.id} className="border-b last:border-b-0">
                            <td className="px-4 py-3 w-[140px]">
                              <span className="text-sm text-muted-foreground">
                                {format(new Date(holiday.date), 'EEE, MMM d')}
                              </span>
                            </td>
                            <td className="px-4 py-3">
                              <div className="flex items-center gap-2">
                                <span className="font-medium">{holiday.name}</span>
                                {holiday.is_recurring && (
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
                                  onClick={() => handleEditClick(holiday)}
                                >
                                  <Pencil className="h-4 w-4" />
                                </Button>
                                <Button
                                  variant="ghost"
                                  size="icon"
                                  onClick={() => handleDeleteClick(holiday)}
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
            <SheetTitle>{isEditing ? 'Edit Holiday' : 'Add Holiday'}</SheetTitle>
            <SheetDescription>
              {isEditing
                ? 'Update the holiday details'
                : 'Add a new holiday to the organization calendar'}
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
                placeholder="e.g., Jour de l'An"
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
                  This holiday occurs every year on the same date
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
        title="Delete Holiday"
        description={
          deleteDialog.holiday
            ? `Are you sure you want to delete "${deleteDialog.holiday.name}"?`
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
