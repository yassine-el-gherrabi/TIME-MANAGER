/**
 * Clock History Component
 *
 * Displays paginated list of clock entries.
 */

import { useEffect, type FC } from 'react';
import { Loader2, History } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import { ClockEntryCard } from './ClockEntryCard';
import { useClockStore } from '../../stores/clockStore';

interface ClockHistoryProps {
  className?: string;
}

export const ClockHistory: FC<ClockHistoryProps> = ({ className }) => {
  const {
    history,
    historyTotal,
    historyPage,
    isLoading,
    error,
    fetchHistory,
    setHistoryPage,
  } = useClockStore();

  useEffect(() => {
    fetchHistory({ page: 1, per_page: 10 });
  }, [fetchHistory]);

  const totalPages = Math.ceil(historyTotal / 10);

  const handlePageChange = (newPage: number) => {
    setHistoryPage(newPage);
    fetchHistory({ page: newPage, per_page: 10 });
  };

  if (isLoading && (!history || history.length === 0)) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <History className="h-5 w-5" />
            Clock History
          </CardTitle>
        </CardHeader>
        <CardContent className="flex items-center justify-center py-8">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </CardContent>
      </Card>
    );
  }

  if (error) {
    return (
      <Card className={className}>
        <CardHeader>
          <CardTitle className="flex items-center gap-2 text-lg">
            <History className="h-5 w-5" />
            Clock History
          </CardTitle>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-destructive">{error}</p>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card className={className}>
      <CardHeader>
        <CardTitle className="flex items-center justify-between text-lg">
          <span className="flex items-center gap-2">
            <History className="h-5 w-5" />
            Clock History
          </span>
          <span className="text-sm font-normal text-muted-foreground">
            {historyTotal} entries
          </span>
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-4">
        {!history || history.length === 0 ? (
          <p className="text-center text-sm text-muted-foreground py-4">
            No clock entries yet
          </p>
        ) : (
          <>
            <div className="space-y-3">
              {history.map((entry) => (
                <ClockEntryCard key={entry.id} entry={entry} />
              ))}
            </div>

            {/* Pagination */}
            {totalPages > 1 && (
              <div className="flex items-center justify-between pt-4">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handlePageChange(historyPage - 1)}
                  disabled={historyPage <= 1 || isLoading}
                >
                  Previous
                </Button>
                <span className="text-sm text-muted-foreground">
                  Page {historyPage} of {totalPages}
                </span>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => handlePageChange(historyPage + 1)}
                  disabled={historyPage >= totalPages || isLoading}
                >
                  Next
                </Button>
              </div>
            )}
          </>
        )}
      </CardContent>
    </Card>
  );
};
