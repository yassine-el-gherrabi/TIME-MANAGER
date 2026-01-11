import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../../stores/authStore';
import { authApi } from '../../api/auth';
import { setTokens } from '../../api/client';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import { PasswordRequirements, isPasswordValid } from '../ui/password-requirements';
import { mapErrorToMessage } from '../../utils/errorHandling';
import type { BootstrapRequest } from '../../types/auth';

type Step = 'organization' | 'admin' | 'confirm';

interface FormData {
  organization_name: string;
  organization_slug: string;
  email: string;
  first_name: string;
  last_name: string;
  password: string;
  confirmPassword: string;
}

interface FormErrors {
  organization_name?: string;
  organization_slug?: string;
  email?: string;
  first_name?: string;
  last_name?: string;
  password?: string;
  confirmPassword?: string;
}

export const SetupWizardForm: React.FC = () => {
  const navigate = useNavigate();
  const setUser = useAuthStore((state) => state.setUser);
  const setNeedsSetup = useAuthStore((state) => state.setNeedsSetup);

  const [step, setStep] = useState<Step>('organization');
  const [formData, setFormData] = useState<FormData>({
    organization_name: '',
    organization_slug: '',
    email: '',
    first_name: '',
    last_name: '',
    password: '',
    confirmPassword: '',
  });
  const [errors, setErrors] = useState<FormErrors>({});
  const [apiError, setApiError] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  const generateSlug = (name: string): string => {
    return name
      .toLowerCase()
      .replace(/[^a-z0-9]+/g, '-')
      .replace(/^-|-$/g, '');
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => {
      const updated = { ...prev, [name]: value };
      // Auto-generate slug from organization name
      if (name === 'organization_name') {
        updated.organization_slug = generateSlug(value);
      }
      return updated;
    });
    // Clear error for this field
    if (errors[name as keyof FormErrors]) {
      setErrors((prev) => ({ ...prev, [name]: undefined }));
    }
  };

  const validateOrganization = (): boolean => {
    const newErrors: FormErrors = {};

    if (!formData.organization_name.trim()) {
      newErrors.organization_name = 'Organization name is required';
    } else if (formData.organization_name.length < 2) {
      newErrors.organization_name = 'Organization name must be at least 2 characters';
    }

    if (!formData.organization_slug.trim()) {
      newErrors.organization_slug = 'Organization slug is required';
    } else if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(formData.organization_slug)) {
      newErrors.organization_slug = 'Slug must be lowercase letters, numbers, and hyphens only';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const validateAdmin = (): boolean => {
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

    if (!formData.password) {
      newErrors.password = 'Password is required';
    } else if (!isPasswordValid(formData.password)) {
      newErrors.password = 'Password does not meet all requirements';
    }

    if (!formData.confirmPassword) {
      newErrors.confirmPassword = 'Please confirm your password';
    } else if (formData.password !== formData.confirmPassword) {
      newErrors.confirmPassword = 'Passwords do not match';
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleNext = () => {
    if (step === 'organization' && validateOrganization()) {
      setStep('admin');
    } else if (step === 'admin' && validateAdmin()) {
      setStep('confirm');
    }
  };

  const handleBack = () => {
    if (step === 'admin') setStep('organization');
    else if (step === 'confirm') setStep('admin');
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setApiError('');
    setIsLoading(true);

    try {
      const request: BootstrapRequest = {
        organization_name: formData.organization_name.trim(),
        organization_slug: formData.organization_slug.trim(),
        email: formData.email.trim().toLowerCase(),
        first_name: formData.first_name.trim(),
        last_name: formData.last_name.trim(),
        password: formData.password,
      };

      const response = await authApi.bootstrap(request);

      // Store access token for authenticated requests
      if (response.access_token) {
        setTokens({ access_token: response.access_token });
      }

      // Mark setup as complete
      setNeedsSetup(false);

      // Set user in store for auto-login
      setUser({
        id: response.user.id,
        email: response.user.email,
        first_name: response.user.first_name,
        last_name: response.user.last_name,
        role: response.user.role,
        organization_id: response.user.organization_id,
        created_at: response.user.created_at,
      });

      // Navigate to dashboard
      navigate('/', { replace: true });
    } catch (err) {
      setApiError(mapErrorToMessage(err));
    } finally {
      setIsLoading(false);
    }
  };

  const renderOrganizationStep = () => (
    <>
      <CardHeader>
        <CardTitle>Create Your Organization</CardTitle>
        <CardDescription>Step 1 of 3 - Set up your workspace</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {apiError && (
          <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
            {apiError}
          </div>
        )}
        <div className="space-y-2">
          <Label htmlFor="organization_name">Organization Name</Label>
          <Input
            id="organization_name"
            name="organization_name"
            value={formData.organization_name}
            onChange={handleChange}
            placeholder="My Company"
            error={errors.organization_name}
            disabled={isLoading}
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="organization_slug">URL Slug</Label>
          <Input
            id="organization_slug"
            name="organization_slug"
            value={formData.organization_slug}
            onChange={handleChange}
            placeholder="my-company"
            error={errors.organization_slug}
            disabled={isLoading}
          />
          <p className="text-xs text-muted-foreground">
            Used in URLs. Lowercase letters, numbers, and hyphens only.
          </p>
        </div>
      </CardContent>
      <CardFooter>
        <Button type="button" onClick={handleNext} className="w-full" disabled={isLoading}>
          Continue
        </Button>
      </CardFooter>
    </>
  );

  const renderAdminStep = () => (
    <>
      <CardHeader>
        <CardTitle>Create Admin Account</CardTitle>
        <CardDescription>Step 2 of 3 - Your administrator credentials</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {apiError && (
          <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
            {apiError}
          </div>
        )}
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="first_name">First Name</Label>
            <Input
              id="first_name"
              name="first_name"
              value={formData.first_name}
              onChange={handleChange}
              placeholder="John"
              error={errors.first_name}
              disabled={isLoading}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="last_name">Last Name</Label>
            <Input
              id="last_name"
              name="last_name"
              value={formData.last_name}
              onChange={handleChange}
              placeholder="Doe"
              error={errors.last_name}
              disabled={isLoading}
            />
          </div>
        </div>
        <div className="space-y-2">
          <Label htmlFor="email">Email</Label>
          <Input
            id="email"
            name="email"
            type="email"
            value={formData.email}
            onChange={handleChange}
            placeholder="admin@company.com"
            error={errors.email}
            disabled={isLoading}
            autoComplete="email"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="password">Password</Label>
          <Input
            id="password"
            name="password"
            type="password"
            value={formData.password}
            onChange={handleChange}
            error={errors.password}
            disabled={isLoading}
            autoComplete="new-password"
          />
          {formData.password && <PasswordRequirements password={formData.password} />}
        </div>
        <div className="space-y-2">
          <Label htmlFor="confirmPassword">Confirm Password</Label>
          <Input
            id="confirmPassword"
            name="confirmPassword"
            type="password"
            value={formData.confirmPassword}
            onChange={handleChange}
            error={errors.confirmPassword}
            disabled={isLoading}
            autoComplete="new-password"
          />
        </div>
      </CardContent>
      <CardFooter className="flex gap-2">
        <Button type="button" variant="outline" onClick={handleBack} disabled={isLoading}>
          Back
        </Button>
        <Button type="button" onClick={handleNext} className="flex-1" disabled={isLoading}>
          Continue
        </Button>
      </CardFooter>
    </>
  );

  const renderConfirmStep = () => (
    <>
      <CardHeader>
        <CardTitle>Confirm Setup</CardTitle>
        <CardDescription>Step 3 of 3 - Review and create</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        {apiError && (
          <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
            {apiError}
          </div>
        )}
        <div className="rounded-lg border p-4 space-y-3">
          <div>
            <p className="text-sm text-muted-foreground">Organization</p>
            <p className="font-medium">{formData.organization_name}</p>
            <p className="text-sm text-muted-foreground">/{formData.organization_slug}</p>
          </div>
          <div className="border-t pt-3">
            <p className="text-sm text-muted-foreground">Administrator</p>
            <p className="font-medium">{formData.first_name} {formData.last_name}</p>
            <p className="text-sm text-muted-foreground">{formData.email}</p>
          </div>
        </div>
        <p className="text-sm text-muted-foreground">
          By clicking "Create Workspace", you'll set up your organization and admin account.
          You'll be logged in automatically.
        </p>
      </CardContent>
      <CardFooter className="flex gap-2">
        <Button type="button" variant="outline" onClick={handleBack} disabled={isLoading}>
          Back
        </Button>
        <Button type="submit" className="flex-1" disabled={isLoading}>
          {isLoading ? 'Creating...' : 'Create Workspace'}
        </Button>
      </CardFooter>
    </>
  );

  return (
    <Card className="w-full max-w-md">
      <form onSubmit={handleSubmit}>
        {step === 'organization' && renderOrganizationStep()}
        {step === 'admin' && renderAdminStep()}
        {step === 'confirm' && renderConfirmStep()}
      </form>
    </Card>
  );
};
