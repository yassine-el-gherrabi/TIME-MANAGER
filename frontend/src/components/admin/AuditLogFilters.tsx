/**
 * Audit Log Filters Component
 *
 * Filter bar for audit logs with entity type, action, and date range.
 */

import type { FC } from 'react';
import { CalendarIcon, Filter, X } from 'lucide-react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../ui/select';
import { AuditAction, ENTITY_TYPE_LABELS, ACTION_LABELS } from '../../types/audit';

interface AuditLogFiltersProps {
  entityType: string;
  action: AuditAction | '';
  startDate: string;
  endDate: string;
  onEntityTypeChange: (value: string) => void;
  onActionChange: (value: AuditAction | '') => void;
  onStartDateChange: (value: string) => void;
  onEndDateChange: (value: string) => void;
  onClearFilters: () => void;
  hasActiveFilters: boolean;
}

export const AuditLogFilters: FC<AuditLogFiltersProps> = ({
  entityType,
  action,
  startDate,
  endDate,
  onEntityTypeChange,
  onActionChange,
  onStartDateChange,
  onEndDateChange,
  onClearFilters,
  hasActiveFilters,
}) => {
  return (
    <div className="mb-6 space-y-4">
      <div className="flex items-center gap-2">
        <Filter className="h-4 w-4 text-muted-foreground" />
        <span className="text-sm font-medium">Filters</span>
        {hasActiveFilters && (
          <Button
            variant="ghost"
            size="sm"
            onClick={onClearFilters}
            className="h-7 px-2 text-muted-foreground hover:text-foreground"
          >
            <X className="h-3 w-3 mr-1" />
            Clear
          </Button>
        )}
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        {/* Entity Type Filter */}
        <Select
          value={entityType}
          onValueChange={onEntityTypeChange}
        >
          <SelectTrigger>
            <SelectValue placeholder="All entity types" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">All entity types</SelectItem>
            {Object.entries(ENTITY_TYPE_LABELS).map(([value, label]) => (
              <SelectItem key={value} value={value}>
                {label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        {/* Action Filter */}
        <Select
          value={action}
          onValueChange={(v) => onActionChange(v as AuditAction | '')}
        >
          <SelectTrigger>
            <SelectValue placeholder="All actions" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">All actions</SelectItem>
            {Object.entries(ACTION_LABELS).map(([value, label]) => (
              <SelectItem key={value} value={value}>
                {label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        {/* Start Date Filter */}
        <div className="relative">
          <CalendarIcon className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            type="date"
            value={startDate}
            onChange={(e) => onStartDateChange(e.target.value)}
            placeholder="Start date"
            className="pl-10"
          />
        </div>

        {/* End Date Filter */}
        <div className="relative">
          <CalendarIcon className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            type="date"
            value={endDate}
            onChange={(e) => onEndDateChange(e.target.value)}
            placeholder="End date"
            className="pl-10"
          />
        </div>
      </div>
    </div>
  );
};
