import { useState, useEffect } from 'react';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { Button } from '../ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '../ui/card';
import { Badge } from '../ui/badge';
import { ClockEntryCard } from './ClockEntryCard';
import {
  startOfMonth,
  endOfMonth,
  startOfWeek,
  endOfWeek,
  eachDayOfInterval,
  isSameMonth,
  isSameDay,
  isBefore,
  isAfter,
  startOfDay,
  format,
  addMonths,
  subMonths,
  parseISO,
} from 'date-fns';
import type { ClockEntryResponse } from '../../types/clock';
import { cn } from '../../lib/utils';

export interface ClockCalendarViewProps {
  entries: ClockEntryResponse[];
  isLoading?: boolean;
  filterStartDate?: string | null;
  filterEndDate?: string | null;
  filterStatus?: string;
}

const WEEKDAYS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

export function ClockCalendarView({
  entries,
  isLoading,
  filterStartDate,
  filterEndDate,
  filterStatus,
}: ClockCalendarViewProps) {
  const [currentMonth, setCurrentMonth] = useState(new Date());
  const [selectedDate, setSelectedDate] = useState<Date | null>(null);

  // Sync month when start date filter changes
  useEffect(() => {
    if (filterStartDate) {
      setCurrentMonth(startOfMonth(parseISO(filterStartDate)));
    }
  }, [filterStartDate]);

  // Check if any filters are active
  const hasActiveFilters = filterStartDate || filterEndDate || (filterStatus && filterStatus !== 'all');

  // Generate calendar days
  const monthStart = startOfMonth(currentMonth);
  const monthEnd = endOfMonth(currentMonth);
  const calendarStart = startOfWeek(monthStart, { weekStartsOn: 1 });
  const calendarEnd = endOfWeek(monthEnd, { weekStartsOn: 1 });
  const calendarDays = eachDayOfInterval({ start: calendarStart, end: calendarEnd });

  // Group entries by date
  const entriesByDate = entries.reduce((acc, entry) => {
    const dateKey = entry.clock_in.split('T')[0];
    if (!acc[dateKey]) {
      acc[dateKey] = [];
    }
    acc[dateKey].push(entry);
    return acc;
  }, {} as Record<string, ClockEntryResponse[]>);

  // Get entries for selected date
  const selectedDateEntries = selectedDate
    ? entriesByDate[format(selectedDate, 'yyyy-MM-dd')] || []
    : [];

  // Get dominant status for a day
  const getDayStatus = (day: Date): 'approved' | 'pending' | 'rejected' | null => {
    const dateKey = format(day, 'yyyy-MM-dd');
    const dayEntries = entriesByDate[dateKey];
    if (!dayEntries || dayEntries.length === 0) return null;

    const statuses = dayEntries.map((e) => e.status);
    if (statuses.includes('pending')) return 'pending';
    if (statuses.includes('rejected')) return 'rejected';
    return 'approved';
  };

  // Get total hours for a day
  const getDayHours = (day: Date): number => {
    const dateKey = format(day, 'yyyy-MM-dd');
    const dayEntries = entriesByDate[dateKey];
    if (!dayEntries) return 0;
    return dayEntries.reduce((sum, e) => sum + (e.duration_minutes || 0), 0) / 60;
  };

  // Check if a day is within the filter date range
  const isDayInFilterRange = (day: Date): boolean => {
    // If no date filters, all days are in range
    if (!filterStartDate && !filterEndDate) return true;

    const dayStart = startOfDay(day);

    if (filterStartDate && filterEndDate) {
      const start = startOfDay(parseISO(filterStartDate));
      const end = startOfDay(parseISO(filterEndDate));
      return !isBefore(dayStart, start) && !isAfter(dayStart, end);
    }

    if (filterStartDate) {
      const start = startOfDay(parseISO(filterStartDate));
      return !isBefore(dayStart, start);
    }

    if (filterEndDate) {
      const end = startOfDay(parseISO(filterEndDate));
      return !isAfter(dayStart, end);
    }

    return true;
  };

  const handlePrevMonth = () => setCurrentMonth(subMonths(currentMonth, 1));
  const handleNextMonth = () => setCurrentMonth(addMonths(currentMonth, 1));

  const handleDayClick = (day: Date) => {
    if (isSameDay(day, selectedDate || new Date(0))) {
      setSelectedDate(null);
    } else {
      setSelectedDate(day);
    }
  };

  if (isLoading) {
    return (
      <Card>
        <CardContent className="pt-6">
          <div className="animate-pulse space-y-4">
            <div className="h-8 w-48 bg-muted rounded mx-auto" />
            <div className="grid grid-cols-7 gap-1">
              {[...Array(35)].map((_, i) => (
                <div key={i} className="h-16 bg-muted rounded" />
              ))}
            </div>
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <div className="space-y-4">
      <Card>
        <CardHeader className="pb-2">
          <div className="flex items-center justify-between">
            <Button variant="outline" size="icon" onClick={handlePrevMonth}>
              <ChevronLeft className="h-4 w-4" />
            </Button>
            <CardTitle className="flex items-center gap-2">
              {format(currentMonth, 'MMMM yyyy')}
              {hasActiveFilters && (
                <Badge variant="secondary" className="text-xs font-normal">
                  Filtered
                </Badge>
              )}
            </CardTitle>
            <Button variant="outline" size="icon" onClick={handleNextMonth}>
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {/* Weekday headers */}
          <div className="grid grid-cols-7 gap-1 mb-2">
            {WEEKDAYS.map((day) => (
              <div
                key={day}
                className="text-center text-xs font-medium text-muted-foreground py-2"
              >
                {day}
              </div>
            ))}
          </div>

          {/* Calendar grid */}
          <div className="grid grid-cols-7 gap-1">
            {calendarDays.map((day) => {
              const isCurrentMonth = isSameMonth(day, currentMonth);
              const isSelected = selectedDate && isSameDay(day, selectedDate);
              const isToday = isSameDay(day, new Date());
              const status = getDayStatus(day);
              const hours = getDayHours(day);
              const hasEntries = status !== null;
              const isInFilterRange = isDayInFilterRange(day);
              const isOutsideFilter = (filterStartDate || filterEndDate) && !isInFilterRange;

              return (
                <button
                  key={day.toISOString()}
                  onClick={() => handleDayClick(day)}
                  disabled={!isCurrentMonth}
                  className={cn(
                    'relative min-h-[4rem] p-1 rounded-md text-left transition-colors',
                    'hover:bg-muted/50 focus:outline-none focus-visible:ring-2 focus-visible:ring-ring',
                    !isCurrentMonth && 'opacity-30 cursor-not-allowed',
                    // Days outside filter range - clearly greyed out
                    isOutsideFilter && isCurrentMonth && 'bg-muted text-muted-foreground/50',
                    // Selected day
                    isSelected && 'bg-accent'
                  )}
                >
                  {/* Today indicator - small dot */}
                  {isToday && isCurrentMonth && (
                    <div className="absolute top-1 left-1 w-1.5 h-1.5 rounded-full bg-primary" />
                  )}
                  <span
                    className={cn(
                      'text-sm font-medium',
                      isToday && 'text-primary font-bold',
                      isOutsideFilter && 'text-muted-foreground/50'
                    )}
                  >
                    {format(day, 'd')}
                  </span>

                  {hasEntries && isCurrentMonth && (
                    <div className="mt-1">
                      <div
                        className={cn(
                          'text-xs font-medium rounded px-1',
                          status === 'approved' && 'bg-green-100 text-green-700',
                          status === 'pending' && 'bg-yellow-100 text-yellow-700',
                          status === 'rejected' && 'bg-red-100 text-red-700'
                        )}
                      >
                        {hours.toFixed(1)}h
                      </div>
                    </div>
                  )}

                  {/* Status indicator dot */}
                  {hasEntries && isCurrentMonth && (
                    <div
                      className={cn(
                        'absolute top-1 right-1 w-2 h-2 rounded-full',
                        status === 'approved' && 'bg-green-500',
                        status === 'pending' && 'bg-yellow-500',
                        status === 'rejected' && 'bg-red-500'
                      )}
                    />
                  )}
                </button>
              );
            })}
          </div>
        </CardContent>
      </Card>

      {/* Selected day entries */}
      {selectedDate && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">
              {format(selectedDate, 'EEEE, MMMM d, yyyy')}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {selectedDateEntries.length === 0 ? (
              <p className="text-sm text-muted-foreground text-center py-4">
                No entries for this day
              </p>
            ) : (
              <div className="space-y-3">
                {selectedDateEntries.map((entry) => (
                  <ClockEntryCard key={entry.id} entry={entry} />
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      )}
    </div>
  );
}
