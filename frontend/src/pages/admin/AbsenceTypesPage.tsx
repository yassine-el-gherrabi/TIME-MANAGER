/**
 * Absence Types Management Page (Admin)
 *
 * Admin page to manage absence types with CRUD operations.
 */

import { useState, useCallback, useEffect } from 'react';
import { toast } from 'sonner';
import { Plus, Loader2, Pencil, Trash2 } from 'lucide-react';
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
import { absenceTypesApi } from '../../api/absenceTypes';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { AbsenceType, CreateAbsenceTypeRequest, UpdateAbsenceTypeRequest } from '../../types/absence';

interface FormData {
  name: string;
  code: string;
  color: string;
  requires_approval: boolean;
  affects_balance: boolean;
  is_paid: boolean;
}

const initialFormData: FormData = {
  name: '',
  code: '',
  color: '#3B82F6',
  requires_approval: true,
  affects_balance: true,
  is_paid: true,
};

export function AbsenceTypesPage() {
  const [types, setTypes] = useState<AbsenceType[]>([]);
  const [loading, setLoading] = useState(true);

  // Form drawer state
  const [formDrawer, setFormDrawer] = useState<{
    open: boolean;
    type: AbsenceType | null;
    loading: boolean;
    error: string;
  }>({ open: false, type: null, loading: false, error: '' });

  const [formData, setFormData] = useState<FormData>(initialFormData);

  // Delete dialog state
  const [deleteDialog, setDeleteDialog] = useState<{
    open: boolean;
    type: AbsenceType | null;
    loading: boolean;
  }>({ open: false, type: null, loading: false });

  // Load types
  const loadTypes = useCallback(async () => {
    try {
      const data = await absenceTypesApi.list();
      setTypes(data);
    } catch (err) {
      toast.error(mapErrorToMessage(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadTypes();
  }, [loadTypes]);

  // Handlers
  const handleCreateClick = () => {
    setFormData(initialFormData);
    setFormDrawer({ open: true, type: null, loading: false, error: '' });
  };

  const handleEditClick = (type: AbsenceType) => {
    setFormData({
      name: type.name,
      code: type.code,
      color: type.color,
      requires_approval: type.requires_approval,
      affects_balance: type.affects_balance,
      is_paid: type.is_paid,
    });
    setFormDrawer({ open: true, type, loading: false, error: '' });
  };

  const handleFormSubmit = async () => {
    if (!formData.name.trim() || !formData.code.trim()) {
      setFormDrawer((prev) => ({ ...prev, error: 'Name and code are required' }));
      return;
    }

    setFormDrawer((prev) => ({ ...prev, loading: true, error: '' }));
    try {
      if (formDrawer.type) {
        // Update
        await absenceTypesApi.update(formDrawer.type.id, formData as UpdateAbsenceTypeRequest);
        toast.success(`Absence type "${formData.name}" updated`);
      } else {
        // Create
        await absenceTypesApi.create(formData as CreateAbsenceTypeRequest);
        toast.success(`Absence type "${formData.name}" created`);
      }
      setFormDrawer({ open: false, type: null, loading: false, error: '' });
      loadTypes();
    } catch (err) {
      setFormDrawer((prev) => ({ ...prev, loading: false, error: mapErrorToMessage(err) }));
    }
  };

  const handleFormCancel = () => {
    setFormDrawer({ open: false, type: null, loading: false, error: '' });
  };

  const handleDeleteClick = (type: AbsenceType) => {
    setDeleteDialog({ open: true, type, loading: false });
  };

  const handleDeleteConfirm = async () => {
    if (!deleteDialog.type) return;

    setDeleteDialog((prev) => ({ ...prev, loading: true }));
    try {
      await absenceTypesApi.delete(deleteDialog.type.id);
      toast.success(`Absence type "${deleteDialog.type.name}" deleted`);
      setDeleteDialog({ open: false, type: null, loading: false });
      loadTypes();
    } catch (err) {
      toast.error(mapErrorToMessage(err));
      setDeleteDialog((prev) => ({ ...prev, loading: false }));
    }
  };

  const isEditing = !!formDrawer.type;

  return (
    <div className="container mx-auto py-8 px-4">
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <div>
            <CardTitle>Absence Types</CardTitle>
            <CardDescription>
              Configure the types of absences employees can request
            </CardDescription>
          </div>
          <Button onClick={handleCreateClick}>
            <Plus className="h-4 w-4 mr-2" />
            Add Type
          </Button>
        </CardHeader>
        <CardContent>
          {loading ? (
            <div className="flex items-center justify-center py-8">
              <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
            </div>
          ) : types.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-sm text-muted-foreground mb-4">
                No absence types configured yet
              </p>
              <Button onClick={handleCreateClick}>
                <Plus className="h-4 w-4 mr-2" />
                Add Absence Type
              </Button>
            </div>
          ) : (
            <div className="rounded-md border">
              <table className="w-full">
                <thead>
                  <tr className="border-b bg-muted/50">
                    <th className="px-4 py-3 text-left text-sm font-medium">Color</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Name</th>
                    <th className="px-4 py-3 text-left text-sm font-medium">Code</th>
                    <th className="px-4 py-3 text-center text-sm font-medium">Approval</th>
                    <th className="px-4 py-3 text-center text-sm font-medium">Balance</th>
                    <th className="px-4 py-3 text-center text-sm font-medium">Paid</th>
                    <th className="px-4 py-3 text-right text-sm font-medium">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {types.map((type) => (
                    <tr key={type.id} className="border-b">
                      <td className="px-4 py-3">
                        <div
                          className="h-6 w-6 rounded-full"
                          style={{ backgroundColor: type.color }}
                        />
                      </td>
                      <td className="px-4 py-3 font-medium">{type.name}</td>
                      <td className="px-4 py-3 text-muted-foreground">{type.code}</td>
                      <td className="px-4 py-3 text-center">
                        {type.requires_approval ? '✓' : '—'}
                      </td>
                      <td className="px-4 py-3 text-center">
                        {type.affects_balance ? '✓' : '—'}
                      </td>
                      <td className="px-4 py-3 text-center">
                        {type.is_paid ? '✓' : '—'}
                      </td>
                      <td className="px-4 py-3 text-right">
                        <div className="flex justify-end gap-2">
                          <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => handleEditClick(type)}
                          >
                            <Pencil className="h-4 w-4" />
                          </Button>
                          <Button
                            variant="ghost"
                            size="icon"
                            onClick={() => handleDeleteClick(type)}
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
          )}
        </CardContent>
      </Card>

      {/* Form Drawer */}
      <Sheet open={formDrawer.open} onOpenChange={(open) => !open && handleFormCancel()}>
        <SheetContent className="overflow-y-auto">
          <SheetHeader>
            <SheetTitle>{isEditing ? 'Edit Absence Type' : 'Add Absence Type'}</SheetTitle>
            <SheetDescription>
              {isEditing
                ? 'Update the absence type configuration'
                : 'Create a new type of absence for employees to request'}
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
                placeholder="e.g., Paid Leave"
                disabled={formDrawer.loading}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="code">Code</Label>
              <Input
                id="code"
                value={formData.code}
                onChange={(e) => setFormData((prev) => ({ ...prev, code: e.target.value.toUpperCase() }))}
                placeholder="e.g., PL"
                disabled={formDrawer.loading}
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="color">Color</Label>
              <div className="flex items-center gap-3">
                <input
                  id="color"
                  type="color"
                  value={formData.color}
                  onChange={(e) => setFormData((prev) => ({ ...prev, color: e.target.value }))}
                  className="h-9 w-14 rounded border cursor-pointer"
                  disabled={formDrawer.loading}
                />
                <Input
                  value={formData.color}
                  onChange={(e) => setFormData((prev) => ({ ...prev, color: e.target.value }))}
                  placeholder="#3B82F6"
                  className="flex-1"
                  disabled={formDrawer.loading}
                />
              </div>
            </div>

            <div className="space-y-4 pt-4">
              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Requires Approval</Label>
                  <p className="text-xs text-muted-foreground">
                    Manager must approve these requests
                  </p>
                </div>
                <Switch
                  checked={formData.requires_approval}
                  onCheckedChange={(checked) =>
                    setFormData((prev) => ({ ...prev, requires_approval: checked }))
                  }
                  disabled={formDrawer.loading}
                />
              </div>

              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Affects Balance</Label>
                  <p className="text-xs text-muted-foreground">
                    Deducts from employee's leave balance
                  </p>
                </div>
                <Switch
                  checked={formData.affects_balance}
                  onCheckedChange={(checked) =>
                    setFormData((prev) => ({ ...prev, affects_balance: checked }))
                  }
                  disabled={formDrawer.loading}
                />
              </div>

              <div className="flex items-center justify-between">
                <div className="space-y-0.5">
                  <Label>Paid Leave</Label>
                  <p className="text-xs text-muted-foreground">
                    Employee is paid during this absence
                  </p>
                </div>
                <Switch
                  checked={formData.is_paid}
                  onCheckedChange={(checked) =>
                    setFormData((prev) => ({ ...prev, is_paid: checked }))
                  }
                  disabled={formDrawer.loading}
                />
              </div>
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
        title="Delete Absence Type"
        description={
          deleteDialog.type
            ? `Are you sure you want to delete "${deleteDialog.type.name}"? This may affect existing absence requests.`
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
