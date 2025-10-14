# Frontend Development Tickets - React + KrakenD Gateway

> **Project**: Time Manager Frontend
> **Technology Stack**: React 18 + Vite + TailwindCSS + KrakenD Gateway
> **Status**: Planning Phase
> **Last Updated**: 2025-10-06

---

## Table of Contents

1. [Epic 1: Project Setup & Infrastructure](#epic-1-project-setup--infrastructure)
2. [Epic 2: Authentication & Authorization](#epic-2-authentication--authorization)
3. [Epic 3: Employee Features](#epic-3-employee-features)
4. [Epic 4: Manager Features](#epic-4-manager-features)
5. [Epic 5: Shared Components & UI](#epic-5-shared-components--ui)
6. [Epic 6: Testing & Quality](#epic-6-testing--quality)
7. [Epic 7: Optimization & Deployment](#epic-7-optimization--deployment)

---

## Ticket Structure

Each ticket follows this format:
- **ID**: Unique identifier
- **Title**: Clear, actionable title
- **Epic**: Parent epic
- **Story Points**: Complexity estimate (1-13 Fibonacci)
- **Priority**: Critical / High / Medium / Low
- **Dependencies**: Other tickets that must be completed first
- **Acceptance Criteria**: Testable requirements
- **Technical Notes**: Implementation guidance
- **Definition of Done**: Completion checklist

---

## Epic 1: Project Setup & Infrastructure

### FE-001: Initialize React + Vite Project

**Story Points**: 3
**Priority**: Critical
**Dependencies**: None

**Description**:
Set up the base React project with Vite build tool, configure development environment, and establish project structure.

**Acceptance Criteria**:
- [ ] React 18 project initialized with Vite
- [ ] TypeScript configured (tsconfig.json)
- [ ] ESLint + Prettier configured for code quality
- [ ] Git hooks configured (husky + lint-staged)
- [ ] Development server runs on port 5173
- [ ] Hot Module Replacement (HMR) working
- [ ] Basic project structure created

**Technical Notes**:
```bash
# Create project
npm create vite@latest frontend -- --template react-ts

# Project structure
frontend/
├── public/
├── src/
│   ├── assets/
│   ├── components/
│   ├── features/
│   ├── hooks/
│   ├── lib/
│   ├── services/
│   ├── store/
│   ├── types/
│   ├── utils/
│   ├── App.tsx
│   └── main.tsx
├── .eslintrc.json
├── .prettierrc
├── package.json
├── tsconfig.json
└── vite.config.ts
```

**Definition of Done**:
- [x] Code reviewed and merged to develop
- [x] README.md updated with setup instructions
- [x] CI pipeline runs successfully
- [x] Team can run `npm install && npm run dev` successfully

---

### FE-002: Configure TailwindCSS + shadcn/ui

**Story Points**: 2
**Priority**: Critical
**Dependencies**: FE-001

**Description**:
Install and configure TailwindCSS with shadcn/ui component library for consistent, accessible UI components.

**Acceptance Criteria**:
- [ ] TailwindCSS installed and configured
- [ ] PostCSS configured
- [ ] shadcn/ui CLI installed
- [ ] Base components initialized (Button, Input, Card, Dialog)
- [ ] Dark mode support configured
- [ ] Custom theme colors defined (brand colors)
- [ ] Responsive breakpoints configured

**Technical Notes**:
```bash
# Install TailwindCSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# Install shadcn/ui
npx shadcn-ui@latest init

# Add initial components
npx shadcn-ui@latest add button
npx shadcn-ui@latest add input
npx shadcn-ui@latest add card
npx shadcn-ui@latest add dialog
npx shadcn-ui@latest add dropdown-menu
npx shadcn-ui@latest add table
npx shadcn-ui@latest add toast
```

**tailwind.config.js**:
```javascript
export default {
  darkMode: ["class"],
  content: ["./index.html", "./src/**/*.{ts,tsx,js,jsx}"],
  theme: {
    extend: {
      colors: {
        primary: { /* brand colors */ },
        secondary: { /* ... */ },
      },
    },
  },
  plugins: [require("tailwindcss-animate")],
}
```

**Definition of Done**:
- [x] TailwindCSS classes work in components
- [x] Dark mode toggle working
- [x] shadcn/ui components render correctly
- [x] Responsive design tested on mobile/tablet/desktop

---

### FE-003: Setup KrakenD Gateway Integration

**Story Points**: 5
**Priority**: Critical
**Dependencies**: FE-001

**Description**:
Configure API client to communicate with backend through KrakenD API Gateway, including request/response interceptors and error handling.

**Acceptance Criteria**:
- [ ] Axios HTTP client configured
- [ ] Base URL points to KrakenD Gateway
- [ ] Request interceptor adds Authorization header
- [ ] Response interceptor handles token refresh
- [ ] Response interceptor handles KrakenD-specific errors
- [ ] Network error handling implemented
- [ ] API client typed with TypeScript
- [ ] Environment variables configured for gateway URL

**Technical Notes**:

**src/lib/api-client.ts**:
```typescript
import axios, { AxiosError } from 'axios';

const KRAKEND_BASE_URL = import.meta.env.VITE_KRAKEND_URL || 'http://localhost:8080';

export const apiClient = axios.create({
  baseURL: KRAKEND_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  timeout: 10000,
});

// Request interceptor - Add JWT token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Response interceptor - Handle token refresh
apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const originalRequest = error.config as any;

    // Handle 401 Unauthorized - Token expired
    if (error.response?.status === 401 && !originalRequest._retry) {
      originalRequest._retry = true;

      try {
        const refreshToken = localStorage.getItem('refresh_token');
        const { data } = await axios.post(`${KRAKEND_BASE_URL}/auth/refresh`, {
          refresh_token: refreshToken,
        });

        localStorage.setItem('access_token', data.data.access_token);
        localStorage.setItem('refresh_token', data.data.refresh_token);

        originalRequest.headers.Authorization = `Bearer ${data.data.access_token}`;
        return apiClient(originalRequest);
      } catch (refreshError) {
        // Refresh failed - logout user
        localStorage.clear();
        window.location.href = '/login';
        return Promise.reject(refreshError);
      }
    }

    // Handle KrakenD-specific errors
    if (error.response?.status === 503) {
      // Service unavailable - Backend down
      console.error('Backend service unavailable');
    }

    return Promise.reject(error);
  }
);
```

**.env.development**:
```bash
VITE_KRAKEND_URL=http://localhost:8080
```

**.env.production**:
```bash
VITE_KRAKEND_URL=https://api.yourdomain.com
```

**Definition of Done**:
- [x] API client successfully calls KrakenD endpoints
- [x] Token refresh flow working
- [x] Error handling tested (401, 403, 404, 500, 503)
- [x] Environment variables properly configured
- [x] TypeScript types for all API responses

---

### FE-004: Setup React Router

**Story Points**: 3
**Priority**: Critical
**Dependencies**: FE-001, FE-003

**Description**:
Configure React Router for client-side routing with protected routes based on authentication status and user role.

**Acceptance Criteria**:
- [ ] React Router v6 installed
- [ ] Route configuration defined
- [ ] Protected route component created
- [ ] Role-based route guards implemented
- [ ] 404 Not Found page created
- [ ] Redirect logic for unauthenticated users
- [ ] Navigation working without page reload

**Technical Notes**:

**src/router.tsx**:
```typescript
import { createBrowserRouter, Navigate } from 'react-router-dom';
import { ProtectedRoute } from './components/ProtectedRoute';
import { RoleGuard } from './components/RoleGuard';

// Layouts
import { RootLayout } from './layouts/RootLayout';
import { AuthLayout } from './layouts/AuthLayout';
import { DashboardLayout } from './layouts/DashboardLayout';

// Pages
import { LoginPage } from './pages/auth/LoginPage';
import { EmployeeDashboard } from './pages/employee/Dashboard';
import { ManagerDashboard } from './pages/manager/Dashboard';
import { ProfilePage } from './pages/ProfilePage';
import { ClockPage } from './pages/ClockPage';
import { TeamsPage } from './pages/manager/TeamsPage';
import { ReportsPage } from './pages/manager/ReportsPage';
import { NotFoundPage } from './pages/NotFoundPage';

export const router = createBrowserRouter([
  {
    path: '/',
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <Navigate to="/dashboard" replace />,
      },
      {
        path: 'auth',
        element: <AuthLayout />,
        children: [
          {
            path: 'login',
            element: <LoginPage />,
          },
        ],
      },
      {
        path: 'dashboard',
        element: (
          <ProtectedRoute>
            <DashboardLayout />
          </ProtectedRoute>
        ),
        children: [
          {
            index: true,
            element: <EmployeeDashboard />, // Role-based redirect inside
          },
          {
            path: 'clock',
            element: <ClockPage />,
          },
          {
            path: 'profile',
            element: <ProfilePage />,
          },
          // Manager-only routes
          {
            path: 'teams',
            element: (
              <RoleGuard allowedRoles={['manager']}>
                <TeamsPage />
              </RoleGuard>
            ),
          },
          {
            path: 'reports',
            element: (
              <RoleGuard allowedRoles={['manager']}>
                <ReportsPage />
              </RoleGuard>
            ),
          },
        ],
      },
      {
        path: '*',
        element: <NotFoundPage />,
      },
    ],
  },
]);
```

**src/components/ProtectedRoute.tsx**:
```typescript
import { Navigate, useLocation } from 'react-router-dom';
import { useAuthStore } from '@/store/authStore';

export function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { isAuthenticated } = useAuthStore();
  const location = useLocation();

  if (!isAuthenticated) {
    return <Navigate to="/auth/login" state={{ from: location }} replace />;
  }

  return <>{children}</>;
}
```

**Definition of Done**:
- [x] All routes accessible via URL
- [x] Protected routes redirect to login when not authenticated
- [x] Role-based routes enforce permissions
- [x] Browser back/forward buttons work correctly
- [x] Route transitions smooth (no flash of content)

---

### FE-005: Setup State Management (Zustand)

**Story Points**: 3
**Priority**: High
**Dependencies**: FE-001

**Description**:
Configure Zustand for global state management, focusing on authentication state and user data.

**Acceptance Criteria**:
- [ ] Zustand installed
- [ ] Auth store created (user, tokens, isAuthenticated)
- [ ] State persistence configured (localStorage)
- [ ] Store actions defined (login, logout, updateUser)
- [ ] TypeScript types for all stores
- [ ] DevTools integration (Redux DevTools)

**Technical Notes**:

**src/store/authStore.ts**:
```typescript
import { create } from 'zustand';
import { persist, devtools } from 'zustand/middleware';

interface User {
  id: number;
  email: string;
  firstName: string;
  lastName: string;
  role: 'employee' | 'manager';
}

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  login: (user: User, accessToken: string, refreshToken: string) => void;
  logout: () => void;
  updateUser: (user: Partial<User>) => void;
}

export const useAuthStore = create<AuthState>()(
  devtools(
    persist(
      (set) => ({
        user: null,
        isAuthenticated: false,

        login: (user, accessToken, refreshToken) => {
          localStorage.setItem('access_token', accessToken);
          localStorage.setItem('refresh_token', refreshToken);
          set({ user, isAuthenticated: true });
        },

        logout: () => {
          localStorage.clear();
          set({ user: null, isAuthenticated: false });
        },

        updateUser: (updatedUser) => {
          set((state) => ({
            user: state.user ? { ...state.user, ...updatedUser } : null,
          }));
        },
      }),
      {
        name: 'auth-storage',
        partialize: (state) => ({ user: state.user, isAuthenticated: state.isAuthenticated }),
      }
    )
  )
);
```

**Definition of Done**:
- [x] State persists across browser refresh
- [x] Login/logout updates state correctly
- [x] DevTools show state changes
- [x] TypeScript autocomplete working

---

### FE-006: Setup TanStack Query (React Query)

**Story Points**: 3
**Priority**: High
**Dependencies**: FE-003

**Description**:
Configure TanStack Query for server state management, data fetching, caching, and automatic refetching.

**Acceptance Criteria**:
- [ ] @tanstack/react-query installed
- [ ] QueryClientProvider configured
- [ ] Default query options set (staleTime, retry, refetchOnWindowFocus)
- [ ] React Query DevTools integrated
- [ ] Query keys structure defined
- [ ] Custom hooks for common queries created

**Technical Notes**:

**src/lib/queryClient.ts**:
```typescript
import { QueryClient } from '@tanstack/react-query';

export const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5 minutes
      retry: 1,
      refetchOnWindowFocus: false,
      refetchOnReconnect: true,
    },
    mutations: {
      retry: 0,
    },
  },
});
```

**src/main.tsx**:
```typescript
import { QueryClientProvider } from '@tanstack/react-query';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import { queryClient } from './lib/queryClient';

root.render(
  <QueryClientProvider client={queryClient}>
    <App />
    <ReactQueryDevtools initialIsOpen={false} />
  </QueryClientProvider>
);
```

**src/lib/queryKeys.ts**:
```typescript
export const queryKeys = {
  users: {
    all: ['users'] as const,
    lists: () => [...queryKeys.users.all, 'list'] as const,
    list: (filters: string) => [...queryKeys.users.lists(), { filters }] as const,
    details: () => [...queryKeys.users.all, 'detail'] as const,
    detail: (id: number) => [...queryKeys.users.details(), id] as const,
  },
  clocks: {
    all: ['clocks'] as const,
    byUser: (userId: number) => [...queryKeys.clocks.all, userId] as const,
    workingHours: (userId: number) => [...queryKeys.clocks.all, 'hours', userId] as const,
  },
  teams: {
    all: ['teams'] as const,
    detail: (id: number) => [...queryKeys.teams.all, id] as const,
  },
};
```

**Definition of Done**:
- [x] Query client working with API calls
- [x] DevTools showing cached queries
- [x] Queries invalidated on mutations
- [x] Loading/error states handled

---

## Epic 2: Authentication & Authorization

### FE-007: Build Login Page

**Story Points**: 5
**Priority**: Critical
**Dependencies**: FE-002, FE-003, FE-004, FE-005

**Description**:
Create login page with email/password form, form validation, and integration with KrakenD authentication endpoint.

**Acceptance Criteria**:
- [ ] Login form with email and password fields
- [ ] Form validation (React Hook Form + Zod)
- [ ] Submit button with loading state
- [ ] Error messages displayed (invalid credentials, network error)
- [ ] Success redirects to dashboard
- [ ] "Remember me" checkbox (optional)
- [ ] Responsive design (mobile-friendly)
- [ ] Keyboard navigation (Tab, Enter to submit)

**Technical Notes**:

**src/pages/auth/LoginPage.tsx**:
```typescript
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useMutation } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '@/store/authStore';
import { authService } from '@/services/authService';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Alert } from '@/components/ui/alert';

const loginSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(8, 'Password must be at least 8 characters'),
});

type LoginFormData = z.infer<typeof loginSchema>;

export function LoginPage() {
  const navigate = useNavigate();
  const { login } = useAuthStore();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema),
  });

  const loginMutation = useMutation({
    mutationFn: authService.login,
    onSuccess: (data) => {
      login(
        data.data.user,
        data.data.access_token,
        data.data.refresh_token
      );
      navigate('/dashboard');
    },
  });

  const onSubmit = (data: LoginFormData) => {
    loginMutation.mutate(data);
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="max-w-md w-full space-y-8 p-8 bg-white rounded-lg shadow">
        <h2 className="text-3xl font-bold text-center">Time Manager</h2>

        {loginMutation.isError && (
          <Alert variant="destructive">
            Invalid email or password
          </Alert>
        )}

        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <div>
            <Label htmlFor="email">Email</Label>
            <Input
              id="email"
              type="email"
              {...register('email')}
              disabled={loginMutation.isPending}
            />
            {errors.email && (
              <p className="text-sm text-red-600 mt-1">{errors.email.message}</p>
            )}
          </div>

          <div>
            <Label htmlFor="password">Password</Label>
            <Input
              id="password"
              type="password"
              {...register('password')}
              disabled={loginMutation.isPending}
            />
            {errors.password && (
              <p className="text-sm text-red-600 mt-1">{errors.password.message}</p>
            )}
          </div>

          <Button
            type="submit"
            className="w-full"
            disabled={loginMutation.isPending}
          >
            {loginMutation.isPending ? 'Logging in...' : 'Log In'}
          </Button>
        </form>
      </div>
    </div>
  );
}
```

**src/services/authService.ts**:
```typescript
import { apiClient } from '@/lib/api-client';

export const authService = {
  login: async (credentials: { email: string; password: string }) => {
    const response = await apiClient.post('/auth/login', credentials);
    return response.data;
  },

  logout: async () => {
    const refreshToken = localStorage.getItem('refresh_token');
    await apiClient.post('/auth/logout', { refresh_token: refreshToken });
  },

  refresh: async (refreshToken: string) => {
    const response = await apiClient.post('/auth/refresh', {
      refresh_token: refreshToken,
    });
    return response.data;
  },
};
```

**Definition of Done**:
- [x] User can log in with valid credentials
- [x] Error shown for invalid credentials
- [x] Form validation working
- [x] Redirects to dashboard after successful login
- [x] Loading state displayed during login
- [x] Responsive on mobile devices

---

### FE-008: Build Logout Functionality

**Story Points**: 2
**Priority**: High
**Dependencies**: FE-005, FE-007

**Description**:
Implement logout functionality that clears authentication state, invalidates tokens, and redirects to login page.

**Acceptance Criteria**:
- [ ] Logout button in header/navigation
- [ ] Logout API call to KrakenD
- [ ] Local storage cleared (tokens)
- [ ] Auth store reset
- [ ] React Query cache cleared
- [ ] Redirect to login page
- [ ] Confirmation dialog (optional)

**Technical Notes**:

**src/components/LogoutButton.tsx**:
```typescript
import { useMutation } from '@tanstack/react-query';
import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '@/store/authStore';
import { authService } from '@/services/authService';
import { queryClient } from '@/lib/queryClient';
import { Button } from '@/components/ui/button';

export function LogoutButton() {
  const navigate = useNavigate();
  const { logout } = useAuthStore();

  const logoutMutation = useMutation({
    mutationFn: authService.logout,
    onSettled: () => {
      logout();
      queryClient.clear();
      navigate('/auth/login');
    },
  });

  return (
    <Button
      variant="ghost"
      onClick={() => logoutMutation.mutate()}
      disabled={logoutMutation.isPending}
    >
      {logoutMutation.isPending ? 'Logging out...' : 'Logout'}
    </Button>
  );
}
```

**Definition of Done**:
- [x] Logout button visible when authenticated
- [x] Clicking logout clears state and redirects
- [x] Cannot access protected routes after logout
- [x] Fresh login required after logout

---

### FE-009: Implement Role-Based UI

**Story Points**: 3
**Priority**: High
**Dependencies**: FE-005, FE-007

**Description**:
Create components and utilities to conditionally render UI elements based on user role (employee vs manager).

**Acceptance Criteria**:
- [ ] `<RoleGuard>` component for conditional rendering
- [ ] `useRole()` hook for role checking
- [ ] Manager-only navigation items hidden for employees
- [ ] Unauthorized access redirects to 403 page
- [ ] Role-based dashboard redirect

**Technical Notes**:

**src/components/RoleGuard.tsx**:
```typescript
import { useAuthStore } from '@/store/authStore';
import { Navigate } from 'react-router-dom';

interface RoleGuardProps {
  children: React.ReactNode;
  allowedRoles: Array<'employee' | 'manager'>;
}

export function RoleGuard({ children, allowedRoles }: RoleGuardProps) {
  const { user } = useAuthStore();

  if (!user || !allowedRoles.includes(user.role)) {
    return <Navigate to="/403" replace />;
  }

  return <>{children}</>;
}
```

**src/hooks/useRole.ts**:
```typescript
import { useAuthStore } from '@/store/authStore';

export function useRole() {
  const { user } = useAuthStore();

  return {
    isManager: user?.role === 'manager',
    isEmployee: user?.role === 'employee',
    role: user?.role,
  };
}
```

**Definition of Done**:
- [x] Manager-only routes protected
- [x] Manager-only UI elements hidden for employees
- [x] 403 page displayed for unauthorized access
- [x] Role checking works throughout app

---

## Epic 3: Employee Features

### FE-010: Build Clock In/Out Component

**Story Points**: 8
**Priority**: Critical
**Dependencies**: FE-002, FE-006, FE-007

**Description**:
Create the main clock in/out functionality with a prominent button that toggles between clock in and clock out states.

**Acceptance Criteria**:
- [ ] Large, centered clock in/out button
- [ ] Button shows current status (last clock status)
- [ ] Clicking button creates clock entry via KrakenD
- [ ] Loading state during API call
- [ ] Success feedback (toast notification)
- [ ] Error handling (display error message)
- [ ] Current time displayed
- [ ] Last clock time displayed
- [ ] Automatic UI update after clocking

**Technical Notes**:

**src/pages/ClockPage.tsx**:
```typescript
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { useAuthStore } from '@/store/authStore';
import { clockService } from '@/services/clockService';
import { queryKeys } from '@/lib/queryKeys';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';
import { Clock } from 'lucide-react';

export function ClockPage() {
  const { user } = useAuthStore();
  const queryClient = useQueryClient();
  const { toast } = useToast();

  // Get last clock to determine next action
  const { data: lastClock } = useQuery({
    queryKey: queryKeys.clocks.byUser(user!.id),
    queryFn: () => clockService.getLastClock(user!.id),
  });

  const clockMutation = useMutation({
    mutationFn: clockService.toggleClock,
    onSuccess: (data) => {
      queryClient.invalidateQueries({ queryKey: queryKeys.clocks.all });
      toast({
        title: data.message,
        description: `Clocked ${data.data.status} at ${new Date(data.data.time).toLocaleTimeString()}`,
      });
    },
    onError: (error: any) => {
      toast({
        title: 'Error',
        description: error.response?.data?.error || 'Failed to clock in/out',
        variant: 'destructive',
      });
    },
  });

  const nextAction = lastClock?.status === 'arrival' ? 'out' : 'in';

  return (
    <div className="min-h-screen flex flex-col items-center justify-center p-8">
      <div className="text-center space-y-8">
        <h1 className="text-4xl font-bold">Clock {nextAction}</h1>

        <div className="text-6xl font-mono">
          {new Date().toLocaleTimeString()}
        </div>

        <Button
          size="lg"
          onClick={() => clockMutation.mutate()}
          disabled={clockMutation.isPending}
          className="h-32 w-32 rounded-full text-xl font-bold"
        >
          {clockMutation.isPending ? (
            'Processing...'
          ) : (
            <>
              <Clock className="mr-2 h-6 w-6" />
              Clock {nextAction}
            </>
          )}
        </Button>

        {lastClock && (
          <div className="text-sm text-gray-600">
            Last {lastClock.status}:{' '}
            {new Date(lastClock.time).toLocaleString()}
          </div>
        )}
      </div>
    </div>
  );
}
```

**src/services/clockService.ts**:
```typescript
import { apiClient } from '@/lib/api-client';

export const clockService = {
  toggleClock: async () => {
    const response = await apiClient.post('/clocks');
    return response.data;
  },

  getLastClock: async (userId: number) => {
    const response = await apiClient.get(`/users/${userId}/clocks`, {
      params: { limit: 1 },
    });
    return response.data.data[0];
  },

  getClockHistory: async (userId: number, params?: any) => {
    const response = await apiClient.get(`/users/${userId}/clocks`, { params });
    return response.data;
  },

  getWorkingHours: async (userId: number, params: any) => {
    const response = await apiClient.get(`/users/${userId}/working-hours`, { params });
    return response.data;
  },
};
```

**Definition of Done**:
- [x] Button displays correct next action (in/out)
- [x] Clicking button successfully clocks in/out
- [x] Toast notification shows success/error
- [x] Last clock time displayed correctly
- [x] UI updates automatically after clocking
- [x] Loading state prevents double-clicks

---

### FE-011: Build Employee Dashboard

**Story Points**: 8
**Priority**: High
**Dependencies**: FE-006, FE-010

**Description**:
Create employee dashboard showing working hours summary, recent clocks, and quick clock in/out access.

**Acceptance Criteria**:
- [ ] Today's working hours displayed
- [ ] Week's working hours displayed
- [ ] Recent clock entries table (last 10)
- [ ] Quick clock in/out button
- [ ] Working hours chart (last 30 days)
- [ ] Responsive grid layout
- [ ] Auto-refresh on clock in/out

**Technical Notes**:

**src/pages/employee/Dashboard.tsx**:
```typescript
import { useQuery } from '@tanstack/react-query';
import { useAuthStore } from '@/store/authStore';
import { clockService } from '@/services/clockService';
import { queryKeys } from '@/lib/queryKeys';
import { Card } from '@/components/ui/card';
import { WorkingHoursChart } from '@/components/charts/WorkingHoursChart';
import { RecentClocksTable } from '@/components/tables/RecentClocksTable';
import { QuickClockButton } from '@/components/QuickClockButton';

export function EmployeeDashboard() {
  const { user } = useAuthStore();

  const { data: workingHours } = useQuery({
    queryKey: queryKeys.clocks.workingHours(user!.id),
    queryFn: () =>
      clockService.getWorkingHours(user!.id, {
        start_date: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
        end_date: new Date().toISOString().split('T')[0],
      }),
  });

  const { data: recentClocks } = useQuery({
    queryKey: queryKeys.clocks.byUser(user!.id),
    queryFn: () => clockService.getClockHistory(user!.id, { limit: 10 }),
  });

  const todayHours = workingHours?.data.breakdown.find(
    (day: any) => day.date === new Date().toISOString().split('T')[0]
  );

  return (
    <div className="p-8 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Dashboard</h1>
        <QuickClockButton />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">Today's Hours</h3>
          <p className="text-3xl font-bold mt-2">
            {todayHours?.hours.toFixed(1) || '0.0'}h
          </p>
        </Card>

        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">This Week</h3>
          <p className="text-3xl font-bold mt-2">
            {workingHours?.data.summary.average_daily_hours.toFixed(1) || '0.0'}h
          </p>
        </Card>

        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">This Month</h3>
          <p className="text-3xl font-bold mt-2">
            {workingHours?.data.summary.total_hours.toFixed(1) || '0.0'}h
          </p>
        </Card>
      </div>

      <Card className="p-6">
        <h2 className="text-xl font-semibold mb-4">Working Hours (Last 30 Days)</h2>
        <WorkingHoursChart data={workingHours?.data.breakdown || []} />
      </Card>

      <Card className="p-6">
        <h2 className="text-xl font-semibold mb-4">Recent Clock Entries</h2>
        <RecentClocksTable data={recentClocks?.data || []} />
      </Card>
    </div>
  );
}
```

**Definition of Done**:
- [x] Dashboard displays working hours correctly
- [x] Chart visualizes 30 days of data
- [x] Recent clocks table shows last 10 entries
- [x] Quick clock button works
- [x] Data refreshes after clocking in/out
- [x] Responsive on mobile/tablet

---

### FE-012: Build Clock History Page

**Story Points**: 5
**Priority**: Medium
**Dependencies**: FE-006, FE-010

**Description**:
Create a paginated view of all clock entries with filtering by date range.

**Acceptance Criteria**:
- [ ] Table showing all clock entries
- [ ] Columns: Date, Time, Status (arrival/departure)
- [ ] Date range filter (start date, end date)
- [ ] Pagination (20 items per page)
- [ ] Sort by date (ascending/descending)
- [ ] Empty state when no clocks
- [ ] Export to CSV button (optional)

**Technical Notes**:

**src/pages/employee/ClockHistory.tsx**:
```typescript
import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useAuthStore } from '@/store/authStore';
import { clockService } from '@/services/clockService';
import { queryKeys } from '@/lib/queryKeys';
import { Table } from '@/components/ui/table';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

export function ClockHistoryPage() {
  const { user } = useAuthStore();
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [page, setPage] = useState(1);

  const { data, isLoading } = useQuery({
    queryKey: [...queryKeys.clocks.byUser(user!.id), { startDate, endDate, page }],
    queryFn: () =>
      clockService.getClockHistory(user!.id, {
        start_date: startDate,
        end_date: endDate,
        page,
        limit: 20,
      }),
  });

  return (
    <div className="p-8">
      <h1 className="text-3xl font-bold mb-6">Clock History</h1>

      <div className="flex gap-4 mb-6">
        <Input
          type="date"
          value={startDate}
          onChange={(e) => setStartDate(e.target.value)}
          placeholder="Start Date"
        />
        <Input
          type="date"
          value={endDate}
          onChange={(e) => setEndDate(e.target.value)}
          placeholder="End Date"
        />
        <Button onClick={() => setPage(1)}>Filter</Button>
      </div>

      <Table>
        <thead>
          <tr>
            <th>Date</th>
            <th>Time</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {data?.data.map((clock: any) => (
            <tr key={clock.id}>
              <td>{new Date(clock.time).toLocaleDateString()}</td>
              <td>{new Date(clock.time).toLocaleTimeString()}</td>
              <td className="capitalize">{clock.status}</td>
            </tr>
          ))}
        </tbody>
      </Table>

      {/* Pagination */}
      <div className="flex justify-center gap-2 mt-6">
        <Button
          disabled={page === 1}
          onClick={() => setPage(page - 1)}
        >
          Previous
        </Button>
        <span className="flex items-center px-4">
          Page {data?.meta.current_page} of {data?.meta.total_pages}
        </span>
        <Button
          disabled={page === data?.meta.total_pages}
          onClick={() => setPage(page + 1)}
        >
          Next
        </Button>
      </div>
    </div>
  );
}
```

**Definition of Done**:
- [x] Table displays all clock entries
- [x] Date filtering works
- [x] Pagination works
- [x] Empty state displayed when no clocks
- [x] Responsive table on mobile

---

### FE-013: Build Profile Page

**Story Points**: 5
**Priority**: Medium
**Dependencies**: FE-002, FE-006

**Description**:
Create user profile page where employees can view and edit their personal information.

**Acceptance Criteria**:
- [ ] Display current user information
- [ ] Edit form (first name, last name, phone, email)
- [ ] Form validation
- [ ] Save button with loading state
- [ ] Success/error notifications
- [ ] Password change section (optional)
- [ ] Delete account button (with confirmation)

**Technical Notes**:

**src/pages/ProfilePage.tsx**:
```typescript
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useAuthStore } from '@/store/authStore';
import { userService } from '@/services/userService';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { useToast } from '@/components/ui/use-toast';

const profileSchema = z.object({
  first_name: z.string().min(1, 'First name required'),
  last_name: z.string().min(1, 'Last name required'),
  email: z.string().email('Invalid email'),
  phone_number: z.string().optional(),
});

type ProfileFormData = z.infer<typeof profileSchema>;

export function ProfilePage() {
  const { user, updateUser } = useAuthStore();
  const queryClient = useQueryClient();
  const { toast } = useToast();

  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<ProfileFormData>({
    resolver: zodResolver(profileSchema),
    defaultValues: {
      first_name: user?.firstName,
      last_name: user?.lastName,
      email: user?.email,
      phone_number: user?.phoneNumber,
    },
  });

  const updateMutation = useMutation({
    mutationFn: (data: ProfileFormData) =>
      userService.updateUser(user!.id, data),
    onSuccess: (data) => {
      updateUser(data.data);
      queryClient.invalidateQueries({ queryKey: ['users'] });
      toast({
        title: 'Success',
        description: 'Profile updated successfully',
      });
    },
  });

  return (
    <div className="p-8 max-w-2xl mx-auto">
      <h1 className="text-3xl font-bold mb-6">Profile</h1>

      <form onSubmit={handleSubmit((data) => updateMutation.mutate(data))} className="space-y-6">
        <div className="grid grid-cols-2 gap-4">
          <div>
            <Label htmlFor="first_name">First Name</Label>
            <Input {...register('first_name')} />
            {errors.first_name && (
              <p className="text-sm text-red-600">{errors.first_name.message}</p>
            )}
          </div>

          <div>
            <Label htmlFor="last_name">Last Name</Label>
            <Input {...register('last_name')} />
            {errors.last_name && (
              <p className="text-sm text-red-600">{errors.last_name.message}</p>
            )}
          </div>
        </div>

        <div>
          <Label htmlFor="email">Email</Label>
          <Input {...register('email')} />
          {errors.email && (
            <p className="text-sm text-red-600">{errors.email.message}</p>
          )}
        </div>

        <div>
          <Label htmlFor="phone_number">Phone Number</Label>
          <Input {...register('phone_number')} />
        </div>

        <div className="flex gap-4">
          <Button type="submit" disabled={updateMutation.isPending}>
            {updateMutation.isPending ? 'Saving...' : 'Save Changes'}
          </Button>
        </div>
      </form>
    </div>
  );
}
```

**src/services/userService.ts**:
```typescript
import { apiClient } from '@/lib/api-client';

export const userService = {
  getUsers: async (params?: any) => {
    const response = await apiClient.get('/users', { params });
    return response.data;
  },

  getUser: async (id: number) => {
    const response = await apiClient.get(`/users/${id}`);
    return response.data;
  },

  createUser: async (data: any) => {
    const response = await apiClient.post('/users', data);
    return response.data;
  },

  updateUser: async (id: number, data: any) => {
    const response = await apiClient.put(`/users/${id}`, data);
    return response.data;
  },

  deleteUser: async (id: number) => {
    await apiClient.delete(`/users/${id}`);
  },
};
```

**Definition of Done**:
- [x] Profile information displayed
- [x] User can edit and save changes
- [x] Form validation working
- [x] Success notification shown
- [x] Auth store updated after save

---

## Epic 4: Manager Features

### FE-014: Build Manager Dashboard

**Story Points**: 8
**Priority**: High
**Dependencies**: FE-006, FE-009

**Description**:
Create manager dashboard showing team statistics, KPIs, and quick actions.

**Acceptance Criteria**:
- [ ] Team overview cards (total members, active today, average hours)
- [ ] KPI cards (lateness rate, weekly average)
- [ ] Recent team activity feed
- [ ] Quick actions (add user, view reports)
- [ ] Team working hours chart
- [ ] Responsive layout

**Technical Notes**:

**src/pages/manager/Dashboard.tsx**:
```typescript
import { useQuery } from '@tanstack/react-query';
import { reportService } from '@/services/reportService';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Link } from 'react-router-dom';

export function ManagerDashboard() {
  const { data: reports } = useQuery({
    queryKey: ['reports', 'global'],
    queryFn: () =>
      reportService.getGlobalReports({
        start_date: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000)
          .toISOString()
          .split('T')[0],
        end_date: new Date().toISOString().split('T')[0],
      }),
  });

  return (
    <div className="p-8 space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">Manager Dashboard</h1>
        <div className="flex gap-2">
          <Button asChild>
            <Link to="/dashboard/teams">Manage Teams</Link>
          </Button>
          <Button asChild variant="outline">
            <Link to="/dashboard/reports">View Reports</Link>
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">Total Employees</h3>
          <p className="text-3xl font-bold mt-2">
            {reports?.data.kpis.total_employees || 0}
          </p>
        </Card>

        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">Active Today</h3>
          <p className="text-3xl font-bold mt-2">
            {reports?.data.kpis.active_employees || 0}
          </p>
        </Card>

        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">Lateness Rate</h3>
          <p className="text-3xl font-bold mt-2">
            {reports?.data.kpis.lateness_rate.value.toFixed(1) || 0}%
          </p>
        </Card>

        <Card className="p-6">
          <h3 className="text-sm font-medium text-gray-600">Avg Weekly Hours</h3>
          <p className="text-3xl font-bold mt-2">
            {reports?.data.kpis.average_weekly_hours.value.toFixed(1) || 0}h
          </p>
        </Card>
      </div>

      <Card className="p-6">
        <h2 className="text-xl font-semibold mb-4">Teams Overview</h2>
        <div className="space-y-4">
          {reports?.data.teams.map((team: any) => (
            <div key={team.team_id} className="flex items-center justify-between p-4 border rounded">
              <div>
                <h3 className="font-semibold">{team.name}</h3>
                <p className="text-sm text-gray-600">
                  Avg: {team.average_hours.toFixed(1)}h | Lateness: {team.lateness_rate.toFixed(1)}%
                </p>
              </div>
              <Button asChild variant="outline" size="sm">
                <Link to={`/dashboard/teams/${team.team_id}`}>View Details</Link>
              </Button>
            </div>
          ))}
        </div>
      </Card>
    </div>
  );
}
```

**src/services/reportService.ts**:
```typescript
import { apiClient } from '@/lib/api-client';

export const reportService = {
  getGlobalReports: async (params: any) => {
    const response = await apiClient.get('/reports', { params });
    return response.data;
  },

  getTeamReports: async (teamId: number, params: any) => {
    const response = await apiClient.get(`/teams/${teamId}/reports`, { params });
    return response.data;
  },
};
```

**Definition of Done**:
- [x] Dashboard shows team statistics
- [x] KPI cards display correct data
- [x] Teams overview list displayed
- [x] Navigation to teams/reports works
- [x] Data refreshes on page load

---

### FE-015: Build Teams Management Page

**Story Points**: 13
**Priority**: High
**Dependencies**: FE-006, FE-009

**Description**:
Create comprehensive team management page with CRUD operations for teams and team members.

**Acceptance Criteria**:
- [ ] List all teams in table
- [ ] Create new team modal
- [ ] Edit team modal
- [ ] Delete team with confirmation
- [ ] View team details (members list)
- [ ] Add member to team
- [ ] Remove member from team
- [ ] Search/filter teams
- [ ] Pagination

**Technical Notes**:

**src/pages/manager/TeamsPage.tsx**:
```typescript
import { useState } from 'react';
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { teamService } from '@/services/teamService';
import { Button } from '@/components/ui/button';
import { Dialog } from '@/components/ui/dialog';
import { Table } from '@/components/ui/table';
import { CreateTeamDialog } from '@/components/teams/CreateTeamDialog';
import { EditTeamDialog } from '@/components/teams/EditTeamDialog';
import { DeleteTeamDialog } from '@/components/teams/DeleteTeamDialog';
import { TeamMembersDialog } from '@/components/teams/TeamMembersDialog';

export function TeamsPage() {
  const queryClient = useQueryClient();
  const [createOpen, setCreateOpen] = useState(false);
  const [selectedTeam, setSelectedTeam] = useState<any>(null);

  const { data: teams } = useQuery({
    queryKey: ['teams'],
    queryFn: teamService.getTeams,
  });

  const deleteMutation = useMutation({
    mutationFn: teamService.deleteTeam,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['teams'] });
    },
  });

  return (
    <div className="p-8">
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold">Teams</h1>
        <Button onClick={() => setCreateOpen(true)}>Create Team</Button>
      </div>

      <Table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Description</th>
            <th>Manager</th>
            <th>Members</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {teams?.data.map((team: any) => (
            <tr key={team.id}>
              <td>{team.name}</td>
              <td>{team.description}</td>
              <td>
                {team.manager.first_name} {team.manager.last_name}
              </td>
              <td>{team.member_count}</td>
              <td className="flex gap-2">
                <Button size="sm" variant="outline" onClick={() => setSelectedTeam(team)}>
                  View Members
                </Button>
                <Button size="sm" variant="outline">Edit</Button>
                <Button size="sm" variant="destructive">Delete</Button>
              </td>
            </tr>
          ))}
        </tbody>
      </Table>

      <CreateTeamDialog open={createOpen} onOpenChange={setCreateOpen} />
      {selectedTeam && (
        <TeamMembersDialog
          team={selectedTeam}
          open={!!selectedTeam}
          onOpenChange={() => setSelectedTeam(null)}
        />
      )}
    </div>
  );
}
```

**src/services/teamService.ts**:
```typescript
import { apiClient } from '@/lib/api-client';

export const teamService = {
  getTeams: async () => {
    const response = await apiClient.get('/teams');
    return response.data;
  },

  getTeam: async (id: number) => {
    const response = await apiClient.get(`/teams/${id}`);
    return response.data;
  },

  createTeam: async (data: any) => {
    const response = await apiClient.post('/teams', data);
    return response.data;
  },

  updateTeam: async (id: number, data: any) => {
    const response = await apiClient.put(`/teams/${id}`, data);
    return response.data;
  },

  deleteTeam: async (id: number) => {
    await apiClient.delete(`/teams/${id}`);
  },

  addMember: async (teamId: number, userId: number) => {
    const response = await apiClient.post(`/teams/${teamId}/members`, { user_id: userId });
    return response.data;
  },

  removeMember: async (teamId: number, userId: number) => {
    await apiClient.delete(`/teams/${teamId}/members/${userId}`);
  },
};
```

**Definition of Done**:
- [x] Teams list displayed
- [x] Create team working
- [x] Edit team working
- [x] Delete team working
- [x] View team members working
- [x] Add/remove members working
- [x] All CRUD operations refresh UI

---

### FE-016: Build User Management Page

**Story Points**: 8
**Priority**: High
**Dependencies**: FE-006, FE-009

**Description**:
Create user management page where managers can view, create, edit, and delete employee accounts.

**Acceptance Criteria**:
- [ ] List all users in table
- [ ] Create user modal
- [ ] Edit user modal
- [ ] Delete user with confirmation
- [ ] Search users by name/email
- [ ] Filter by role (employee/manager)
- [ ] Pagination
- [ ] View user details (teams, working hours)

**Technical Notes**: Similar to FE-015 but for users

**Definition of Done**:
- [x] Users list displayed
- [x] Create user working
- [x] Edit user working
- [x] Delete user working
- [x] Search/filter working
- [x] Pagination working

---

### FE-017: Build Reports & KPI Page

**Story Points**: 13
**Priority**: High
**Dependencies**: FE-006, FE-009

**Description**:
Create comprehensive reports page with KPI visualizations, charts, and data export.

**Acceptance Criteria**:
- [ ] Date range selector
- [ ] KPI cards (lateness rate, average hours)
- [ ] Working hours trends chart
- [ ] Team comparison chart
- [ ] Top performers table
- [ ] Export to CSV/PDF (optional)
- [ ] Filter by team
- [ ] Responsive charts

**Technical Notes**:

**src/pages/manager/ReportsPage.tsx**:
```typescript
import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { reportService } from '@/services/reportService';
import { Card } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Select } from '@/components/ui/select';
import { Button } from '@/components/ui/button';
import { KPICards } from '@/components/reports/KPICards';
import { WorkingHoursTrendChart } from '@/components/charts/WorkingHoursTrendChart';
import { TeamComparisonChart } from '@/components/charts/TeamComparisonChart';

export function ReportsPage() {
  const [startDate, setStartDate] = useState('');
  const [endDate, setEndDate] = useState('');
  const [teamId, setTeamId] = useState<number | null>(null);

  const { data: reports } = useQuery({
    queryKey: ['reports', { startDate, endDate, teamId }],
    queryFn: () =>
      reportService.getGlobalReports({
        start_date: startDate,
        end_date: endDate,
        team_id: teamId,
      }),
    enabled: !!startDate && !!endDate,
  });

  return (
    <div className="p-8 space-y-6">
      <h1 className="text-3xl font-bold">Reports & KPIs</h1>

      <Card className="p-6">
        <div className="flex gap-4">
          <Input
            type="date"
            value={startDate}
            onChange={(e) => setStartDate(e.target.value)}
            placeholder="Start Date"
          />
          <Input
            type="date"
            value={endDate}
            onChange={(e) => setEndDate(e.target.value)}
            placeholder="End Date"
          />
          <Select onValueChange={(value) => setTeamId(Number(value))}>
            <option value="">All Teams</option>
            {/* Team options */}
          </Select>
          <Button>Generate Report</Button>
        </div>
      </Card>

      {reports && (
        <>
          <KPICards kpis={reports.data.kpis} />

          <Card className="p-6">
            <h2 className="text-xl font-semibold mb-4">Working Hours Trend</h2>
            <WorkingHoursTrendChart data={reports.data} />
          </Card>

          <Card className="p-6">
            <h2 className="text-xl font-semibold mb-4">Team Comparison</h2>
            <TeamComparisonChart teams={reports.data.teams} />
          </Card>

          <Card className="p-6">
            <h2 className="text-xl font-semibold mb-4">Top Performers</h2>
            <Table>
              <thead>
                <tr>
                  <th>Employee</th>
                  <th>Total Hours</th>
                  <th>Late Count</th>
                </tr>
              </thead>
              <tbody>
                {reports.data.top_performers.map((performer: any) => (
                  <tr key={performer.user_id}>
                    <td>{performer.name}</td>
                    <td>{performer.hours.toFixed(1)}h</td>
                    <td>{performer.lateness_count}</td>
                  </tr>
                ))}
              </tbody>
            </Table>
          </Card>
        </>
      )}
    </div>
  );
}
```

**Definition of Done**:
- [x] Date range filter works
- [x] KPI cards display correct data
- [x] Charts render correctly
- [x] Team filter works
- [x] Top performers table displayed
- [x] Responsive on all devices

---

## Epic 5: Shared Components & UI

### FE-018: Build Layout Components

**Story Points**: 5
**Priority**: High
**Dependencies**: FE-002, FE-004

**Description**:
Create reusable layout components (Header, Sidebar, Footer) used across all pages.

**Acceptance Criteria**:
- [ ] Header with logo and user menu
- [ ] Sidebar with navigation links
- [ ] Role-based navigation (employee vs manager)
- [ ] Responsive sidebar (mobile drawer)
- [ ] Active route highlighting
- [ ] Logout button in header

**Technical Notes**:

**src/layouts/DashboardLayout.tsx**:
```typescript
import { Outlet } from 'react-router-dom';
import { Header } from '@/components/layout/Header';
import { Sidebar } from '@/components/layout/Sidebar';

export function DashboardLayout() {
  return (
    <div className="min-h-screen bg-gray-50">
      <Header />
      <div className="flex">
        <Sidebar />
        <main className="flex-1">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
```

**src/components/layout/Sidebar.tsx**:
```typescript
import { Link, useLocation } from 'react-router-dom';
import { useRole } from '@/hooks/useRole';
import { Clock, LayoutDashboard, Users, FileText, UserCircle } from 'lucide-react';

export function Sidebar() {
  const location = useLocation();
  const { isManager } = useRole();

  const links = [
    { to: '/dashboard', label: 'Dashboard', icon: LayoutDashboard },
    { to: '/dashboard/clock', label: 'Clock', icon: Clock },
    { to: '/dashboard/profile', label: 'Profile', icon: UserCircle },
  ];

  if (isManager) {
    links.push(
      { to: '/dashboard/teams', label: 'Teams', icon: Users },
      { to: '/dashboard/reports', label: 'Reports', icon: FileText }
    );
  }

  return (
    <aside className="w-64 bg-white border-r min-h-screen p-4">
      <nav className="space-y-2">
        {links.map((link) => {
          const Icon = link.icon;
          const isActive = location.pathname === link.to;

          return (
            <Link
              key={link.to}
              to={link.to}
              className={`flex items-center gap-3 px-4 py-2 rounded-lg transition-colors ${
                isActive
                  ? 'bg-blue-50 text-blue-600'
                  : 'text-gray-700 hover:bg-gray-50'
              }`}
            >
              <Icon className="h-5 w-5" />
              {link.label}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
```

**Definition of Done**:
- [x] Layouts render correctly
- [x] Navigation working
- [x] Role-based links shown/hidden
- [x] Active route highlighted
- [x] Responsive sidebar

---

### FE-019: Build Chart Components (Recharts)

**Story Points**: 8
**Priority**: Medium
**Dependencies**: FE-011, FE-017

**Description**:
Create reusable chart components using Recharts library for data visualization.

**Acceptance Criteria**:
- [ ] Working hours line chart
- [ ] Team comparison bar chart
- [ ] KPI trend charts
- [ ] Responsive charts
- [ ] Tooltips with formatted data
- [ ] Loading states
- [ ] Empty states

**Technical Notes**:

```bash
npm install recharts
```

**src/components/charts/WorkingHoursChart.tsx**:
```typescript
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { format } from 'date-fns';

interface WorkingHoursChartProps {
  data: Array<{ date: string; hours: number }>;
}

export function WorkingHoursChart({ data }: WorkingHoursChartProps) {
  return (
    <ResponsiveContainer width="100%" height={300}>
      <LineChart data={data}>
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis
          dataKey="date"
          tickFormatter={(date) => format(new Date(date), 'MMM dd')}
        />
        <YAxis
          label={{ value: 'Hours', angle: -90, position: 'insideLeft' }}
        />
        <Tooltip
          labelFormatter={(date) => format(new Date(date), 'PPPP')}
          formatter={(value: number) => [`${value.toFixed(2)}h`, 'Hours']}
        />
        <Line
          type="monotone"
          dataKey="hours"
          stroke="#3b82f6"
          strokeWidth={2}
          dot={{ r: 4 }}
        />
      </LineChart>
    </ResponsiveContainer>
  );
}
```

**Definition of Done**:
- [x] Charts render correctly
- [x] Data formatted properly
- [x] Responsive on all screen sizes
- [x] Tooltips working
- [x] Loading/empty states

---

### FE-020: Build Table Components

**Story Points**: 5
**Priority**: Medium
**Dependencies**: FE-002

**Description**:
Create reusable table components with sorting, pagination, and filtering.

**Acceptance Criteria**:
- [ ] Sortable columns
- [ ] Pagination controls
- [ ] Loading skeleton
- [ ] Empty state
- [ ] Row actions (edit, delete)
- [ ] Responsive (horizontal scroll on mobile)

**Technical Notes**: Use shadcn/ui Table component as base, enhance with sorting/pagination logic

**Definition of Done**:
- [x] Table component reusable
- [x] Sorting working
- [x] Pagination working
- [x] Empty/loading states
- [x] Responsive

---

## Epic 6: Testing & Quality

### FE-021: Setup Testing Infrastructure

**Story Points**: 5
**Priority**: High
**Dependencies**: FE-001

**Description**:
Configure testing framework (Vitest + React Testing Library) for unit and integration tests.

**Acceptance Criteria**:
- [ ] Vitest configured
- [ ] React Testing Library installed
- [ ] Test utilities created (render with providers)
- [ ] Mock service worker (MSW) configured
- [ ] Coverage reporting configured
- [ ] Test scripts in package.json

**Technical Notes**:

```bash
npm install -D vitest @testing-library/react @testing-library/jest-dom @testing-library/user-event jsdom
npm install -D msw
```

**vite.config.ts**:
```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './src/test/setup.ts',
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: ['node_modules/', 'src/test/'],
    },
  },
});
```

**package.json**:
```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

**Definition of Done**:
- [x] Tests run with `npm test`
- [x] Coverage report generated
- [x] Test utilities working
- [x] MSW mocking API calls

---

### FE-022: Write Component Tests

**Story Points**: 8
**Priority**: Medium
**Dependencies**: FE-021

**Description**:
Write unit tests for critical components (Login, Clock Button, Dashboard).

**Acceptance Criteria**:
- [ ] Login form tests (validation, submission)
- [ ] Clock button tests (toggle states)
- [ ] Dashboard tests (data display)
- [ ] Table component tests (sorting, pagination)
- [ ] Chart component tests (rendering)
- [ ] 70%+ code coverage

**Technical Notes**:

**src/pages/auth/LoginPage.test.tsx**:
```typescript
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { BrowserRouter } from 'react-router-dom';
import { LoginPage } from './LoginPage';
import { server } from '@/test/mocks/server';
import { rest } from 'msw';

const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

const wrapper = ({ children }: { children: React.ReactNode }) => (
  <QueryClientProvider client={queryClient}>
    <BrowserRouter>{children}</BrowserRouter>
  </QueryClientProvider>
);

describe('LoginPage', () => {
  it('renders login form', () => {
    render(<LoginPage />, { wrapper });
    expect(screen.getByLabelText(/email/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/password/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /log in/i })).toBeInTheDocument();
  });

  it('shows validation errors', async () => {
    render(<LoginPage />, { wrapper });

    await userEvent.click(screen.getByRole('button', { name: /log in/i }));

    expect(await screen.findByText(/invalid email/i)).toBeInTheDocument();
  });

  it('successfully logs in', async () => {
    server.use(
      rest.post('/auth/login', (req, res, ctx) => {
        return res(
          ctx.json({
            data: {
              user: { id: 1, email: 'test@example.com', role: 'employee' },
              access_token: 'token',
              refresh_token: 'refresh',
            },
          })
        );
      })
    );

    render(<LoginPage />, { wrapper });

    await userEvent.type(screen.getByLabelText(/email/i), 'test@example.com');
    await userEvent.type(screen.getByLabelText(/password/i), 'password123');
    await userEvent.click(screen.getByRole('button', { name: /log in/i }));

    await waitFor(() => {
      expect(window.location.pathname).toBe('/dashboard');
    });
  });
});
```

**Definition of Done**:
- [x] Tests pass
- [x] Coverage ≥70%
- [x] Critical paths tested
- [x] CI runs tests

---

### FE-023: E2E Testing Setup (Optional)

**Story Points**: 8
**Priority**: Low
**Dependencies**: FE-021

**Description**:
Setup Playwright for end-to-end testing of critical user flows.

**Acceptance Criteria**:
- [ ] Playwright installed
- [ ] E2E test for login flow
- [ ] E2E test for clock in/out flow
- [ ] E2E test for creating team (manager)
- [ ] Tests run in CI

**Definition of Done**:
- [x] E2E tests pass locally
- [x] Tests run in CI
- [x] Critical flows covered

---

## Epic 7: Optimization & Deployment

### FE-024: Performance Optimization

**Story Points**: 5
**Priority**: Medium
**Dependencies**: FE-011, FE-014

**Description**:
Optimize application performance (code splitting, lazy loading, image optimization).

**Acceptance Criteria**:
- [ ] Route-based code splitting
- [ ] Lazy loading for charts/heavy components
- [ ] Image optimization
- [ ] Bundle size < 500KB (gzipped)
- [ ] Lighthouse score > 90
- [ ] React.memo on expensive components

**Technical Notes**:

**Code Splitting:**
```typescript
import { lazy, Suspense } from 'react';

const ReportsPage = lazy(() => import('./pages/manager/ReportsPage'));

// In router
{
  path: 'reports',
  element: (
    <Suspense fallback={<div>Loading...</div>}>
      <ReportsPage />
    </Suspense>
  ),
}
```

**Definition of Done**:
- [x] Bundle size reduced
- [x] Lighthouse score > 90
- [x] No performance warnings in console
- [x] Fast page loads

---

### FE-025: Accessibility Audit

**Story Points**: 5
**Priority**: High
**Dependencies**: FE-002, FE-018

**Description**:
Ensure application meets WCAG 2.1 Level AA accessibility standards.

**Acceptance Criteria**:
- [ ] Keyboard navigation working (Tab, Enter, Escape)
- [ ] Screen reader compatible (ARIA labels)
- [ ] Color contrast ratio ≥ 4.5:1
- [ ] Focus indicators visible
- [ ] No automated accessibility errors (axe DevTools)
- [ ] Form labels properly associated

**Technical Notes**:

```bash
npm install -D @axe-core/react
```

**Definition of Done**:
- [x] axe DevTools shows 0 errors
- [x] Keyboard navigation works
- [x] Screen reader tested
- [x] Color contrast meets standards

---

### FE-026: Build Docker Image

**Story Points**: 3
**Priority**: Critical
**Dependencies**: FE-024

**Description**:
Create production-ready Docker image with multi-stage build for optimized size.

**Acceptance Criteria**:
- [ ] Dockerfile with multi-stage build
- [ ] Environment variables properly injected
- [ ] NGINX serves static files
- [ ] Image size < 100MB
- [ ] Health check endpoint
- [ ] Works with docker-compose

**Technical Notes**:

**Dockerfile.prod**:
```dockerfile
# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

COPY package*.json ./
RUN npm ci

COPY . .

ARG VITE_KRAKEND_URL
ENV VITE_KRAKEND_URL=$VITE_KRAKEND_URL

RUN npm run build

# Production stage
FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=3s \
  CMD wget --quiet --tries=1 --spider http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]
```

**nginx.conf**:
```nginx
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Gzip compression
    gzip on;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2|ttf|eot)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # SPA fallback
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

**Definition of Done**:
- [x] Docker image builds successfully
- [x] Image runs in docker-compose
- [x] Static files served correctly
- [x] Environment variables working

---

### FE-027: CI/CD Pipeline Integration

**Story Points**: 3
**Priority**: High
**Dependencies**: FE-021, FE-026

**Description**:
Integrate frontend build and tests into GitHub Actions CI/CD pipeline.

**Acceptance Criteria**:
- [ ] GitHub Actions workflow for frontend
- [ ] Install dependencies
- [ ] Run linter (ESLint)
- [ ] Run type checking (TypeScript)
- [ ] Run tests with coverage
- [ ] Build production bundle
- [ ] Build Docker image
- [ ] Upload coverage to Codecov (optional)

**Technical Notes**:

**.github/workflows/frontend-ci.yml**:
```yaml
name: Frontend CI

on:
  push:
    branches: [main, develop]
    paths:
      - 'frontend/**'
  pull_request:
    branches: [main, develop]
    paths:
      - 'frontend/**'

jobs:
  frontend:
    name: Frontend Tests & Build
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
          cache-dependency-path: frontend/package-lock.json

      - name: Install dependencies
        working-directory: ./frontend
        run: npm ci

      - name: Lint code
        working-directory: ./frontend
        run: npm run lint

      - name: Type check
        working-directory: ./frontend
        run: npm run type-check

      - name: Run tests
        working-directory: ./frontend
        run: npm test -- --coverage

      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v3
        with:
          files: ./frontend/coverage/coverage-final.json
          flags: frontend

      - name: Build application
        working-directory: ./frontend
        run: npm run build
        env:
          VITE_KRAKEND_URL: ${{ secrets.VITE_KRAKEND_URL }}

      - name: Build Docker image
        uses: docker/build-push-action@v5
        with:
          context: ./frontend
          file: ./frontend/Dockerfile.prod
          push: false
          tags: timemanager-frontend:test
```

**Definition of Done**:
- [x] CI pipeline runs on PR
- [x] All checks pass
- [x] Docker image builds in CI
- [x] Coverage reported

---

## Summary & Estimation

### Total Story Points by Epic

| Epic | Story Points | Estimated Days |
|------|--------------|----------------|
| Epic 1: Setup & Infrastructure | 24 | 5 days |
| Epic 2: Authentication | 13 | 2-3 days |
| Epic 3: Employee Features | 26 | 5-6 days |
| Epic 4: Manager Features | 42 | 8-9 days |
| Epic 5: Shared Components | 18 | 4 days |
| Epic 6: Testing & Quality | 21 | 4-5 days |
| Epic 7: Optimization & Deployment | 16 | 3-4 days |
| **TOTAL** | **160** | **~30-35 days** |

### Priority Breakdown

- **Critical**: 10 tickets (FE-001 to FE-007, FE-010, FE-026)
- **High**: 11 tickets
- **Medium**: 5 tickets
- **Low**: 1 ticket

### Recommended Sprint Plan (2-week sprints)

**Sprint 1 (Setup + Auth)**:
- FE-001 to FE-009
- **Goal**: Working authentication + project foundation

**Sprint 2 (Employee Core)**:
- FE-010 to FE-013
- **Goal**: Employee can clock in/out and view dashboard

**Sprint 3 (Manager Core)**:
- FE-014, FE-015, FE-018, FE-019
- **Goal**: Manager can manage teams and view basic reports

**Sprint 4 (Manager Advanced)**:
- FE-016, FE-017, FE-020
- **Goal**: Complete manager features with reports

**Sprint 5 (Testing + Optimization)**:
- FE-021 to FE-025
- **Goal**: Tests, performance, accessibility

**Sprint 6 (Deployment + Polish)**:
- FE-026, FE-027, bug fixes, polish
- **Goal**: Production-ready application

---

## Notes

### KrakenD Gateway Specific Considerations

**API Endpoints through Gateway:**
- All API calls go through KrakenD: `http://localhost:8080/api/*`
- KrakenD handles rate limiting, authentication aggregation
- Frontend doesn't need to know backend microservices structure

**Error Handling:**
- `503 Service Unavailable`: Backend service down
- `429 Too Many Requests`: Rate limit from KrakenD
- Gateway-specific error codes in response headers

**Configuration:**
- Environment variable `VITE_KRAKEND_URL` for gateway URL
- Separate URLs for dev/staging/production

### Testing Strategy

**Unit Tests (70% coverage target):**
- Components rendering
- Form validation
- User interactions
- State management

**Integration Tests:**
- API integration (with MSW)
- Authentication flow
- CRUD operations

**E2E Tests (optional):**
- Critical user journeys
- Login → Clock in/out → View dashboard

---

**Document Status**: Frontend planning complete - Ready for implementation
**Review Frequency**: Weekly during development
**Owner**: Frontend Team
**Last Updated**: 2025-10-06
