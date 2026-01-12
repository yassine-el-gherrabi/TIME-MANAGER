/**
 * Clock History List Component
 *
 * Displays a simple list of clock entries with pagination.
 * Controlled component - receives entries as props.
 */

import { type FC } from 'react';
import { Button } from '../ui/button';
import { ClockEntryCard } from './ClockEntryCard';
import type { ClockEntryResponse } from '../../types/clock';

export interface ClockHistoryListProps {
  entries: ClockEntryResponse[];
  total: number;
  page: number;
  perPage?: number;
  isLoading?: boolean;
  onPageChange: (page: number) => void;
  className?: string;
}

export const ClockHistoryList: FC<ClockHistoryListProps> = ({
  entries,
  total,
  page,
  perPage = 10,
  isLoading,
  onPageChange,
  className,
}) => {
  const totalPages = Math.ceil(total / perPage);

  if (entries.length === 0 && !isLoading) {
    return (
      <div className={className}>
        <p className="text-center text-sm text-muted-foreground py-8">
          No clock entries found
        </p>
      </div>
    );
  }

  return (
    <div className={className}>
      <div className="space-y-3">
        {entries.map((entry) => (
          <ClockEntryCard key={entry.id} entry={entry} />
        ))}
      </div>

      {/* Pagination */}
      {totalPages > 1 && (
        <div className="flex items-center justify-between pt-4 mt-4 border-t">
          <Button
            variant="outline"
            size="sm"
            onClick={() => onPageChange(page - 1)}
            disabled={page <= 1 || isLoading}
          >
            Previous
          </Button>
          <span className="text-sm text-muted-foreground">
            Page {page} of {totalPages}
          </span>
          <Button
            variant="outline"
            size="sm"
            onClick={() => onPageChange(page + 1)}
            disabled={page >= totalPages || isLoading}
          >
            Next
          </Button>
        </div>
      )}
    </div>
  );
};
