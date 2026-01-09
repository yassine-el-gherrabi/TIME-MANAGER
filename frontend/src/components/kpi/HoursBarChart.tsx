/**
 * Hours Bar Chart Component
 *
 * Displays hours worked vs theoretical hours in a bar chart.
 * Includes navigation controls and granularity selector.
 */

import { useMemo } from 'react';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import { format, parseISO, startOfWeek, endOfWeek, getWeek } from 'date-fns';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import type { ChartDataPoint } from '../../types/kpi';

type Granularity = 'day' | 'week' | 'month';

interface HoursBarChartProps {
  data: ChartDataPoint[];
  title?: string;
  description?: string;
  granularity?: Granularity;
  periodLabel?: string;
  onNavigate?: (direction: 'prev' | 'next') => void;
  onGranularityChange?: (granularity: Granularity) => void;
}

/**
 * Format date based on granularity
 */
function formatDate(dateStr: string, granularity: Granularity): string {
  if (!dateStr) return '';
  try {
    const date = parseISO(dateStr);
    if (isNaN(date.getTime())) return dateStr;
    switch (granularity) {
      case 'day':
        return format(date, 'EEE dd');
      case 'week': {
        // Week starts on Monday
        const weekNum = getWeek(date, { weekStartsOn: 1 });
        const monday = startOfWeek(date, { weekStartsOn: 1 });
        const sunday = endOfWeek(date, { weekStartsOn: 1 });
        const formatShort = (d: Date) => format(d, 'dd/MM');
        return `W${weekNum} (${formatShort(monday)}-${formatShort(sunday)})`;
      }
      case 'month':
        return format(date, 'MMM');
      default:
        return format(date, 'dd/MM');
    }
  } catch {
    return dateStr;
  }
}

/**
 * Custom tooltip component
 */
function CustomTooltip({ active, payload, label }: {
  active?: boolean;
  payload?: Array<{ value: number; dataKey: string; color: string }>;
  label?: string;
}) {
  if (!active || !payload || !payload.length) return null;

  return (
    <div className="bg-background border rounded-lg shadow-lg p-3">
      <p className="font-medium text-sm mb-2">{label}</p>
      {payload.map((entry, index) => (
        <p key={index} className="text-sm" style={{ color: entry.color }}>
          {entry.dataKey === 'hours_worked' ? 'Worked' : 'Expected'}:{' '}
          <span className="font-medium">{entry.value.toFixed(1)}h</span>
        </p>
      ))}
    </div>
  );
}

/**
 * Mode selector buttons
 */
function GranularitySelector({
  value,
  onChange,
}: {
  value: Granularity;
  onChange?: (g: Granularity) => void;
}) {
  const options: Granularity[] = ['day', 'week', 'month'];
  const labels = { day: 'Day', week: 'Week', month: 'Month' };

  return (
    <div className="flex rounded-md border">
      {options.map((option) => (
        <Button
          key={option}
          variant={value === option ? 'secondary' : 'ghost'}
          size="sm"
          className={`h-7 px-2 text-xs rounded-none first:rounded-l-md last:rounded-r-md ${
            value === option ? 'bg-muted' : ''
          }`}
          onClick={() => onChange?.(option)}
        >
          {labels[option]}
        </Button>
      ))}
    </div>
  );
}

export function HoursBarChart({
  data,
  title = 'Hours Worked',
  description,
  granularity = 'day',
  periodLabel,
  onNavigate,
  onGranularityChange,
}: HoursBarChartProps) {
  // Transform data for chart
  const chartData = useMemo(() => {
    return data.map((point) => ({
      ...point,
      name: formatDate(point.date, granularity),
    }));
  }, [data, granularity]);

  // Calculate summary
  const summary = useMemo(() => {
    if (!data.length) return null;
    const totalWorked = data.reduce((sum, p) => sum + p.hours_worked, 0);
    const totalExpected = data.reduce((sum, p) => sum + p.theoretical_hours, 0);
    return { totalWorked, totalExpected };
  }, [data]);

  const hasControls = onNavigate || onGranularityChange;

  return (
    <Card>
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between gap-2">
          <div className="min-w-0">
            <CardTitle className="text-lg">{title}</CardTitle>
            <CardDescription>
              {periodLabel || description || 'Daily hours vs expected'}
            </CardDescription>
          </div>
          {hasControls && (
            <div className="flex items-center gap-2 flex-shrink-0">
              {onGranularityChange && (
                <GranularitySelector
                  value={granularity}
                  onChange={onGranularityChange}
                />
              )}
              {onNavigate && (
                <div className="flex">
                  <Button
                    variant="outline"
                    size="icon"
                    className="h-7 w-7 rounded-r-none"
                    onClick={() => onNavigate('prev')}
                  >
                    <ChevronLeft className="h-4 w-4" />
                  </Button>
                  <Button
                    variant="outline"
                    size="icon"
                    className="h-7 w-7 rounded-l-none border-l-0"
                    onClick={() => onNavigate('next')}
                  >
                    <ChevronRight className="h-4 w-4" />
                  </Button>
                </div>
              )}
            </div>
          )}
        </div>
        {summary && (
          <p className="text-sm text-muted-foreground mt-1">
            Total: <span className="font-medium">{summary.totalWorked.toFixed(1)}h</span>
            {' / '}
            <span className="text-muted-foreground">{summary.totalExpected.toFixed(1)}h expected</span>
          </p>
        )}
      </CardHeader>
      <CardContent>
        {!data.length ? (
          <div className="h-[280px] flex items-center justify-center text-muted-foreground">
            No data available
          </div>
        ) : (
          <div className="h-[280px]">
            <ResponsiveContainer width="100%" height="100%">
              <BarChart
                data={chartData}
                margin={{ top: 10, right: 10, left: 0, bottom: 0 }}
              >
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="name"
                  tick={{ fontSize: 12 }}
                  tickLine={false}
                  axisLine={false}
                  className="text-muted-foreground"
                />
                <YAxis
                  tick={{ fontSize: 12 }}
                  tickLine={false}
                  axisLine={false}
                  tickFormatter={(value) => `${value}h`}
                  className="text-muted-foreground"
                />
                <Tooltip content={<CustomTooltip />} />
                <Legend
                  wrapperStyle={{ paddingTop: '10px' }}
                  formatter={(value) =>
                    value === 'hours_worked' ? 'Worked' : 'Expected'
                  }
                />
                <Bar
                  dataKey="hours_worked"
                  fill="hsl(var(--primary))"
                  radius={[4, 4, 0, 0]}
                  name="hours_worked"
                />
                <Bar
                  dataKey="theoretical_hours"
                  fill="hsl(var(--muted-foreground) / 0.3)"
                  radius={[4, 4, 0, 0]}
                  name="theoretical_hours"
                />
              </BarChart>
            </ResponsiveContainer>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
