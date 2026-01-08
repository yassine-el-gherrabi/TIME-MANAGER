import React from 'react';

export interface AuthLayoutProps {
  children: React.ReactNode;
  title?: string;
}

export const AuthLayout: React.FC<AuthLayoutProps> = ({ children, title = 'Time Manager' }) => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-4">
      <div className="w-full max-w-md space-y-8">
        <div className="text-center space-y-2">
          <h1 className="text-4xl font-bold tracking-tight text-foreground">{title}</h1>
          <p className="text-sm text-muted-foreground font-medium">Manage your time effectively</p>
        </div>
        {children}
      </div>
    </div>
  );
};
