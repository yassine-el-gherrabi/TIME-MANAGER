/**
 * Audit Log Details Sheet Component
 *
 * Side panel showing detailed audit log information including JSON diff.
 */

import type { FC } from 'react';
import { useTranslation } from 'react-i18next';
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

const formatDate = (dateString: string, locale: string): string => {
  const date = new Date(dateString);
  return new Intl.DateTimeFormat(locale === 'fr' ? 'fr-FR' : 'en-US', {
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
  const { t, i18n } = useTranslation();

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent className="overflow-y-auto sm:max-w-lg">
        <SheetHeader>
          <SheetTitle>{t('audit.details')}</SheetTitle>
          <SheetDescription>
            {t('audit.detailsDescription')}
          </SheetDescription>
        </SheetHeader>

        {log && (
          <div className="mt-6 space-y-6">
            {/* Basic Info */}
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-xs text-muted-foreground">{t('audit.action')}</label>
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
                <label className="text-xs text-muted-foreground">{t('audit.date')}</label>
                <p className="text-sm mt-1">{formatDate(log.created_at, i18n.language)}</p>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="text-xs text-muted-foreground">{t('audit.entityType')}</label>
                <p className="text-sm mt-1">
                  {ENTITY_TYPE_LABELS[log.entity_type] || log.entity_type}
                </p>
              </div>
              <div>
                <label className="text-xs text-muted-foreground">{t('audit.entityId')}</label>
                <p className="text-sm mt-1 font-mono text-xs break-all">
                  {log.entity_id}
                </p>
              </div>
            </div>

            {/* User Info */}
            <div className="border-t pt-4">
              <h3 className="text-sm font-medium mb-3">{t('audit.user')}</h3>
              {log.user ? (
                <div className="space-y-2">
                  <div>
                    <label className="text-xs text-muted-foreground">{t('common.name')}</label>
                    <p className="text-sm">
                      {log.user.first_name} {log.user.last_name}
                    </p>
                  </div>
                  <div>
                    <label className="text-xs text-muted-foreground">{t('common.email')}</label>
                    <p className="text-sm">{log.user.email}</p>
                  </div>
                </div>
              ) : (
                <p className="text-sm text-muted-foreground">{t('audit.systemAction')}</p>
              )}
            </div>

            {/* Connection Info */}
            <div className="border-t pt-4">
              <h3 className="text-sm font-medium mb-3">{t('audit.connection')}</h3>
              <div className="space-y-2">
                <div>
                  <label className="text-xs text-muted-foreground">{t('audit.ipAddress')}</label>
                  <p className="text-sm font-mono">
                    {log.ip_address || '-'}
                  </p>
                </div>
                {log.user_agent && (
                  <div>
                    <label className="text-xs text-muted-foreground">{t('audit.userAgent')}</label>
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
                <h3 className="text-sm font-medium mb-3">{t('audit.changes')}</h3>
                <JsonDisplay data={log.old_values} title={t('audit.oldValues')} />
                <JsonDisplay data={log.new_values} title={t('audit.newValues')} />
              </div>
            )}
          </div>
        )}
      </SheetContent>
    </Sheet>
  );
};
