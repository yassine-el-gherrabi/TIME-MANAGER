import { LoginForm, AuthLayout } from '../components/auth';

export function LoginPage() {
  return (
    <AuthLayout title="Welcome Back">
      <LoginForm />
    </AuthLayout>
  );
}
