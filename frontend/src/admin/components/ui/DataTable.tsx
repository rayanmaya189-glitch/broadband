import type { ReactNode } from 'react';
import { motion } from 'framer-motion';
import { EmptyState } from './EmptyState';
import { Spinner } from './Spinner';

export interface Column<T> {
  key: string;
  header: string;
  render?: (row: T) => ReactNode;
  align?: 'left' | 'right' | 'center';
  className?: string;
  headerClassName?: string;
}

interface DataTableProps<T> {
  columns: Column<T>[];
  data: T[];
  rowKey: (row: T) => string | number;
  loading?: boolean;
  emptyTitle?: string;
  emptyDescription?: string;
  onRowClick?: (row: T) => void;
  dense?: boolean;
  footer?: ReactNode;
}

export function DataTable<T>({
  columns,
  data,
  rowKey,
  loading,
  emptyTitle,
  emptyDescription,
  onRowClick,
  dense,
  footer,
}: DataTableProps<T>) {
  const alignClass = (align?: string) =>
    align === 'right' ? 'text-right' : align === 'center' ? 'text-center' : 'text-left';

  return (
    <div className="glass-card overflow-hidden rounded-2xl">
      <div className="overflow-x-auto">
        <table className="w-full min-w-[640px] border-collapse text-sm">
          <thead>
            <tr className="border-b border-white/10 bg-white/[0.02]">
              {columns.map((col) => (
                <th
                  key={col.key}
                  className={`px-4 py-3 text-xs font-semibold uppercase tracking-wider text-dark-400 ${alignClass(col.align)} ${col.headerClassName ?? ''}`}
                >
                  {col.header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {loading ? (
              <tr>
                <td colSpan={columns.length}>
                  <Spinner label="Loading…" />
                </td>
              </tr>
            ) : data.length === 0 ? (
              <tr>
                <td colSpan={columns.length}>
                  <EmptyState title={emptyTitle} description={emptyDescription} />
                </td>
              </tr>
            ) : (
              data.map((row, i) => (
                <motion.tr
                  key={rowKey(row)}
                  initial={{ opacity: 0 }}
                  animate={{ opacity: 1 }}
                  transition={{ delay: Math.min(i * 0.02, 0.3) }}
                  onClick={onRowClick ? () => onRowClick(row) : undefined}
                  className={`border-b border-white/5 last:border-0 ${onRowClick ? 'cursor-pointer hover:bg-white/[0.03]' : 'hover:bg-white/[0.02]'} transition-colors`}
                >
                  {columns.map((col) => (
                    <td key={col.key} className={`${dense ? 'px-4 py-2.5' : 'px-4 py-3.5'} text-dark-200 ${alignClass(col.align)} ${col.className ?? ''}`}>
                      {col.render ? col.render(row) : (row as Record<string, unknown>)[col.key] as ReactNode}
                    </td>
                  ))}
                </motion.tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      {footer && <div className="border-t border-white/5 px-4 py-3">{footer}</div>}
    </div>
  );
}
