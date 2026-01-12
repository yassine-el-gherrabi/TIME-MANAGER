/**
 * Profile Page
 *
 * User profile management page for viewing and editing personal information.
 * Users can edit: first_name, last_name, phone
 * Email and role can only be changed by admins.
 */

import { useState, useEffect, type FC, type FormEvent } from 'react';
import { toast } from 'sonner';
import { User, Mail, Shield, Loader2 } from 'lucide-react';
import { Button } from '../../components/ui/button';
import { Input } from '../../components/ui/input';
import { Label } from '../../components/ui/label';
import { PhoneInput } from '../../components/ui/phone-input';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../components/ui/card';
import { useAuth } from '../../hooks/useAuth';
import { usersApi } from '../../api/users';
import { mapErrorToMessage } from '../../utils/errorHandling';
import { UserRole } from '../../types/auth';

/** Get role badge class (matches UsersTable styling) */
const getRoleBadgeClass = (role: UserRole): string => {
  switch (role) {
    case UserRole.SuperAdmin:
      return 'bg-amber-100 text-amber-800 border-amber-200';
    case UserRole.Admin:
      return 'bg-purple-100 text-purple-800 border-purple-200';
    case UserRole.Manager:
      return 'bg-blue-100 text-blue-800 border-blue-200';
    case UserRole.Employee:
    default:
      return 'bg-gray-100 text-gray-800 border-gray-200';
  }
};

/** Format role for display */
const formatRole = (role: UserRole): string => {
  switch (role) {
    case UserRole.SuperAdmin:
      return 'Super Admin';
    case UserRole.Admin:
      return 'Admin';
    case UserRole.Manager:
      return 'Manager';
    case UserRole.Employee:
    default:
      return 'Employee';
  }
};

interface FormData {
  first_name: string;
  last_name: string;
  phone: string;
}

interface FormErrors {
  first_name?: string;
  last_name?: string;
  phone?: string;
}

