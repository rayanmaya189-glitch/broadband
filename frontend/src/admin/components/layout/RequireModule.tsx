import type { ReactNode } from 'react';
import { Navigate, useLocation } from 'react-router-dom';
import { useAdminStore } from '../../adminStore';
import { roleHasModule, type ModuleKey } from '../../lib/permissions';

/**
 * Route guard that enforces role → module access.
 *
 * The sidebar already hides modules the user's role cannot open, but that is
 * cosmetic. This guard hard-fails at the route level, so deep-linking to an
 * unauthorized module redirects to the admin 403 page instead of rendering it.
 */
export function RequireModule({ module, children }: { module: ModuleKey; children: ReactNode }) {
  const role = useAdminStore((s) => s.role);
  const location = useLocation();

  if (!roleHasModule(role, module)) {
    return <Navigate to="/admin/403" replace state={{ from: location.pathname }} />;
  }
  return <>{children}</>;
}
