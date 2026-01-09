/**
 * Audit Logs Table Component
 *
 * Table displaying audit log entries with clickable rows.
 */

import type { FC } from 'react';
import { Loader2 } from 'lucide-react';
import { cn } from '../../lib/utils';
import {
  AuditAction,
  ACTION_LABELS,
  ACTION_COLORS,
  ACTION_COLORS_DARK,
  ENTITY_TYPE_LABELS,
} from '../../types/audit';
import type { AuditLog } from '../../types/audit';

interface AuditLogsTableProps {
  logs: AuditLog[];
  isLoading: boolean;
  onRowClick: (log: AuditLog) => void;
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat('fr-FR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date);
};

const ActionBadge: FC<{ action: AuditAction }> = ({ action }) => {
  return (
    <span
      className={cn(
        'inline-flex items-center px-2 py-0.5 text-xs font-medium rounded border',
        ACTION_COLORS[action],
        ACTION_COLORS_DARK[action]
      )}
    >
      {ACTION_LABELS[action]}
    </span>
  );
};

export const AuditLogsTable: FC<AuditLogsTableProps> = ({
  logs,
  isLoading,
  onRowClick,
}) => {
  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
      </div>
    );
  }

  if (logs.length === 0) {
    return (
      <div className="py-12 text-center text-muted-foreground">
        No audit logs found
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr className="border-b">
            <th className="text-left py-3 px-4 text-sm font-medium text-muted-foreground">
              Date
            </th>
            <th className="text-left py-3 px-4 text-sm font-medium text-muted-foreground">
              User
            </th>
            <th className="text-left py-3 px-4 text-sm font-medium text-muted-foreground">
              Action
            </th>
            <th className="text-left py-3 px-4 text-sm font-medium text-muted-foreground">
              Entity
            </th>
            <th className="text-left py-3 px-4 text-sm font-medium text-muted-foreground">
              IP
            </th>
          </tr>
        </thead>
        <tbody>
          {logs.map((log) => (
            <tr
              key={log.id}
              onClick={() => onRowClick(log)}
              className="border-b cursor-pointer hover:bg-muted/50 transition-colors"
            >
              <td className="py-3 px-4">
                <span className="text-sm">{formatDate(log.created_at)}</span>
              </td>
              <td className="py-3 px-4">
                {log.user ? (
                  <div>
                    <div className="text-sm font-medium">
                      {log.user.first_name} {log.user.last_name}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {log.user.email}
                    </div>
                  </div>
                ) : (
                  <span className="text-sm text-muted-foreground">System</span>
                )}
              </td>
              <td className="py-3 px-4">
                <ActionBadge action={log.action} />
              </td>
              <td className="py-3 px-4">
                <div>
                  <div className="text-sm">
                    {ENTITY_TYPE_LABELS[log.entity_type] || log.entity_type}
                  </div>
                  <div className="text-xs text-muted-foreground font-mono truncate max-w-[200px]">
                    {log.entity_id}
                  </div>
                </div>
              </td>
              <td className="py-3 px-4">
                <span className="text-sm text-muted-foreground">
                  {log.ip_address || '-'}
                </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};
