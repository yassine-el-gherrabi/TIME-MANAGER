/**
 * Absence Request Form Component
 *
 * Form for submitting new absence requests.
 */

import { useState, useEffect } from 'react';
import type { FC, FormEvent } from 'react';
import { format, addDays, isWeekend } from 'date-fns';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import { AbsenceTypeSelector } from './AbsenceTypeSelector';
import type { AbsenceType, CreateAbsenceRequest, LeaveBalance } from '../../types/absence';

export interface AbsenceRequestFormProps {
  absenceTypes: AbsenceType[];
  balances: LeaveBalance[];
  onSubmit: (data: CreateAbsenceRequest) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string;
  /** Use 'sheet' variant when rendering inside a Sheet/Drawer */
  variant?: 'card' | 'sheet';
}

interface FormData {
  type_id: string;
  start_date: string;
  end_date: string;
  reason: string;
}

interface FormErrors {
  type_id?: string;
  start_date?: string;
  end_date?: string;
}

/**
 * Calculate working days between two dates (excluding weekends)
 */
const calculateWorkingDays = (startDate: string, endDate: string): number => {
  const start = new Date(startDate);
  const end = new Date(endDate);

  let count = 0;
  let current = start;

  while (current <= end) {
    if (!isWeekend(current)) {
      count++;
    }
    current = addDays(current, 1);
  }

  return count;
};

export const AbsenceRequestForm: FC<AbsenceRequestFormProps> = ({
  absenceTypes,
  balances,
  onSubmit,
  onCancel,
  isLoading,
  error,
  variant = 'card',
}) => {
  const today = format(new Date(), 'yyyy-MM-dd');

  const [formData, setFormData] = useState<FormData>({
    type_id: '',
    start_date: today,
    end_date: today,
    reason: '',
  });

  const [errors, setErrors] = useState<FormErrors>({});
  const [estimatedDays, setEstimatedDays] = useState(1);

  // Calculate estimated working days when dates change
  useEffect(() => {
    if (formData.start_date && formData.end_date) {
      const days = calculateWorkingDays(formData.start_date, formData.end_date);
      setEstimatedDays(days);
    }
  }, [formData.start_date, formData.end_date]);

  // Get balance for selected type
  const selectedType = absenceTypes.find((t) => t.id === formData.type_id);
  const selectedBalance = balances.find((b) => b.absence_type_id === formData.type_id);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.type_id) {
      newErrors.type_id = 'Please select an absence type';
    }

    if (!formData.start_date) {
      newErrors.start_date = 'Start date is required';
    }

    if (!formData.end_date) {
      newErrors.end_date = 'End date is required';
    } else if (formData.start_date && formData.end_date < formData.start_date) {
      newErrors.end_date = 'End date must be after start date';
    }

    // Check balance if type affects balance
    if (selectedType?.affects_balance && selectedBalance) {
      if (estimatedDays > selectedBalance.remaining) {
        newErrors.type_id = `Insufficient balance. You have ${selectedBalance.remaining} days remaining.`;
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    await onSubmit({
      type_id: formData.type_id,
      start_date: formData.start_date,
      end_date: formData.end_date,
      reason: formData.reason || undefined,
    });
  };

  const handleChange = (field: keyof FormData, value: string) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const formContent = (
    <>
      {error && (
        <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
          {error}
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="type_id">Absence Type</Label>
        <AbsenceTypeSelector
          types={absenceTypes}
          value={formData.type_id}
          onChange={(value) => handleChange('type_id', value)}
          disabled={isLoading}
          error={errors.type_id}
        />
      </div>

      {/* Balance indicator for selected type */}
      {selectedBalance && selectedType?.affects_balance && (
        <div className="p-3 bg-muted rounded-md">
          <div className="flex items-center justify-between text-sm">
            <span className="text-muted-foreground">Available balance:</span>
            <span className="font-medium">{selectedBalance.remaining} days</span>
          </div>
        </div>
      )}

      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="start_date">Start Date</Label>
          <Input
            id="start_date"
            type="date"
            value={formData.start_date}
            onChange={(e) => handleChange('start_date', e.target.value)}
            min={today}
            disabled={isLoading}
            error={errors.start_date}
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="end_date">End Date</Label>
          <Input
            id="end_date"
            type="date"
            value={formData.end_date}
            onChange={(e) => handleChange('end_date', e.target.value)}
            min={formData.start_date || today}
            disabled={isLoading}
            error={errors.end_date}
          />
        </div>
      </div>

      {/* Estimated days */}
      <div className="p-3 bg-muted rounded-md">
        <div className="flex items-center justify-between text-sm">
          <span className="text-muted-foreground">Estimated working days:</span>
          <span className="font-medium">{estimatedDays} {estimatedDays === 1 ? 'day' : 'days'}</span>
        </div>
        <p className="text-xs text-muted-foreground mt-1">
          Weekends excluded. Final count may differ if holidays are included.
        </p>
      </div>

      <div className="space-y-2">
        <Label htmlFor="reason">Reason (optional)</Label>
        <textarea
          id="reason"
          value={formData.reason}
          onChange={(e) => handleChange('reason', e.target.value)}
          disabled={isLoading}
          placeholder="Add any additional notes..."
          className="flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
        />
      </div>
    </>
  );

  const formButtons = (
    <>
      <Button type="button" variant="outline" onClick={onCancel} disabled={isLoading}>
        Cancel
      </Button>
      <Button type="submit" disabled={isLoading}>
        {isLoading ? 'Submitting...' : 'Submit Request'}
      </Button>
    </>
  );

  // Sheet variant - no Card wrapper
  if (variant === 'sheet') {
    return (
      <form onSubmit={handleSubmit} className="flex flex-col gap-4 py-4">
        <div className="space-y-4">
          {formContent}
        </div>
        <div className="flex justify-end gap-4 pt-4 border-t">
          {formButtons}
        </div>
      </form>
    );
  }

  // Default Card variant
  return (
    <Card className="w-full max-w-lg">
      <CardHeader>
        <CardTitle>Request Absence</CardTitle>
        <CardDescription>
          Submit a new absence request for approval.
        </CardDescription>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">
          {formContent}
        </CardContent>
        <CardFooter className="flex justify-end gap-4">
          {formButtons}
        </CardFooter>
      </form>
    </Card>
  );
};
