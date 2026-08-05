import { Loader2 } from 'lucide-react';

export function Spinner({ label = 'Loading…' }: { label?: string }) {
  return (
    <div className="flex flex-col items-center justify-center gap-3 py-16 text-dark-400">
      <Loader2 className="h-7 w-7 animate-spin text-accent-400" />
      <p className="text-sm">{label}</p>
    </div>
  );
}
