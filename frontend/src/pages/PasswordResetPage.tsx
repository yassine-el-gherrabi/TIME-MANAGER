import { PasswordResetForm, AuthLayout } from '../components/auth';

export function PasswordResetPage() {
  return (
    <AuthLayout title="Set New Password">
      <PasswordResetForm />
    </AuthLayout>
  );
}
