/**
 * Trend Line Chart Component
 *
 * Displays hours trend over time as a line chart.
 * Shows both actual and expected hours with area fill.
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
import { format, parseISO } from 'date-fns';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import type { ChartDataPoint } from '../../types/kpi';

interface TrendLineChartProps {
  data: ChartDataPoint[];
  title?: string;
  description?: string;
  granularity?: 'day' | 'week' | 'month';
}

/**
 * Format date based on granularity
 */
function formatDate(dateStr: string, granularity: 'day' | 'week' | 'month'): string {
  const date = parseISO(dateStr);
  switch (granularity) {
    case 'day':
      return format(date, 'dd MMM');
    case 'week':
      return format(date, "'W'w MMM");
    case 'month':
      return format(date, 'MMM yy');
    default:
      return format(date, 'dd/MM');
  }
}

/**
 * Custom tooltip component
 */
function CustomTooltip({ active, payload, label }: {
  active?: boolean;
  payload?: Array<{ value: number; dataKey: string; color: string; stroke?: string }>;
  label?: string;
}) {
  if (!active || !payload || !payload.length) return null;

  const worked = payload.find((p) => p.dataKey === 'hours_worked');
  const expected = payload.find((p) => p.dataKey === 'theoretical_hours');
  const variance = worked && expected ? worked.value - expected.value : 0;

  return (
    <div className="bg-background border rounded-lg shadow-lg p-3">
      <p className="font-medium text-sm mb-2">{label}</p>
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

export function TrendLineChart({
  data,
  title = 'Hours Trend',
  description = 'Evolution over time',
  granularity = 'day',
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

  if (!data.length) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">{title}</CardTitle>
          <CardDescription>{description}</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="h-[300px] flex items-center justify-center text-muted-foreground">
            No data available
          </div>
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <div className="flex items-start justify-between">
          <div>
            <CardTitle className="text-lg">{title}</CardTitle>
            <CardDescription>{description}</CardDescription>
          </div>
          {summary && (
            <div className="text-right text-sm">
              <p className="font-medium">
                {summary.totalWorked.toFixed(0)}h / {summary.totalExpected.toFixed(0)}h
              </p>
              <p className={summary.variance >= 0 ? 'text-green-600' : 'text-red-600'}>
                {summary.variance >= 0 ? '+' : ''}{summary.variance.toFixed(1)}h ({summary.percentComplete.toFixed(0)}%)
              </p>
            </div>
          )}
        </div>
      </CardHeader>
      <CardContent>
        <div className="h-[300px]">
          <ResponsiveContainer width="100%" height="100%">
            <ComposedChart
              data={chartData}
              margin={{ top: 10, right: 10, left: 0, bottom: 0 }}
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
                wrapperStyle={{ paddingTop: '20px' }}
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
      </CardContent>
    </Card>
  );
}