export const ProfilePage: FC = () => {
  const { user, refreshUser } = useAuth();
  const [isEditing, setIsEditing] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [formData, setFormData] = useState<FormData>({
    first_name: '',
    last_name: '',
    phone: '',
  });
  const [errors, setErrors] = useState<FormErrors>({});

  // Initialize form data when user loads
  useEffect(() => {
    if (user) {
      setFormData({
        first_name: user.first_name || '',
        last_name: user.last_name || '',
        phone: user.phone || '',
      });
    }
  }, [user]);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.first_name.trim()) {
      newErrors.first_name = 'First name is required';
    } else if (formData.first_name.length > 100) {
      newErrors.first_name = 'First name must be at most 100 characters';
    }

    if (!formData.last_name.trim()) {
      newErrors.last_name = 'Last name is required';
    } else if (formData.last_name.length > 100) {
      newErrors.last_name = 'Last name must be at most 100 characters';
    }

    if (formData.phone && formData.phone.length > 20) {
      newErrors.phone = 'Phone number must be at most 20 characters';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleChange = (field: keyof FormData) => (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData((prev) => ({ ...prev, [field]: e.target.value }));
    if (errors[field]) {
      setErrors((prev) => ({ ...prev, [field]: undefined }));
    }
  };

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();

    if (!validateForm() || !user) {
      return;
    }

    setIsSubmitting(true);

    try {
      await usersApi.update(user.id, {
        first_name: formData.first_name.trim(),
        last_name: formData.last_name.trim(),
        phone: formData.phone.trim() || null,
      });

      toast.success('Profile updated successfully');
      setIsEditing(false);

      // Refresh user data in auth context
      await refreshUser();
    } catch (error) {
      toast.error(mapErrorToMessage(error));
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleCancel = () => {
    if (user) {
      setFormData({
        first_name: user.first_name || '',
        last_name: user.last_name || '',
        phone: user.phone || '',
      });
    }
    setErrors({});
    setIsEditing(false);
  };

  if (!user) {
    return (
      <div className="flex items-center justify-center py-8">
        <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-2xl space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Profile</h1>
        <p className="mt-2 text-muted-foreground">
          View and manage your personal information
        </p>
      </div>

      {/* Editable Information */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <User className="h-5 w-5" />
            Personal Information
          </CardTitle>
          <CardDescription>
            Update your name and contact details
          </CardDescription>
        </CardHeader>
        <CardContent>
          {isEditing ? (
            <form onSubmit={handleSubmit} className="space-y-4">
              <div className="grid gap-4 sm:grid-cols-2">
                <div className="space-y-2">
                  <Label htmlFor="first_name">First Name</Label>
                  <Input
                    id="first_name"
                    value={formData.first_name}
                    onChange={handleChange('first_name')}
                    className={errors.first_name ? 'border-destructive' : ''}
                    placeholder="Enter your first name"
                    disabled={isSubmitting}
                  />
                  {errors.first_name && (
                    <p className="text-sm text-destructive">{errors.first_name}</p>
                  )}
                </div>

                <div className="space-y-2">
                  <Label htmlFor="last_name">Last Name</Label>
                  <Input
                    id="last_name"
                    value={formData.last_name}
                    onChange={handleChange('last_name')}
                    className={errors.last_name ? 'border-destructive' : ''}
                    placeholder="Enter your last name"
                    disabled={isSubmitting}
                  />
                  {errors.last_name && (
                    <p className="text-sm text-destructive">{errors.last_name}</p>
                  )}
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="phone">Phone Number</Label>
                <PhoneInput
                  value={formData.phone}
                  onChange={(value) => {
                    setFormData((prev) => ({ ...prev, phone: value }));
                    if (errors.phone) {
                      setErrors((prev) => ({ ...prev, phone: undefined }));
                    }
                  }}
                  disabled={isSubmitting}
                  error={!!errors.phone}
                />
                {errors.phone && (
                  <p className="text-sm text-destructive">{errors.phone}</p>
                )}
              </div>

              <div className="flex justify-end gap-2 pt-4">
                <Button
                  type="button"
                  variant="outline"
                  onClick={handleCancel}
                  disabled={isSubmitting}
                >
                  Cancel
                </Button>
                <Button type="submit" disabled={isSubmitting}>
                  {isSubmitting ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Saving...
                    </>
                  ) : (
                    'Save Changes'
                  )}
                </Button>
              </div>
            </form>
          ) : (
            <div className="space-y-4">
              <div className="grid gap-4 sm:grid-cols-2">
                <div>
                  <p className="text-sm font-medium text-muted-foreground">First Name</p>
                  <p className="mt-1">{user.first_name}</p>
                </div>
                <div>
                  <p className="text-sm font-medium text-muted-foreground">Last Name</p>
                  <p className="mt-1">{user.last_name}</p>
                </div>
              </div>
              <div>
                <p className="text-sm font-medium text-muted-foreground">Phone Number</p>
                <p className="mt-1">{user.phone || '—'}</p>
              </div>
              <div className="pt-4">
                <Button onClick={() => setIsEditing(true)}>
                  Edit Profile
                </Button>
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* Read-only Information */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <Shield className="h-5 w-5" />
            Account Information
          </CardTitle>
          <CardDescription>
            These details are managed by your administrator
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-4">
          <div className="flex items-center gap-3">
            <Mail className="h-4 w-4 text-muted-foreground" />
            <div>
              <p className="text-sm font-medium text-muted-foreground">Email</p>
              <p className="mt-1">{user.email}</p>
            </div>
          </div>
          <div className="flex items-center gap-3">
            <Shield className="h-4 w-4 text-muted-foreground" />
            <div>
              <p className="text-sm font-medium text-muted-foreground">Role</p>
              <span
                className={`mt-1 inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold border ${getRoleBadgeClass(user.role)}`}
              >
                {formatRole(user.role)}
              </span>
            </div>
          </div>
          <p className="text-sm text-muted-foreground pt-2">
            Contact your administrator to change your email or role.
          </p>
        </CardContent>
      </Card>
    </div>
  );
};
