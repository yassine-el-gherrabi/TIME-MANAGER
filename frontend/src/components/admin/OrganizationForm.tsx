/**
 * Organization Form Component
 *
 * Form for creating and editing organizations.
 * Supports both card and sheet variants.
 * Super Admin only.
 */

import { useState, useEffect } from 'react';
import type { FC, FormEvent } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import type { OrganizationResponse, CreateOrganizationRequest } from '../../types/organization';

/**
 * Common timezones for the dropdown
 */
const COMMON_TIMEZONES = [
  'Europe/Paris',
  'Europe/London',
  'Europe/Berlin',
  'Europe/Madrid',
  'Europe/Rome',
  'Europe/Amsterdam',
  'Europe/Brussels',
  'America/New_York',
  'America/Chicago',
  'America/Denver',
  'America/Los_Angeles',
  'America/Toronto',
  'Asia/Tokyo',
  'Asia/Shanghai',
  'Asia/Singapore',
  'Asia/Dubai',
  'Australia/Sydney',
  'Pacific/Auckland',
  'UTC',
];

export interface OrganizationFormProps {
  organization?: OrganizationResponse | null;
  onSubmit: (data: CreateOrganizationRequest) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string;
  /** Use 'sheet' variant when rendering inside a Sheet/Drawer */
  variant?: 'card' | 'sheet';
}

interface FormData {
  name: string;
  slug: string;
  timezone: string;
}

interface FormErrors {
  name?: string;
  slug?: string;
}

/**
 * Generate a slug from a name
 */
const generateSlug = (name: string): string => {
  return name
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
};

export const OrganizationForm: FC<OrganizationFormProps> = ({
  organization,
  onSubmit,
  onCancel,
  isLoading,
  error,
  variant = 'card',
}) => {
  const isEditing = !!organization;

  const [formData, setFormData] = useState<FormData>({
    name: '',
    slug: '',
    timezone: 'Europe/Paris',
  });

  const [errors, setErrors] = useState<FormErrors>({});
  const [autoSlug, setAutoSlug] = useState(true);

  useEffect(() => {
    if (organization) {
      setFormData({
        name: organization.name,
        slug: organization.slug,
        timezone: organization.timezone,
      });
      setAutoSlug(false);
    } else {
      setFormData({
        name: '',
        slug: '',
        timezone: 'Europe/Paris',
      });
      setAutoSlug(true);
    }
  }, [organization]);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.name.trim()) {
      newErrors.name = 'Organization name is required';
    } else if (formData.name.trim().length < 2) {
      newErrors.name = 'Name must be at least 2 characters';
    } else if (formData.name.trim().length > 100) {
      newErrors.name = 'Name must be at most 100 characters';
    }

    if (!isEditing) {
      if (!formData.slug.trim()) {
        newErrors.slug = 'Slug is required';
      } else if (!/^[a-z0-9][a-z0-9-]*[a-z0-9]$|^[a-z0-9]$/.test(formData.slug)) {
        newErrors.slug = 'Slug must contain only lowercase letters, numbers, and hyphens';
      } else if (formData.slug.length < 2) {
        newErrors.slug = 'Slug must be at least 2 characters';
      } else if (formData.slug.length > 50) {
        newErrors.slug = 'Slug must be at most 50 characters';
      }
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    const data: CreateOrganizationRequest = {
      name: formData.name.trim(),
      slug: formData.slug.trim(),
      timezone: formData.timezone,
    };

    await onSubmit(data);
  };

  const handleChange = (field: keyof FormData, value: string) => {
    setFormData((prev) => {
      const newData = { ...prev, [field]: value };

      // Auto-generate slug from name for new organizations
      if (field === 'name' && autoSlug && !isEditing) {
        newData.slug = generateSlug(value);
      }

      return newData;
    });

    if (errors[field as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const handleSlugChange = (value: string) => {
    setAutoSlug(false);
    handleChange('slug', value.toLowerCase().replace(/[^a-z0-9-]/g, ''));
  };

  const formContent = (
    <>
      {error && (
        <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
          {error}
        </div>
      )}

      <div className="space-y-2">
        <Label htmlFor="org-name">Organization Name *</Label>
        <Input
          id="org-name"
          type="text"
          value={formData.name}
          onChange={(e) => handleChange('name', e.target.value)}
          error={errors.name}
          disabled={isLoading}
          placeholder="e.g., Acme Corporation"
        />
      </div>

      <div className="space-y-2">
        <Label htmlFor="org-slug">
          Slug *
          {isEditing && (
            <span className="ml-2 text-xs text-muted-foreground">(cannot be changed)</span>
          )}
        </Label>
        <Input
          id="org-slug"
          type="text"
          value={formData.slug}
          onChange={(e) => handleSlugChange(e.target.value)}
          error={errors.slug}
          disabled={isLoading || isEditing}
          placeholder="e.g., acme-corporation"
        />
        <p className="text-xs text-muted-foreground">
          URL-friendly identifier. Only lowercase letters, numbers, and hyphens.
        </p>
      </div>

      <div className="space-y-2">
        <Label htmlFor="org-timezone">Timezone</Label>
        <select
          id="org-timezone"
          value={formData.timezone}
          onChange={(e) => handleChange('timezone', e.target.value)}
          disabled={isLoading}
          className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
        >
          {COMMON_TIMEZONES.map((tz) => (
            <option key={tz} value={tz}>
              {tz}
            </option>
          ))}
        </select>
      </div>
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
        <div className="space-y-4">{formContent}</div>
        <div className="flex justify-end gap-4 pt-4 border-t">{formButtons}</div>
      </form>
    );
  }

  // Default Card variant
  return (
    <Card className="w-full max-w-lg">
      <CardHeader>
        <CardTitle>{isEditing ? 'Edit Organization' : 'Create Organization'}</CardTitle>
        <CardDescription>
          {isEditing
            ? 'Update organization information'
            : 'Create a new organization for multi-tenant management.'}
        </CardDescription>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">{formContent}</CardContent>
        <CardFooter className="flex justify-end gap-4">{formButtons}</CardFooter>
      </form>
    </Card>
  );
};
