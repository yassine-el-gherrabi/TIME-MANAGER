import { useState, useEffect } from 'react';
import type { FC, FormEvent } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import { UserRole } from '../../types/auth';
import type { UserResponse, CreateUserRequest } from '../../types/user';

export interface UserFormProps {
  user?: UserResponse | null;
  onSubmit: (data: CreateUserRequest) => Promise<void>;
  onCancel: () => void;
  isLoading?: boolean;
  error?: string;
  /** Use 'sheet' variant when rendering inside a Sheet/Drawer */
  variant?: 'card' | 'sheet';
}

interface FormData {
  email: string;
  first_name: string;
  last_name: string;
  role: UserRole;
}

interface FormErrors {
  email?: string;
  first_name?: string;
  last_name?: string;
}

export const UserForm: FC<UserFormProps> = ({
  user,
  onSubmit,
  onCancel,
  isLoading,
  error,
  variant = 'card',
}) => {
  const isEditing = !!user;

  const [formData, setFormData] = useState<FormData>({
    email: '',
    first_name: '',
    last_name: '',
    role: UserRole.Employee,
  });

  const [errors, setErrors] = useState<FormErrors>({});

  useEffect(() => {
    if (user) {
      setFormData({
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        role: user.role,
      });
    } else {
      // Reset form when user is cleared (e.g., sheet closed)
      setFormData({
        email: '',
        first_name: '',
        last_name: '',
        role: UserRole.Employee,
      });
    }
  }, [user]);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.email.trim()) {
      newErrors.email = 'Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      newErrors.email = 'Invalid email format';
    }

    if (!formData.first_name.trim()) {
      newErrors.first_name = 'First name is required';
    }

    if (!formData.last_name.trim()) {
      newErrors.last_name = 'Last name is required';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    await onSubmit(formData);
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
        <Label htmlFor="email">Email</Label>
        <Input
          id="email"
          type="email"
          value={formData.email}
          onChange={(e) => handleChange('email', e.target.value)}
          error={errors.email}
          disabled={isLoading}
          autoComplete="email"
        />
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="space-y-2">
          <Label htmlFor="first_name">First Name</Label>
          <Input
            id="first_name"
            type="text"
            value={formData.first_name}
            onChange={(e) => handleChange('first_name', e.target.value)}
            error={errors.first_name}
            disabled={isLoading}
            autoComplete="given-name"
          />
        </div>

        <div className="space-y-2">
          <Label htmlFor="last_name">Last Name</Label>
          <Input
            id="last_name"
            type="text"
            value={formData.last_name}
            onChange={(e) => handleChange('last_name', e.target.value)}
            error={errors.last_name}
            disabled={isLoading}
            autoComplete="family-name"
          />
        </div>
      </div>

      <div className="space-y-2">
        <Label htmlFor="role">Role</Label>
        <select
          id="role"
          value={formData.role}
          onChange={(e) => handleChange('role', e.target.value)}
          disabled={isLoading}
          className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
        >
          <option value={UserRole.Employee}>Employee</option>
          <option value={UserRole.Manager}>Manager</option>
          <option value={UserRole.Admin}>Admin</option>
          <option value={UserRole.SuperAdmin}>Super Admin</option>
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
        <CardTitle>{isEditing ? 'Edit User' : 'Create User'}</CardTitle>
        <CardDescription>
          {isEditing
            ? 'Update user information'
            : 'Create a new user. They will receive an invitation email.'}
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
