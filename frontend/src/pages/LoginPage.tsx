import { Navigate } from 'react-router-dom';
import { LoginForm, AuthLayout } from '../components/auth';
import { useAuth } from '../hooks/useAuth';

export function LoginPage() {
  const { needsSetup, isAuthenticated } = useAuth();

  // Redirect to setup wizard if system needs initial setup
  if (needsSetup) {
    return <Navigate to="/setup" replace />;
  }

  // Redirect to dashboard if already authenticated
  if (isAuthenticated) {
    return <Navigate to="/" replace />;
  }

  return (
    <AuthLayout title="Welcome Back">
      <LoginForm />
    </AuthLayout>
  );
}
