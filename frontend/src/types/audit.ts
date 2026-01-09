/**
 * Audit Log Types
 *
 * TypeScript type definitions for the audit logging system.
 */

/**
 * Audit action enum
 */
export enum AuditAction {
  Create = 'create',
  Update = 'update',
  Delete = 'delete',
}

/**
 * User info in audit log
 */
export interface AuditUserInfo {
  id: string;
  email: string;
  first_name: string;
  last_name: string;
}

/**
 * Audit log entry
 */
export interface AuditLog {
  id: string;
  action: AuditAction;
  entity_type: string;
  entity_id: string;
  old_values: Record<string, unknown> | null;
  new_values: Record<string, unknown> | null;
  ip_address: string | null;
  user_agent: string | null;
  created_at: string;
  user: AuditUserInfo | null;
}

/**
 * Paginated audit logs response
 */
export interface PaginatedAuditLogs {
  data: AuditLog[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

/**
 * Audit log filter params
 */
export interface AuditLogFilter {
  [key: string]: string | number | AuditAction | undefined;
  page?: number;
  per_page?: number;
  entity_type?: string;
  action?: AuditAction;
  user_id?: string;
  entity_id?: string;
  start_date?: string;
  end_date?: string;
}

/**
 * Entity type labels (French)
 */
export const ENTITY_TYPE_LABELS: Record<string, string> = {
  users: 'Utilisateurs',
  teams: 'Equipes',
  absences: 'Absences',
  absence_types: 'Types d\'absence',
  clock_entries: 'Pointages',
  work_schedules: 'Horaires',
  closed_days: 'Jours fermés',
  leave_balances: 'Soldes de congés',
};

/**
 * Action labels (French)
 */
export const ACTION_LABELS: Record<AuditAction, string> = {
  [AuditAction.Create]: 'Création',
  [AuditAction.Update]: 'Modification',
  [AuditAction.Delete]: 'Suppression',
};

/**
 * Action badge colors
 */
export const ACTION_COLORS: Record<AuditAction, string> = {
  [AuditAction.Create]: 'bg-green-100 text-green-800 border-green-200',
  [AuditAction.Update]: 'bg-blue-100 text-blue-800 border-blue-200',
  [AuditAction.Delete]: 'bg-red-100 text-red-800 border-red-200',
};

/**
 * Action badge colors (dark mode)
 */
export const ACTION_COLORS_DARK: Record<AuditAction, string> = {
  [AuditAction.Create]: 'dark:bg-green-900/30 dark:text-green-400 dark:border-green-800',
  [AuditAction.Update]: 'dark:bg-blue-900/30 dark:text-blue-400 dark:border-blue-800',
  [AuditAction.Delete]: 'dark:bg-red-900/30 dark:text-red-400 dark:border-red-800',
};
