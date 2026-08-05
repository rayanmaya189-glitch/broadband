import { Link } from 'react-router-dom';
import { ShieldAlert } from 'lucide-react';

export function AdminForbidden() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center gap-4 bg-dark-950 p-8 text-center">
      <div className="flex h-16 w-16 items-center justify-center rounded-2xl border border-red-500/30 bg-red-500/10 text-red-400">
        <ShieldAlert className="h-8 w-8" />
      </div>
      <h1 className="text-2xl font-semibold text-white">Access denied</h1>
      <p className="max-w-sm text-sm text-dark-400">
        Your role does not grant access to this area of the admin console.
      </p>
      <Link
        to="/admin"
        className="mt-2 rounded-xl bg-primary-600 px-5 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500"
      >
        Go to dashboard
      </Link>
    </div>
  );
}

export function AdminNotFound() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-center gap-4 bg-dark-950 p-8 text-center">
      <h1 className="text-6xl font-bold text-white">404</h1>
      <p className="text-sm text-dark-400">This admin page does not exist.</p>
      <Link
        to="/admin"
        className="mt-2 rounded-xl bg-primary-600 px-5 py-2.5 text-sm font-medium text-white transition-colors hover:bg-primary-500"
      >
        Go to dashboard
      </Link>
    </div>
  );
}
