/**
 * Hours Bar Chart Component
 *
 * Displays hours worked vs theoretical hours in a bar chart.
 * Uses Recharts for visualization.
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
import { format, parseISO } from 'date-fns';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import type { ChartDataPoint } from '../../types/kpi';

interface HoursBarChartProps {
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
      return format(date, 'EEE dd');
    case 'week':
      return format(date, "'W'w");
    case 'month':
      return format(date, 'MMM');
    default:
      return format(date, 'dd/MM');
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

export function HoursBarChart({
  data,
  title = 'Hours Worked',
  description = 'Daily hours vs expected',
  granularity = 'day',
}: HoursBarChartProps) {
  // Transform data for chart
  const chartData = useMemo(() => {
    return data.map((point) => ({
      ...point,
      name: formatDate(point.date, granularity),
    }));
  }, [data, granularity]);

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
        <CardTitle className="text-lg">{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="h-[300px]">
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
                wrapperStyle={{ paddingTop: '20px' }}
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
      </CardContent>
    </Card>
  );
}
