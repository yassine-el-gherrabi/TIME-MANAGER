import { PasswordResetRequestForm, AuthLayout } from '../components/auth';

export function PasswordResetRequestPage() {
  return (
    <AuthLayout title="Reset Password">
      <PasswordResetRequestForm />
    </AuthLayout>
  );
}
