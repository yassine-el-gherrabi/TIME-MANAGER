import React from 'react';
import { Navigate } from 'react-router-dom';
import { AuthLayout, SetupWizardForm } from '../components/auth';
import { useAuth } from '../hooks/useAuth';

export const SetupWizardPage: React.FC = () => {
  const { needsSetup, isAuthenticated } = useAuth();

  // Redirect to dashboard if already authenticated
  if (isAuthenticated) {
    return <Navigate to="/" replace />;
  }

  // Redirect to login if setup is not needed
  if (!needsSetup) {
    return <Navigate to="/login" replace />;
  }

  return (
    <AuthLayout title="Welcome to Time Manager">
      <SetupWizardForm />
    </AuthLayout>
  );
};
