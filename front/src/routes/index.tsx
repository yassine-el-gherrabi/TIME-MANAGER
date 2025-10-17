import { Suspense } from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { ProtectedRoute } from './ProtectedRoute';
import { ROUTES, ROUTE_PATHS, getDashboardPath } from './config';
import { useAuth } from '@/hooks/useAuth';

// Loading fallback component
const LoadingFallback = () => (
  <div className="flex min-h-screen items-center justify-center">
    <div className="h-12 w-12 animate-spin rounded-full border-b-2 border-t-2 border-primary"></div>
  </div>
);

// Root redirect component
const RootRedirect = () => {
  const { user } = useAuth();

  if (!user) {
    return <Navigate to={ROUTE_PATHS.LOGIN} replace />;
  }

  return <Navigate to={getDashboardPath(user.role)} replace />;
};

export const AppRoutes = () => {
  return (
    <BrowserRouter>
      <Suspense fallback={<LoadingFallback />}>
        <Routes>
          {/* Root redirect */}
          <Route path={ROUTE_PATHS.ROOT} element={<RootRedirect />} />

          {/* Dynamic routes from configuration */}
          {ROUTES.map((route) => {
            const Element = route.element;

            // Public routes (no authentication required)
            if (route.isPublic) {
              return <Route key={route.path} path={route.path} element={<Element />} />;
            }

            // Protected routes (authentication + optional role-based access)
            return (
              <Route
                key={route.path}
                path={route.path}
                element={
                  <ProtectedRoute allowedRoles={route.allowedRoles}>
                    <Element />
                  </ProtectedRoute>
                }
              />
            );
          })}

          {/* Catch all - redirect to root */}
          <Route path="*" element={<Navigate to={ROUTE_PATHS.ROOT} replace />} />
        </Routes>
      </Suspense>
    </BrowserRouter>
  );
};
