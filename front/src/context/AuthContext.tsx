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
    const initializeAuth = async () => {
      const storedToken = localStorage.getItem('token');

      if (storedToken) {
        // Check if token is expired
        if (isTokenExpired(storedToken)) {
          // Token expired - clear auth data and show message
          clearAuthData();
          toast.error('Session Expired', {
            description: 'Your session has expired. Please log in again.',
          });
          setLoading(false);
          return;
        }

        try {
          // Set token first so API calls can use it
          setToken(storedToken);

          // Fetch fresh user data from API
          const userData = await authApi.me();
          setUser(userData);
        } catch (error) {
          console.error('Failed to fetch user data:', error);
          // Clear invalid data
          clearAuthData();
          setToken(null);
          setUser(null);
        }
      }

      setLoading(false);
    };

    initializeAuth();
  }, []);

  const login = async (credentials: LoginCredentials) => {
    // Don't clear auth state on login failure - let the error propagate
    const response = await authApi.login(credentials);

    // Persist token to localStorage
    localStorage.setItem('token', response.token);

    // Set token in state
    setToken(response.token);

    // Fetch fresh user data from API (consistent with initialization)
    const userData = await authApi.me();
    setUser(userData);
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
