/**
 * Dashboard Page
 *
 * Main dashboard with clock widget, KPIs, and presence overview.
 */

import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Clock, TrendingUp, Calendar, CheckCircle, Users } from 'lucide-react';
import { useAuth } from '../hooks/useAuth';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../components/ui/card';
import { ClockWidget } from '../components/clock';
import { KPICard, PresenceWidget, HoursBarChart, TrendLineChart } from '../components/kpi';
import { useKPIStore, getDateRange } from '../stores/kpiStore';
import { UserRole } from '../types/auth';

export function DashboardPage() {
  const navigate = useNavigate();
  const { user, logout } = useAuth();
  const { myKpis, fetchMyKpis, charts, fetchCharts } = useKPIStore();

  // Load user KPIs and chart data on mount
  useEffect(() => {
    const params = getDateRange('month');
    fetchMyKpis(params);
    // Fetch weekly chart data (last 7 days, daily granularity)
    fetchCharts({
      ...getDateRange('week'),
      granularity: 'day',
    });
  }, [fetchMyKpis, fetchCharts]);

  const handleLogout = async () => {
    try {
      await logout();
      navigate('/login');
    } catch (error) {
      console.error('Logout failed:', error);
    }
  };

  const isManager = user?.role === UserRole.Manager || user?.role === UserRole.Admin || user?.role === UserRole.SuperAdmin;
  const isAdmin = user?.role === UserRole.Admin || user?.role === UserRole.SuperAdmin;

  // Check if user has meaningful clock data (days_worked > 0)
  const hasClockData = myKpis && myKpis.days_worked > 0;

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">
            Welcome, {user?.first_name}!
          </h1>
          <p className="text-muted-foreground">
            Here&apos;s your time tracking overview
          </p>
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
              title="Hours This Month"
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
              description="This month"
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
          title="Weekly Hours"
          description="Hours worked this week"
          granularity={charts?.granularity ?? 'day'}
        />
        <TrendLineChart
          data={charts?.data ?? []}
          title="Hours Trend"
          description="Daily progression"
          granularity={charts?.granularity ?? 'day'}
        />
      </div>

      {/* Manager Section - Presence Widget */}
      {isManager && (
        <div className="grid gap-6 md:grid-cols-2">
          <PresenceWidget />

          {/* Quick Actions Card */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2 text-lg">
                <Users className="h-5 w-5" />
                Quick Actions
              </CardTitle>
              <CardDescription>
                Management shortcuts
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-2">
              <Button
                variant="outline"
                className="w-full justify-start"
                onClick={() => navigate('/clock/pending')}
              >
                View Pending Approvals
              </Button>
              {isAdmin && (
                <>
                  <Button
                    variant="outline"
                    className="w-full justify-start"
                    onClick={() => navigate('/admin/users')}
                  >
                    Manage Users
                  </Button>
                  <Button
                    variant="outline"
                    className="w-full justify-start"
                    onClick={() => navigate('/admin/teams')}
                  >
                    Manage Teams
                  </Button>
                  <Button
                    variant="outline"
                    className="w-full justify-start"
                    onClick={() => navigate('/admin/schedules')}
                  >
                    Manage Schedules
                  </Button>
                </>
              )}
            </CardContent>
          </Card>
        </div>
      )}

      {/* User Info Card */}
      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-lg">Your Profile</CardTitle>
          <Button variant="outline" size="sm" onClick={() => navigate('/profile')}>
            Edit Profile
          </Button>
        </CardHeader>
        <CardContent className="grid gap-2 sm:grid-cols-2 lg:grid-cols-4">
          <div>
            <p className="text-sm font-medium text-muted-foreground">Email</p>
            <p className="text-sm">{user?.email}</p>
          </div>
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
