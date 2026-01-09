/**
 * Team Form Component
 *
 * Form for creating and editing teams.
 * Supports both card and sheet variants.
 */

import { useState, useEffect } from 'react';
import type { FC, FormEvent } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import type { TeamResponse, CreateTeamRequest } from '../../types/team';

export interface TeamFormProps {
  team?: TeamResponse | null;
  onSubmit: (data: CreateTeamRequest) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string;
  /** Use 'sheet' variant when rendering inside a Sheet/Drawer */
  variant?: 'card' | 'sheet';
  /** Available managers for the dropdown */
  managers?: Array<{ id: string; name: string }>;
  /** Available schedules for the dropdown */
  schedules?: Array<{ id: string; name: string }>;
}

interface FormData {
  name: string;
  description: string;
  manager_id: string;
  work_schedule_id: string;
}

interface FormErrors {
  name?: string;
}

export const TeamForm: FC<TeamFormProps> = ({
  team,
  onSubmit,
  onCancel,
  isLoading,
  error,
  variant = 'card',
  managers = [],
  schedules = [],
}) => {
  const isEditing = !!team;

  const [formData, setFormData] = useState<FormData>({
    name: '',
    description: '',
    manager_id: '',
    work_schedule_id: '',
  });

  const [errors, setErrors] = useState<FormErrors>({});

  useEffect(() => {
    if (team) {
      setFormData({
        name: team.name,
        description: team.description || '',
        manager_id: team.manager_id || '',
        work_schedule_id: '',
      });
    } else {
      setFormData({
        name: '',
        description: '',
        manager_id: '',
        work_schedule_id: '',
      });
    }
  }, [team]);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Team name is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    const data: CreateTeamRequest = {
      name: formData.name.trim(),
    };

    if (formData.description.trim()) {
      data.description = formData.description.trim();
    }

    if (formData.manager_id) {
      data.manager_id = formData.manager_id;
    }

    await onSubmit(data);
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
        <Label htmlFor="team-name">Team Name *</Label>
        <Input
          id="team-name"
          type="text"
          value={formData.name}
          onChange={(e) => handleChange('name', e.target.value)}
          error={errors.name}
          disabled={isLoading}
          placeholder="e.g., Engineering, Marketing"
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="team-description">Description</Label>
        <Input
          id="team-description"
          type="text"
          value={formData.description}
          onChange={(e) => handleChange('description', e.target.value)}
          disabled={isLoading}
          placeholder="Optional team description"
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="team-manager">Manager</Label>
        <select
          id="team-manager"
          value={formData.manager_id}
          onChange={(e) => handleChange('manager_id', e.target.value)}
          disabled={isLoading}
          className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
        >
          <option value="">No manager assigned</option>
          {managers.map((manager) => (
            <option key={manager.id} value={manager.id}>
              {manager.name}
            </option>
          ))}
        </select>
      </div>

      {schedules.length > 0 && (
        <div className="space-y-2">
          <Label htmlFor="team-schedule">Default Schedule</Label>
          <select
            id="team-schedule"
            value={formData.work_schedule_id}
            onChange={(e) => handleChange('work_schedule_id', e.target.value)}
            disabled={isLoading}
            className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
          >
            <option value="">No default schedule</option>
            {schedules.map((schedule) => (
              <option key={schedule.id} value={schedule.id}>
                {schedule.name}
              </option>
            ))}
          </select>
        </div>
      )}
    </>
  );

  const formButtons = (
    <>
      <Button type="button" variant="outline" onClick={onCancel} disabled={isLoading}>
        Cancel
      </Button>
      <Button type="submit" disabled={isLoading}>
        {isLoading ? (isEditing ? 'Saving...' : 'Creating...') : isEditing ? 'Save' : 'Create'}
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
        <CardTitle>{isEditing ? 'Edit Team' : 'Create Team'}</CardTitle>
        <CardDescription>
          {isEditing
            ? 'Update team information'
            : 'Create a new team for your organization.'}
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
