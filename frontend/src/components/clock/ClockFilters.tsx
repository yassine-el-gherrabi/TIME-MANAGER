import { Button } from '../ui/button';
import { Label } from '../ui/label';
import {
  startOfWeek,
  endOfWeek,
  startOfMonth,
  endOfMonth,
  subMonths,
  format,
} from 'date-fns';
import type { ClockEntryStatus, ClockFilterState } from '../../types/clock';

export interface ClockFiltersProps {
  filters: ClockFilterState;
  onFiltersChange: (filters: Partial<ClockFilterState>) => void;
  onReset: () => void;
}

type DatePreset = 'this_week' | 'this_month' | 'last_month' | 'custom';

const getPresetFromFilters = (filters: ClockFilterState): DatePreset => {
  if (!filters.startDate && !filters.endDate) return 'custom';

  const now = new Date();
  const thisWeekStart = format(startOfWeek(now, { weekStartsOn: 1 }), 'yyyy-MM-dd');
  const thisWeekEnd = format(endOfWeek(now, { weekStartsOn: 1 }), 'yyyy-MM-dd');
  const thisMonthStart = format(startOfMonth(now), 'yyyy-MM-dd');
  const thisMonthEnd = format(endOfMonth(now), 'yyyy-MM-dd');
  const lastMonth = subMonths(now, 1);
  const lastMonthStart = format(startOfMonth(lastMonth), 'yyyy-MM-dd');
  const lastMonthEnd = format(endOfMonth(lastMonth), 'yyyy-MM-dd');

  if (filters.startDate === thisWeekStart && filters.endDate === thisWeekEnd) {
    return 'this_week';
  }
  if (filters.startDate === thisMonthStart && filters.endDate === thisMonthEnd) {
    return 'this_month';
  }
  if (filters.startDate === lastMonthStart && filters.endDate === lastMonthEnd) {
    return 'last_month';
  }

  return 'custom';
};

export function ClockFilters({ filters, onFiltersChange, onReset }: ClockFiltersProps) {
  const currentPreset = getPresetFromFilters(filters);

  // Validate that FROM date is not after TO date
  const hasDateError =
    filters.startDate &&
    filters.endDate &&
    new Date(filters.startDate) > new Date(filters.endDate);

  const handlePresetChange = (preset: DatePreset) => {
    const now = new Date();

    switch (preset) {
      case 'this_week': {
        const start = format(startOfWeek(now, { weekStartsOn: 1 }), 'yyyy-MM-dd');
        const end = format(endOfWeek(now, { weekStartsOn: 1 }), 'yyyy-MM-dd');
        onFiltersChange({ startDate: start, endDate: end });
        break;
      }
      case 'this_month': {
        const start = format(startOfMonth(now), 'yyyy-MM-dd');
        const end = format(endOfMonth(now), 'yyyy-MM-dd');
        onFiltersChange({ startDate: start, endDate: end });
        break;
      }
      case 'last_month': {
        const lastMonth = subMonths(now, 1);
        const start = format(startOfMonth(lastMonth), 'yyyy-MM-dd');
        const end = format(endOfMonth(lastMonth), 'yyyy-MM-dd');
        onFiltersChange({ startDate: start, endDate: end });
        break;
      }
      case 'custom':
        // Don't change dates, just allow custom input
        break;
    }
  };

  const handleStatusChange = (status: ClockEntryStatus | 'all') => {
    onFiltersChange({ status });
  };

  const hasActiveFilters =
    filters.startDate !== null ||
    filters.endDate !== null ||
    filters.status !== 'all';

  return (
    <div className="space-y-4">
      {/* Preset Buttons */}
      <div className="flex flex-wrap gap-2">
        <Button
          variant={currentPreset === 'this_week' ? 'default' : 'outline'}
          size="sm"
          onClick={() => handlePresetChange('this_week')}
        >
          This Week
        </Button>
        <Button
          variant={currentPreset === 'this_month' ? 'default' : 'outline'}
          size="sm"
          onClick={() => handlePresetChange('this_month')}
        >
          This Month
        </Button>
        <Button
          variant={currentPreset === 'last_month' ? 'default' : 'outline'}
          size="sm"
          onClick={() => handlePresetChange('last_month')}
        >
          Last Month
        </Button>
      </div>

      {/* Date Range and Status Filters */}
      <div className="flex flex-wrap items-end gap-4">
        <div className="space-y-1">
          <Label htmlFor="start_date" className="text-xs text-muted-foreground">
            From
          </Label>
          <input
            id="start_date"
            type="date"
            value={filters.startDate || ''}
            onChange={(e) => onFiltersChange({ startDate: e.target.value || null })}
            className={`flex h-9 w-full rounded-md border bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring ${
              hasDateError ? 'border-destructive' : 'border-input'
            }`}
          />
        </div>

        <div className="space-y-1">
          <Label htmlFor="end_date" className="text-xs text-muted-foreground">
            To
          </Label>
          <input
            id="end_date"
            type="date"
            value={filters.endDate || ''}
            onChange={(e) => onFiltersChange({ endDate: e.target.value || null })}
            className={`flex h-9 w-full rounded-md border bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring ${
              hasDateError ? 'border-destructive' : 'border-input'
            }`}
          />
        </div>

        <div className="space-y-1">
          <Label htmlFor="status" className="text-xs text-muted-foreground">
            Status
          </Label>
          <select
            id="status"
            value={filters.status}
            onChange={(e) => handleStatusChange(e.target.value as ClockEntryStatus | 'all')}
            className="flex h-9 w-full rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
          >
            <option value="all">All Statuses</option>
            <option value="pending">Pending</option>
            <option value="approved">Approved</option>
            <option value="rejected">Rejected</option>
          </select>
        </div>

        {hasActiveFilters && (
          <Button variant="ghost" size="sm" onClick={onReset}>
            Reset
          </Button>
        )}
      </div>

      {/* Date validation error */}
      {hasDateError && (
        <p className="text-sm text-destructive">
          "From" date must be before "To" date
        </p>
      )}
    </div>
  );
}
