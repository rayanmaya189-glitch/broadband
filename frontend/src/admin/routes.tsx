import type { RouteObject } from 'react-router-dom';
import { AdminLayout } from './components/layout/AdminLayout';
import { RequireAuth } from './components/layout/RequireAuth';
import { AdminLoginPage } from './pages/Login';
import { AdminForbidden, AdminNotFound } from './pages/Errors';
import { DashboardPage } from './pages/Dashboard';
import { CustomersPage } from './pages/Customers';
import { PlansPage } from './pages/Plans';
import { SubscriptionsPage } from './pages/Subscriptions';
import { BillingPage } from './pages/Billing';
import { TicketsPage } from './pages/Tickets';
import { LeadsPage } from './pages/Leads';
import { NetworkPage } from './pages/Network';
import { DevicesPage } from './pages/Devices';
import { MonitoringPage } from './pages/Monitoring';
import { InstallationsPage } from './pages/Installations';
import { ApprovalsPage } from './pages/Approvals';
import { NotificationsPage } from './pages/Notifications';
import { UsersPage } from './pages/Users';
import { RolesPage } from './pages/Roles';
import { BranchesPage } from './pages/Branches';
import { AuditPage } from './pages/Audit';
import { AccountingPage } from './pages/Accounting';
import { CoveragePage } from './pages/Coverage';
import { ProfilePage } from './pages/Profile';

export const adminRoutes: RouteObject = {
  path: 'admin',
  children: [
    { path: 'login', element: <AdminLoginPage /> },
    { path: '403', element: <AdminForbidden /> },
    {
      element: (
        <RequireAuth>
          <AdminLayout />
        </RequireAuth>
      ),
      children: [
        { index: true, element: <DashboardPage /> },
        { path: 'customers', element: <CustomersPage /> },
        { path: 'plans', element: <PlansPage /> },
        { path: 'subscriptions', element: <SubscriptionsPage /> },
        { path: 'billing', element: <BillingPage /> },
        { path: 'tickets', element: <TicketsPage /> },
        { path: 'leads', element: <LeadsPage /> },
        { path: 'network', element: <NetworkPage /> },
        { path: 'devices', element: <DevicesPage /> },
        { path: 'monitoring', element: <MonitoringPage /> },
        { path: 'installations', element: <InstallationsPage /> },
        { path: 'approvals', element: <ApprovalsPage /> },
        { path: 'notifications', element: <NotificationsPage /> },
        { path: 'users', element: <UsersPage /> },
        { path: 'roles', element: <RolesPage /> },
        { path: 'branches', element: <BranchesPage /> },
        { path: 'audit', element: <AuditPage /> },
        { path: 'accounting', element: <AccountingPage /> },
        { path: 'coverage', element: <CoveragePage /> },
        { path: 'profile', element: <ProfilePage /> },
        { path: '*', element: <AdminNotFound /> },
      ],
    },
  ],
};
