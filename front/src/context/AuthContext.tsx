import React, { createContext, useState, useEffect, type ReactNode } from 'react';
import { toast } from 'sonner';
import type { User, LoginCredentials } from '@/types';
import { authApi } from '@/api/auth';
import { clearAuthData } from '@/api/client';
import { isTokenExpired } from '@/utils/jwt';

interface AuthContextType {
  user: User | null;
  token: string | null;
  login: (credentials: LoginCredentials) => Promise<void>;
  logout: () => Promise<void>;
  loading: boolean;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  // Initialize auth state from localStorage on mount
  useEffect(() => {
    const storedToken = localStorage.getItem('token');
    const storedUser = localStorage.getItem('user');

    if (storedToken && storedUser) {
      // Check if token is expired
      if (isTokenExpired(storedToken)) {
        // Token expired - clear auth data and show message
        clearAuthData();
        toast.error('Session expirée', {
          description: 'Votre session a expiré. Veuillez vous reconnecter.',
        });
        setLoading(false);
        return;
      }

      try {
        setToken(storedToken);
        setUser(JSON.parse(storedUser));
      } catch (error) {
        console.error('Failed to parse stored user data:', error);
        // Clear invalid data
        clearAuthData();
      }
    }

    setLoading(false);
  }, []);

  const login = async (credentials: LoginCredentials) => {
    // Don't clear auth state on login failure - let the error propagate
    const response = await authApi.login(credentials);

    // Store auth data
    setToken(response.token);
    setUser(response.user);

    // Persist to localStorage
    localStorage.setItem('token', response.token);
    localStorage.setItem('user', JSON.stringify(response.user));
  };

  const logout = async () => {
    try {
      await authApi.logout();
    } catch (error) {
      // Continue with logout even if API call fails
      console.warn('Logout API call failed:', error);
    } finally {
      // Clear auth state
      setToken(null);
      setUser(null);

      // Clear localStorage using centralized function
      clearAuthData();
    }
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        token,
        login,
        logout,
        loading,
        isAuthenticated: !!token && !!user,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

// eslint-disable-next-line react-refresh/only-export-components
export const useAuth = () => {
  const context = React.useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within AuthProvider');
  }
  return context;
};
