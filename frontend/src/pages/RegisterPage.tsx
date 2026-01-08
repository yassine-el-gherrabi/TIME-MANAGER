import { RegisterForm, AuthLayout } from '../components/auth';

export function RegisterPage() {
  return (
    <AuthLayout title="Create Account">
      <RegisterForm />
    </AuthLayout>
  );
}
