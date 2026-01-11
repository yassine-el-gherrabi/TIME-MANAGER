/**
 * Trend Line Chart Component
 *
 * Displays hours trend over time as a line chart.
 * Shows both actual and expected hours with area fill.
 * Includes navigation controls and granularity selector.
 */

import { useMemo } from 'react';
import {
  Line,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  Area,
  ComposedChart,
} from 'recharts';
import { format, parseISO, startOfWeek, endOfWeek, getWeek } from 'date-fns';
import { ChevronLeft, ChevronRight } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import { Button } from '../ui/button';
import type { ChartDataPoint } from '../../types/kpi';

type Granularity = 'day' | 'week' | 'month';

interface TrendLineChartProps {
  data: ChartDataPoint[];
  title?: string;
  description?: string;
  granularity?: Granularity;
  periodLabel?: string;
  onNavigate?: (direction: 'prev' | 'next') => void;
  onGranularityChange?: (granularity: Granularity) => void;
}

/**
 * Format date label for chart display
 */
interface FormattedLabel {
  line1: string;
  line2?: string;
}

function formatDate(dateStr: string, granularity: Granularity): FormattedLabel {
  if (!dateStr) return { line1: '' };
  try {
    const date = parseISO(dateStr);
    if (isNaN(date.getTime())) return { line1: dateStr };
    switch (granularity) {
      case 'day':
        return { line1: format(date, 'dd MMM') };
      case 'week': {
        // Week starts on Monday - split into two lines
        const weekNum = getWeek(date, { weekStartsOn: 1 });
        const monday = startOfWeek(date, { weekStartsOn: 1 });
        const sunday = endOfWeek(date, { weekStartsOn: 1 });
        const formatShort = (d: Date) => format(d, 'dd/MM');
        return {
          line1: `W${weekNum}`,
          line2: `${formatShort(monday)}-${formatShort(sunday)}`,
        };
      }
      case 'month':
        return { line1: format(date, 'MMM yy') };
      default:
        return { line1: format(date, 'dd/MM') };
    }
  } catch {
    return { line1: dateStr };
  }
}

/**
 * Custom XAxis tick component for multi-line labels
 */
function CustomXAxisTick({ x, y, payload }: { x?: number; y?: number; payload?: { value: FormattedLabel } }) {
  if (!payload?.value) return null;
  const { line1, line2 } = payload.value;

  return (
    <g transform={`translate(${x},${y})`}>
      <text
        x={0}
        y={0}
        dy={12}
        textAnchor="middle"
        className="fill-muted-foreground"
        style={{ fontSize: 12 }}
      >
        {line1}
      </text>
      {line2 && (
        <text
          x={0}
          y={0}
          dy={26}
          textAnchor="middle"
          className="fill-muted-foreground"
          style={{ fontSize: 10 }}
        >
          {line2}
        </text>
      )}
    </g>
  );
}

/**
 * Custom tooltip component
 */
function CustomTooltip({ active, payload, label }: {
  active?: boolean;
  payload?: Array<{ value: number; dataKey: string; color: string; stroke?: string }>;
  label?: FormattedLabel;
}) {
  if (!active || !payload || !payload.length) return null;

  const labelStr = typeof label === 'object' && label
    ? label.line2 ? `${label.line1} (${label.line2})` : label.line1
    : String(label);

  const worked = payload.find((p) => p.dataKey === 'hours_worked');
  const expected = payload.find((p) => p.dataKey === 'theoretical_hours');
  const variance = worked && expected ? worked.value - expected.value : 0;

  return (
    <div className="bg-background border rounded-lg shadow-lg p-3">
      <p className="font-medium text-sm mb-2">{labelStr}</p>
      {worked && (
        <p className="text-sm text-primary">
          Worked: <span className="font-medium">{worked.value.toFixed(1)}h</span>
        </p>
      )}
      {expected && (
        <p className="text-sm text-muted-foreground">
          Expected: <span className="font-medium">{expected.value.toFixed(1)}h</span>
        </p>
      )}
      <p className={`text-sm mt-1 ${variance >= 0 ? 'text-green-600' : 'text-red-600'}`}>
        Variance: <span className="font-medium">{variance >= 0 ? '+' : ''}{variance.toFixed(1)}h</span>
      </p>
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

export function TrendLineChart({
  data,
  title = 'Hours Trend',
  description,
  granularity = 'day',
  periodLabel,
  onNavigate,
  onGranularityChange,
}: TrendLineChartProps) {
  // Transform data for chart
  const chartData = useMemo(() => {
    return data.map((point) => ({
      ...point,
      name: formatDate(point.date, granularity),
    }));
  }, [data, granularity]);

  // Calculate summary stats
  const summary = useMemo(() => {
    if (!data.length) return null;

    const totalWorked = data.reduce((sum, p) => sum + p.hours_worked, 0);
    const totalExpected = data.reduce((sum, p) => sum + p.theoretical_hours, 0);
    const variance = totalWorked - totalExpected;
    const percentComplete = totalExpected > 0 ? (totalWorked / totalExpected) * 100 : 0;

    return { totalWorked, totalExpected, variance, percentComplete };
  }, [data]);

  const hasControls = onNavigate || onGranularityChange;

  return (
    <Card>
      <CardHeader className="pb-2">
        <div className="flex items-start justify-between gap-2">
          <div className="min-w-0">
            <CardTitle className="text-lg">{title}</CardTitle>
            <CardDescription>
              {periodLabel || description || 'Evolution over time'}
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
          <div className="text-sm mt-1">
            <span className="text-muted-foreground">
              {summary.totalWorked.toFixed(0)}h / {summary.totalExpected.toFixed(0)}h
            </span>
            <span className={`ml-2 ${summary.variance >= 0 ? 'text-green-600' : 'text-red-600'}`}>
              ({summary.variance >= 0 ? '+' : ''}{summary.variance.toFixed(1)}h, {summary.percentComplete.toFixed(0)}%)
            </span>
          </div>
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
              <ComposedChart
                data={chartData}
                margin={{ top: 10, right: 10, left: 0, bottom: granularity === 'week' ? 20 : 0 }}
              >
                <defs>
                  <linearGradient id="colorWorked" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="hsl(var(--primary))" stopOpacity={0.3} />
                    <stop offset="95%" stopColor="hsl(var(--primary))" stopOpacity={0} />
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" className="stroke-muted" />
                <XAxis
                  dataKey="name"
                  tick={<CustomXAxisTick />}
                  tickLine={false}
                  axisLine={false}
                  height={granularity === 'week' ? 45 : 30}
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
                <Area
                  type="monotone"
                  dataKey="hours_worked"
                  stroke="hsl(var(--primary))"
                  fill="url(#colorWorked)"
                  name="hours_worked"
                  legendType="none"
                />
                <Line
                  type="monotone"
                  dataKey="theoretical_hours"
                  stroke="hsl(var(--muted-foreground))"
                  strokeDasharray="5 5"
                  dot={false}
                  name="theoretical_hours"
                />
                <Line
                  type="monotone"
                  dataKey="hours_worked"
                  stroke="hsl(var(--primary))"
                  strokeWidth={2}
                  dot={{ fill: 'hsl(var(--primary))', strokeWidth: 2, r: 4 }}
                  activeDot={{ r: 6 }}
                  name="hours_worked"
                />
              </ComposedChart>
            </ResponsiveContainer>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
