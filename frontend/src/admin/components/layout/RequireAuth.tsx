import type { ReactNode } from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { useAdminStore } from '../../adminStore';

export function RequireAuth({ children }: { children: ReactNode }) {
  const authenticated = useAdminStore((s) => s.authenticated);
  const location = useLocation();

  if (!authenticated) {
    return <Navigate to="/admin/login" replace state={{ from: location.pathname }} />;
  }
  return <>{children}</>;
}
