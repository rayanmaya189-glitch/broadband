import type { RouteObject } from 'react-router-dom';
import { AdminLayout } from './components/layout/AdminLayout';
import { RequireAuth } from './components/layout/RequireAuth';
import { RequireModule } from './components/layout/RequireModule';
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
        {
          index: true,
          element: (
            <RequireModule module="dashboard">
              <DashboardPage />
            </RequireModule>
          ),
        },
        {
          path: 'customers',
          element: (
            <RequireModule module="customers">
              <CustomersPage />
            </RequireModule>
          ),
        },
        {
          path: 'plans',
          element: (
            <RequireModule module="plans">
              <PlansPage />
            </RequireModule>
          ),
        },
        {
          path: 'subscriptions',
          element: (
            <RequireModule module="subscriptions">
              <SubscriptionsPage />
            </RequireModule>
          ),
        },
        {
          path: 'billing',
          element: (
            <RequireModule module="billing">
              <BillingPage />
            </RequireModule>
          ),
        },
        {
          path: 'tickets',
          element: (
            <RequireModule module="tickets">
              <TicketsPage />
            </RequireModule>
          ),
        },
        {
          path: 'leads',
          element: (
            <RequireModule module="leads">
              <LeadsPage />
            </RequireModule>
          ),
        },
        {
          path: 'network',
          element: (
            <RequireModule module="network">
              <NetworkPage />
            </RequireModule>
          ),
        },
        {
          path: 'devices',
          element: (
            <RequireModule module="devices">
              <DevicesPage />
            </RequireModule>
          ),
        },
        {
          path: 'monitoring',
          element: (
            <RequireModule module="monitoring">
              <MonitoringPage />
            </RequireModule>
          ),
        },
        {
          path: 'installations',
          element: (
            <RequireModule module="installations">
              <InstallationsPage />
            </RequireModule>
          ),
        },
        {
          path: 'approvals',
          element: (
            <RequireModule module="approvals">
              <ApprovalsPage />
            </RequireModule>
          ),
        },
        {
          path: 'notifications',
          element: (
            <RequireModule module="notifications">
              <NotificationsPage />
            </RequireModule>
          ),
        },
        {
          path: 'users',
          element: (
            <RequireModule module="users">
              <UsersPage />
            </RequireModule>
          ),
        },
        {
          path: 'roles',
          element: (
            <RequireModule module="roles">
              <RolesPage />
            </RequireModule>
          ),
        },
        {
          path: 'branches',
          element: (
            <RequireModule module="branches">
              <BranchesPage />
            </RequireModule>
          ),
        },
        {
          path: 'audit',
          element: (
            <RequireModule module="audit">
              <AuditPage />
            </RequireModule>
          ),
        },
        {
          path: 'accounting',
          element: (
            <RequireModule module="accounting">
              <AccountingPage />
            </RequireModule>
          ),
        },
        {
          path: 'coverage',
          element: (
            <RequireModule module="coverage">
              <CoveragePage />
            </RequireModule>
          ),
        },
        {
          path: 'profile',
          element: (
            <RequireModule module="profile">
              <ProfilePage />
            </RequireModule>
          ),
        },
        { path: '*', element: <AdminNotFound /> },
      ],
    },
  ],
};
