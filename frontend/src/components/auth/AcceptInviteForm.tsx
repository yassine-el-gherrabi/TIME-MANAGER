import React, { useState, useEffect } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useAuthStore } from '../../stores/authStore';
import { authApi } from '../../api/auth';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Label } from '../ui/label';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '../ui/card';
import { PasswordRequirements, isPasswordValid } from '../ui/password-requirements';
import { mapErrorToMessage } from '../../utils/errorHandling';

export const AcceptInviteForm: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const token = searchParams.get('token') || '';
  const acceptInvite = useAuthStore((state) => state.acceptInvite);

  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [errors, setErrors] = useState({ password: '', confirmPassword: '' });
  const [apiError, setApiError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isValidating, setIsValidating] = useState(true);
  const [isTokenValid, setIsTokenValid] = useState(false);

  // Validate token on mount
  useEffect(() => {
    const validateToken = async () => {
      if (!token) {
        setApiError('No invitation token provided');
        setIsValidating(false);
        return;
      }

      try {
        const response = await authApi.verifyInvite({ token });
        setIsTokenValid(response.valid);
        if (!response.valid) {
          setApiError(response.message || 'Invalid or expired invitation token');
        }
      } catch (err) {
        setApiError(mapErrorToMessage(err));
        setIsTokenValid(false);
      } finally {
        setIsValidating(false);
      }
    };

    validateToken();
  }, [token]);

  const validateForm = (): boolean => {
    const newErrors = { password: '', confirmPassword: '' };

    if (!password) {
      newErrors.password = 'Password is required';
    } else if (!isPasswordValid(password)) {
      newErrors.password = 'Password does not meet all requirements';
    }

    if (!confirmPassword) {
      newErrors.confirmPassword = 'Please confirm your password';
    } else if (password !== confirmPassword) {
      newErrors.confirmPassword = 'Passwords do not match';
    }

    setErrors(newErrors);
    return !newErrors.password && !newErrors.confirmPassword;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setApiError('');

    if (!token) {
      setApiError('Invalid invitation token');
      return;
    }

    if (!validateForm()) return;

    setIsLoading(true);
    try {
      await acceptInvite({ token, password });
      navigate('/', { replace: true });
    } catch (err) {
      setApiError(mapErrorToMessage(err));
    } finally {
      setIsLoading(false);
    }
  };

  if (isValidating) {
    return (
      <Card className="w-full max-w-md">
        <CardContent className="flex items-center justify-center py-8">
          <div className="text-muted-foreground">Validating invitation...</div>
        </CardContent>
      </Card>
    );
  }

  if (!isTokenValid) {
    return (
      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Invalid Invitation</CardTitle>
          <CardDescription>This invitation link is no longer valid</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
            {apiError || 'The invitation link has expired or has already been used. Please contact your administrator for a new invitation.'}
          </div>
        </CardContent>
        <CardFooter>
          <Button variant="outline" className="w-full" onClick={() => navigate('/login')}>
            Go to Login
          </Button>
        </CardFooter>
      </Card>
    );
  }

  return (
    <Card className="w-full max-w-md">
      <CardHeader>
        <CardTitle>Set Your Password</CardTitle>
        <CardDescription>Create a password to activate your account</CardDescription>
      </CardHeader>
      <form onSubmit={handleSubmit}>
        <CardContent className="space-y-4">
          {apiError && (
            <div className="p-3 text-sm text-destructive bg-destructive/10 border border-destructive rounded-md">
              {apiError}
            </div>
          )}
          <div className="space-y-2">
            <Label htmlFor="password">Password</Label>
            <Input
              id="password"
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              error={errors.password}
              disabled={isLoading}
              autoComplete="new-password"
            />
            {password && <PasswordRequirements password={password} />}
          </div>
          <div className="space-y-2">
            <Label htmlFor="confirmPassword">Confirm Password</Label>
            <Input
              id="confirmPassword"
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              error={errors.confirmPassword}
              disabled={isLoading}
              autoComplete="new-password"
            />
          </div>
        </CardContent>
        <CardFooter>
          <Button type="submit" className="w-full" disabled={isLoading}>
            {isLoading ? 'Activating...' : 'Activate Account'}
          </Button>
        </CardFooter>
      </form>
    </Card>
  );
};
