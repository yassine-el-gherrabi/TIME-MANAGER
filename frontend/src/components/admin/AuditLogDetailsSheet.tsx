/**
 * Audit Log Details Sheet Component
 *
 * Side panel showing detailed audit log information including JSON diff.
 */

import type { FC } from 'react';
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from '../ui/sheet';
import { cn } from '../../lib/utils';
import {
  ACTION_LABELS,
  ACTION_COLORS,
  ACTION_COLORS_DARK,
  ENTITY_TYPE_LABELS,
} from '../../types/audit';
import type { AuditLog } from '../../types/audit';

interface AuditLogDetailsSheetProps {
  log: AuditLog | null;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

const formatDate = (dateString: string): string => {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat('fr-FR', {
    day: '2-digit',
    month: 'long',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  }).format(date);
};

const JsonDisplay: FC<{ data: Record<string, unknown> | null; title: string }> = ({
  data,
  title,
}) => {
  if (!data) return null;

  return (
    <div className="mt-4">
      <h4 className="text-sm font-medium mb-2">{title}</h4>
      <pre className="bg-muted p-3 rounded-md text-xs overflow-x-auto max-h-64 overflow-y-auto">
        {JSON.stringify(data, null, 2)}
      </pre>
    </div>
  );
};

export const AuditLogDetailsSheet: FC<AuditLogDetailsSheetProps> = ({
  log,
  open,
  onOpenChange,
}) => {
  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent className="overflow-y-auto sm:max-w-lg">
        <SheetHeader>
          <SheetTitle>Audit Log Details</SheetTitle>
          <SheetDescription>
            Detailed information about this audit log entry
          </SheetDescription>
        </SheetHeader>

        {log && (
          <div className="mt-6 space-y-6">
            {/* Basic Info */}
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-xs text-muted-foreground">Action</label>
                <div className="mt-1">
                  <span
                    className={cn(
                      'inline-flex items-center px-2 py-0.5 text-xs font-medium rounded border',
                      ACTION_COLORS[log.action],
                      ACTION_COLORS_DARK[log.action]
                    )}
                  >
                    {ACTION_LABELS[log.action]}
                  </span>
                </div>
              </div>
              <div>
                <label className="text-xs text-muted-foreground">Date</label>
                <p className="text-sm mt-1">{formatDate(log.created_at)}</p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-xs text-muted-foreground">Entity Type</label>
                <p className="text-sm mt-1">
                  {ENTITY_TYPE_LABELS[log.entity_type] || log.entity_type}
                </p>
              </div>
              <div>
                <label className="text-xs text-muted-foreground">Entity ID</label>
                <p className="text-sm mt-1 font-mono text-xs break-all">
                  {log.entity_id}
                </p>
              </div>
            </div>

            {/* User Info */}
            <div className="border-t pt-4">
              <h3 className="text-sm font-medium mb-3">User</h3>
              {log.user ? (
                <div className="space-y-2">
                  <div>
                    <label className="text-xs text-muted-foreground">Name</label>
                    <p className="text-sm">
                      {log.user.first_name} {log.user.last_name}
                    </p>
                  </div>
                  <div>
                    <label className="text-xs text-muted-foreground">Email</label>
                    <p className="text-sm">{log.user.email}</p>
                  </div>
                </div>
              ) : (
                <p className="text-sm text-muted-foreground">System action</p>
              )}
            </div>

            {/* Connection Info */}
            <div className="border-t pt-4">
              <h3 className="text-sm font-medium mb-3">Connection</h3>
              <div className="space-y-2">
                <div>
                  <label className="text-xs text-muted-foreground">IP Address</label>
                  <p className="text-sm font-mono">
                    {log.ip_address || '-'}
                  </p>
                </div>
                {log.user_agent && (
                  <div>
                    <label className="text-xs text-muted-foreground">User Agent</label>
                    <p className="text-xs text-muted-foreground break-all">
                      {log.user_agent}
                    </p>
                  </div>
                )}
              </div>
            </div>

            {/* Changes */}
            {(log.old_values || log.new_values) && (
              <div className="border-t pt-4">
                <h3 className="text-sm font-medium mb-3">Changes</h3>
                <JsonDisplay data={log.old_values} title="Old Values" />
                <JsonDisplay data={log.new_values} title="New Values" />
              </div>
            )}
          </div>
        )}
      </SheetContent>
    </Sheet>
  );
};
