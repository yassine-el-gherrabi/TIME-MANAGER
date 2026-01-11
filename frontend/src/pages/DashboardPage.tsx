/**
 * Dashboard Page
 *
 * Main dashboard with clock widget, KPIs, and presence overview.
 */

import { useEffect, useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { Clock, TrendingUp, Calendar, CheckCircle, Users, Globe, History, CalendarDays, ClipboardCheck, Zap } from 'lucide-react';
import { useAuth } from '../hooks/useAuth';
import { useOnboarding } from '../hooks/useOnboarding';
import { logger } from '../utils/logger';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../components/ui/card';
import { ClockWidget } from '../components/clock';
import { KPICard, PresenceWidget, HoursBarChart, TrendLineChart } from '../components/kpi';
import { WelcomeModal } from '../components/onboarding';
import {
  useKPIStore,
  getDateRange,
  getWeekRange,
  getMonthRange,
  navigatePeriod,
  formatPeriodLabel,
} from '../stores/kpiStore';
import { UserRole } from '../types/auth';

type Granularity = 'day' | 'week' | 'month';

/** Get current month name */
const getCurrentMonthName = (): string => {
  return new Date().toLocaleDateString('en-US', { month: 'long' });
};

export function DashboardPage() {
  const navigate = useNavigate();
  const { user, logout } = useAuth();
  const { myKpis, fetchMyKpis, charts, fetchCharts } = useKPIStore();
  const { showOnboarding, dismissOnboarding } = useOnboarding(user?.id);

  // Chart state management
  const [chartDate, setChartDate] = useState(new Date());
  const [chartGranularity, setChartGranularity] = useState<Granularity>('day');

  // Get date range based on granularity
  const getChartDateRange = useCallback((date: Date, granularity: Granularity) => {
    switch (granularity) {
      case 'day':
        return getWeekRange(date);
      case 'week':
        return getMonthRange(date);
      case 'month': {
        // Last 6 months
        const start = new Date(date);
        start.setMonth(start.getMonth() - 5);
        start.setDate(1);
        const end = new Date(date.getFullYear(), date.getMonth() + 1, 0, 23, 59, 59, 999);
        return {
          start_date: start.toISOString(),
          end_date: end.toISOString(),
        };
      }
      default:
        return getWeekRange(date);
    }
  }, []);

  // Get period label based on granularity
  const getPeriodLabel = useCallback((date: Date, granularity: Granularity): string => {
    switch (granularity) {
      case 'day':
        return formatPeriodLabel(date, 'week');
      case 'week':
        return formatPeriodLabel(date, 'month');
      case 'month': {
        const start = new Date(date);
        start.setMonth(start.getMonth() - 5);
        const formatMonth = (d: Date) => d.toLocaleDateString('en-US', { month: 'short', year: '2-digit' });
        return `${formatMonth(start)} - ${formatMonth(date)}`;
      }
      default:
        return '';
    }
  }, []);

  // Fetch charts with current settings
  const refreshCharts = useCallback((date: Date, granularity: Granularity) => {
    const range = getChartDateRange(date, granularity);
    fetchCharts({
      ...range,
      granularity,
    });
  }, [fetchCharts, getChartDateRange]);

  // Load user KPIs on mount
  useEffect(() => {
    const params = getDateRange('month');
    fetchMyKpis(params);
  }, [fetchMyKpis]);

  // Load chart data when settings change
  useEffect(() => {
    refreshCharts(chartDate, chartGranularity);
  }, [chartDate, chartGranularity, refreshCharts]);

  // Handle chart navigation
  const handleChartNavigate = useCallback((direction: 'prev' | 'next') => {
    const period = chartGranularity === 'day' ? 'week' : 'month';
    const newDate = navigatePeriod(chartDate, period, direction);
    setChartDate(newDate);
  }, [chartDate, chartGranularity]);

  // Handle granularity change
  const handleGranularityChange = useCallback((granularity: Granularity) => {
    setChartGranularity(granularity);
    setChartDate(new Date()); // Reset to current date on granularity change
  }, []);

  const handleLogout = async () => {
    try {
      await logout();
      navigate('/login');
    } catch (error) {
      logger.error('Logout failed', error, { component: 'DashboardPage', action: 'logout' });
    }
  };

  const isManager = user?.role === UserRole.Manager || user?.role === UserRole.Admin || user?.role === UserRole.SuperAdmin;
  const isAdmin = user?.role === UserRole.Admin || user?.role === UserRole.SuperAdmin;

  // Check if user has meaningful clock data (days_worked > 0)
  const hasClockData = myKpis && myKpis.days_worked > 0;

  // Period label for charts
  const periodLabel = getPeriodLabel(chartDate, chartGranularity);

  return (
    <div className="space-y-6">
      {/* Onboarding Welcome Modal */}
      {user && (
        <WelcomeModal
          open={showOnboarding}
          firstName={user.first_name}
          role={user.role}
          onDismiss={dismissOnboarding}
        />
      )}

      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">
            Welcome, {user?.first_name}!
          </h1>
          <div className="flex items-center gap-4 text-muted-foreground">
            <span>Here&apos;s your time tracking overview</span>
            {user?.organization_timezone && (
              <span className="flex items-center gap-1 text-sm">
                <Globe className="h-3.5 w-3.5" />
                {user.organization_timezone}
              </span>
            )}
          </div>
        </div>
        <Button onClick={handleLogout} variant="outline">
          Logout
        </Button>
      </div>

      {/* Main Grid */}
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        {/* Clock Widget - Takes priority */}
        <div className="lg:col-span-1">
          <ClockWidget />
        </div>

        {/* KPI Cards */}
        <div className="lg:col-span-2">
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <KPICard
              title={`Hours in ${getCurrentMonthName()}`}
              value={hasClockData ? `${myKpis.total_hours_worked.toFixed(1)}h` : '—'}
              description={hasClockData ? `of ${myKpis.theoretical_hours.toFixed(0)}h expected` : 'No clock entries'}
              icon={<Clock className="h-5 w-5" />}
            />
            <KPICard
              title="Punctuality Rate"
              value={hasClockData ? `${myKpis.punctuality_rate.toFixed(0)}%` : '—'}
              description={hasClockData ? `${myKpis.days_late} late days` : 'No clock entries'}
              icon={<CheckCircle className="h-5 w-5" />}
              trend={hasClockData ? {
                value: myKpis.punctuality_rate >= 95 ? 5 : myKpis.punctuality_rate >= 90 ? 0 : -5,
                label: 'target 95%',
                isPositive: myKpis.punctuality_rate >= 95,
              } : undefined}
            />
            <KPICard
              title="Days Worked"
              value={myKpis?.days_worked ?? '—'}
              description={`in ${getCurrentMonthName()}`}
              icon={<Calendar className="h-5 w-5" />}
            />
            <KPICard
              title="Hours Variance"
              value={hasClockData ? `${myKpis.hours_variance >= 0 ? '+' : ''}${myKpis.hours_variance.toFixed(1)}h` : '—'}
              description={hasClockData ? 'vs expected' : 'No clock entries'}
              icon={<TrendingUp className="h-5 w-5" />}
              trend={hasClockData ? {
                value: Math.round(myKpis.hours_variance),
                label: 'difference',
                isPositive: myKpis.hours_variance >= 0,
              } : undefined}
            />
          </div>
        </div>
      </div>

      {/* Charts Section - All Users */}
      <div className="grid gap-6 md:grid-cols-2">
        <HoursBarChart
          data={charts?.data ?? []}
          title="Hours Worked"
          periodLabel={periodLabel}
          granularity={chartGranularity}
          onNavigate={handleChartNavigate}
          onGranularityChange={handleGranularityChange}
        />
        <TrendLineChart
          data={charts?.data ?? []}
          title="Hours Trend"
          periodLabel={periodLabel}
          granularity={chartGranularity}
          onNavigate={handleChartNavigate}
          onGranularityChange={handleGranularityChange}
        />
      </div>

      {/* Quick Actions & Presence Section */}
      <div className={`grid gap-6 ${isManager ? 'md:grid-cols-2' : 'md:grid-cols-1'}`}>
        {/* Presence Widget - Managers only */}
        {isManager && <PresenceWidget />}

        {/* Quick Actions Card - All users with role-specific options */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2 text-lg">
              <Zap className="h-5 w-5" />
              Quick Actions
            </CardTitle>
            <CardDescription>
              {isAdmin ? 'Management & personal shortcuts' : isManager ? 'Team management' : 'Personal shortcuts'}
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-2">
            {/* Employee actions - available to all */}
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => navigate('/clock/history')}
            >
              <History className="h-4 w-4" />
              View Clock History
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => navigate('/absences')}
            >
              <CalendarDays className="h-4 w-4" />
              Request Absence
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => navigate('/calendar')}
            >
              <Calendar className="h-4 w-4" />
              Team Calendar
            </Button>

            {/* Manager actions */}
            {isManager && (
              <>
                <div className="border-t my-2" />
                <Button
                  variant="outline"
                  className="w-full justify-start gap-2"
                  onClick={() => navigate('/clock/pending')}
                >
                  <ClipboardCheck className="h-4 w-4" />
                  Pending Clock Approvals
                </Button>
                <Button
                  variant="outline"
                  className="w-full justify-start gap-2"
                  onClick={() => navigate('/absences/pending')}
                >
                  <CheckCircle className="h-4 w-4" />
                  Pending Absence Requests
                </Button>
              </>
            )}

            {/* Admin actions */}
            {isAdmin && (
              <>
                <div className="border-t my-2" />
                <Button
                  variant="outline"
                  className="w-full justify-start gap-2"
                  onClick={() => navigate('/admin/users')}
                >
                  <Users className="h-4 w-4" />
                  Manage Users
                </Button>
                <Button
                  variant="outline"
                  className="w-full justify-start gap-2"
                  onClick={() => navigate('/admin/teams')}
                >
                  <Users className="h-4 w-4" />
                  Manage Teams
                </Button>
              </>
            )}
          </CardContent>
        </Card>
      </div>

      {/* User Info Card */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-lg">Your Profile</CardTitle>
          <Button variant="outline" size="sm" onClick={() => navigate('/profile')}>
            Edit Profile
          </Button>
        </CardHeader>
        <CardContent className={`grid gap-2 sm:grid-cols-2 ${user?.role === UserRole.SuperAdmin ? 'lg:grid-cols-5' : 'lg:grid-cols-4'}`}>
          <div>
            <p className="text-sm font-medium text-muted-foreground">Email</p>
            <p className="text-sm">{user?.email}</p>
          </div>
          {user?.role === UserRole.SuperAdmin && (
            <div>
              <p className="text-sm font-medium text-muted-foreground">Organization</p>
              <p className="text-sm">{user?.organization_name}</p>
            </div>
          )}
          <div>
            <p className="text-sm font-medium text-muted-foreground">Role</p>
            <p className="text-sm capitalize">{user?.role?.replace('_', ' ')}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-muted-foreground">Avg Daily Hours</p>
            <p className="text-sm">
              {hasClockData ? `${myKpis.average_daily_hours.toFixed(1)}h` : '—'}
            </p>
          </div>
          <div>
            <p className="text-sm font-medium text-muted-foreground">Settings</p>
            <div className="flex gap-2 mt-1">
              <Button
                variant="link"
                size="sm"
                className="h-auto p-0"
                onClick={() => navigate('/settings/password')}
              >
                Change Password
              </Button>
              <span className="text-muted-foreground">•</span>
              <Button
                variant="link"
                size="sm"
                className="h-auto p-0"
                onClick={() => navigate('/settings/sessions')}
              >
                Sessions
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
