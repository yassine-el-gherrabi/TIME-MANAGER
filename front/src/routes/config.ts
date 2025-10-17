import { lazy } from 'react';
import type { Role } from '@/types';

// Lazy load pages for better performance
const LoginPage = lazy(() =>
  import('@/pages/auth/LoginPage').then((module) => ({ default: module.LoginPage }))
);
const EmployeeDashboardPage = lazy(() =>
  import('@/pages/employee/DashboardPage').then((module) => ({
    default: module.EmployeeDashboardPage,
  }))
);
const ManagerDashboardPage = lazy(() =>
  import('@/pages/manager/DashboardPage').then((module) => ({
    default: module.ManagerDashboardPage,
  }))
);

export interface RouteConfig {
  path: string;
  element: React.ComponentType;
  allowedRoles?: Role[];
  isPublic?: boolean;
  title?: string;
}

// Route paths as constants to avoid typos
export const ROUTE_PATHS = {
  ROOT: '/',
  LOGIN: '/login',
  EMPLOYEE_DASHBOARD: '/employee/dashboard',
  MANAGER_DASHBOARD: '/manager/dashboard',
} as const;

// Centralized route configuration
export const ROUTES: RouteConfig[] = [
  // Public routes
  {
    path: ROUTE_PATHS.LOGIN,
    element: LoginPage,
    isPublic: true,
    title: 'Login',
  },

  // Employee routes
  {
    path: ROUTE_PATHS.EMPLOYEE_DASHBOARD,
    element: EmployeeDashboardPage,
    allowedRoles: ['employee'],
    title: 'Employee Dashboard',
  },

  // Manager routes
  {
    path: ROUTE_PATHS.MANAGER_DASHBOARD,
    element: ManagerDashboardPage,
    allowedRoles: ['manager'],
    title: 'Manager Dashboard',
  },
];

// Helper to get dashboard path by role
export const getDashboardPath = (role: Role): string => {
  return role === 'manager' ? ROUTE_PATHS.MANAGER_DASHBOARD : ROUTE_PATHS.EMPLOYEE_DASHBOARD;
};

// Helper to find route by path
export const findRoute = (path: string): RouteConfig | undefined => {
  return ROUTES.find((route) => route.path === path);
};
