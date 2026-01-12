/**
 * Dashboard Page
 *
 * Main dashboard with clock widget, KPIs, and presence overview.
 */

import { useEffect, useState, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
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

/** Get current month name based on locale */
const getCurrentMonthName = (locale: string): string => {
  return new Date().toLocaleDateString(locale === 'fr' ? 'fr-FR' : 'en-US', { month: 'long' });
};

export function DashboardPage() {
  const navigate = useNavigate();
  const { t, i18n } = useTranslation();
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

  // Period label and boundaries for charts
  const periodLabel = getPeriodLabel(chartDate, chartGranularity);
  const chartDateRange = getChartDateRange(chartDate, chartGranularity);

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
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-xl sm:text-2xl font-bold tracking-tight">
            {t('dashboard.welcome', { name: user?.first_name })}
          </h1>
          <div className="flex flex-wrap items-center gap-2 sm:gap-4 text-muted-foreground">
            <span>{t('dashboard.overview')}</span>
            {user?.organization_timezone && (
              <span className="flex items-center gap-1 text-sm">
                <Globe className="h-3.5 w-3.5" />
                {user.organization_timezone}
              </span>
            )}
          </div>
        </div>
        <Button onClick={handleLogout} variant="outline" className="w-full sm:w-auto">
          {t('auth.logout')}
        </Button>
      </div>

      {/* Main Grid */}
      <div className="grid gap-6 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
        {/* Clock Widget - Takes priority */}
        <div className="lg:col-span-1">
          <ClockWidget />
        </div>

        {/* KPI Cards */}
        <div className="lg:col-span-2">
          <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
            <KPICard
              title={t('dashboard.hoursThisMonth', { month: getCurrentMonthName(i18n.language) })}
              value={hasClockData ? `${myKpis.total_hours_worked.toFixed(1)}h` : '—'}
              description={hasClockData ? t('dashboard.ofExpected', { hours: myKpis.theoretical_hours.toFixed(0) }) : t('dashboard.noClockEntries')}
              icon={<Clock className="h-5 w-5" />}
            />
            <KPICard
              title={t('dashboard.punctualityRate')}
              value={hasClockData ? `${myKpis.punctuality_rate.toFixed(0)}%` : '—'}
              description={hasClockData ? t('dashboard.lateDays', { days: myKpis.days_late }) : t('dashboard.noClockEntries')}
              icon={<CheckCircle className="h-5 w-5" />}
              trend={hasClockData ? {
                value: myKpis.punctuality_rate >= 95 ? 5 : myKpis.punctuality_rate >= 90 ? 0 : -5,
                label: `${t('common.target')} 95%`,
                isPositive: myKpis.punctuality_rate >= 95,
              } : undefined}
            />
            <KPICard
              title={t('dashboard.daysWorked')}
              value={myKpis?.days_worked ?? '—'}
              description={`${t('common.in')} ${getCurrentMonthName(i18n.language)}`}
              icon={<Calendar className="h-5 w-5" />}
            />
            <KPICard
              title={t('dashboard.hoursVariance')}
              value={hasClockData ? `${myKpis.hours_variance >= 0 ? '+' : ''}${myKpis.hours_variance.toFixed(1)}h` : '—'}
              description={hasClockData ? t('dashboard.vsExpected') : t('dashboard.noClockEntries')}
              icon={<TrendingUp className="h-5 w-5" />}
              trend={hasClockData ? {
                value: Math.round(myKpis.hours_variance),
                label: t('common.difference'),
                isPositive: myKpis.hours_variance >= 0,
              } : undefined}
            />
          </div>
        </div>
      </div>

      {/* Charts Section - All Users */}
      <div className="grid gap-6 grid-cols-1 md:grid-cols-2">
        <HoursBarChart
          data={charts?.data ?? []}
          title={t('dashboard.hoursWorked')}
          periodLabel={periodLabel}
          granularity={chartGranularity}
          periodStart={chartDateRange.start_date}
          periodEnd={chartDateRange.end_date}
          onNavigate={handleChartNavigate}
          onGranularityChange={handleGranularityChange}
        />
        <TrendLineChart
          data={charts?.data ?? []}
          title={t('dashboard.hoursTrend')}
          periodLabel={periodLabel}
          granularity={chartGranularity}
          periodStart={chartDateRange.start_date}
          periodEnd={chartDateRange.end_date}
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
              {t('dashboard.quickActions')}
            </CardTitle>
            <CardDescription>
              {isAdmin ? t('dashboard.managementShortcuts') : isManager ? t('dashboard.teamManagement') : t('dashboard.personalShortcuts')}
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
              {t('dashboard.viewClockHistory')}
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => navigate('/absences')}
            >
              <CalendarDays className="h-4 w-4" />
              {t('dashboard.requestAbsence')}
            </Button>
            <Button
              variant="outline"
              className="w-full justify-start gap-2"
              onClick={() => navigate('/calendar')}
            >
              <Calendar className="h-4 w-4" />
              {t('dashboard.teamCalendar')}
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
                  {t('dashboard.pendingClockApprovals')}
                </Button>
                <Button
                  variant="outline"
                  className="w-full justify-start gap-2"
                  onClick={() => navigate('/absences/pending')}
                >
                  <CheckCircle className="h-4 w-4" />
                  {t('dashboard.pendingAbsenceRequests')}
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
                  {t('dashboard.manageUsers')}
                </Button>
                <Button
                  variant="outline"
                  className="w-full justify-start gap-2"
                  onClick={() => navigate('/admin/teams')}
                >
                  <Users className="h-4 w-4" />
                  {t('dashboard.manageTeams')}
                </Button>
              </>
            )}
          </CardContent>
        </Card>
      </div>

      {/* User Info Card */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-lg">{t('dashboard.yourProfile')}</CardTitle>
          <Button variant="outline" size="sm" onClick={() => navigate('/profile')}>
            {t('dashboard.editProfile')}
          </Button>
        </CardHeader>
        <CardContent className={`grid gap-4 grid-cols-1 sm:grid-cols-2 ${user?.role === UserRole.SuperAdmin ? 'lg:grid-cols-5' : 'lg:grid-cols-4'}`}>
          <div>
            <p className="text-sm font-medium text-muted-foreground">{t('common.email')}</p>
            <p className="text-sm">{user?.email}</p>
          </div>
          {user?.role === UserRole.SuperAdmin && (
            <div>
              <p className="text-sm font-medium text-muted-foreground">{t('users.organization')}</p>
              <p className="text-sm">{user?.organization_name}</p>
            </div>
          )}
          <div>
            <p className="text-sm font-medium text-muted-foreground">{t('users.role')}</p>
            <p className="text-sm capitalize">{user?.role?.replace('_', ' ')}</p>
          </div>
          <div>
            <p className="text-sm font-medium text-muted-foreground">{t('dashboard.avgDailyHours')}</p>
            <p className="text-sm">
              {hasClockData ? `${myKpis.average_daily_hours.toFixed(1)}h` : '—'}
            </p>
          </div>
          <div>
            <p className="text-sm font-medium text-muted-foreground">{t('nav.settings')}</p>
            <div className="flex gap-2 mt-1">
              <Button
                variant="link"
                size="sm"
                className="h-auto p-0"
                onClick={() => navigate('/settings/password')}
              >
                {t('dashboard.changePassword')}
              </Button>
              <span className="text-muted-foreground">•</span>
              <Button
                variant="link"
                size="sm"
                className="h-auto p-0"
                onClick={() => navigate('/settings/sessions')}
              >
                {t('nav.sessions')}
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
