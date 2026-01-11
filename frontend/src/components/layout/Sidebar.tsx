import type { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import {
  LayoutDashboard,
  Users,
  UsersRound,
  Calendar,
  CalendarDays,
  CalendarCheck,
  KeyRound,
  MonitorSmartphone,
  LogOut,
  Clock,
  ClipboardCheck,
  Timer,
  Briefcase,
  FileType,
  PartyPopper,
  User,
  ScrollText,
  Building2,
} from 'lucide-react';
import { NavLink } from './NavLink';
import { Button } from '../ui/button';
import { ClockStatusIndicator } from '../clock/ClockStatusIndicator';
import { NotificationBell } from '../notifications';
import { LanguageSelector } from '../LanguageSelector';
import { useAuthStore } from '../../stores/authStore';
import { UserRole } from '../../types/auth';

export const Sidebar: FC = () => {
  const navigate = useNavigate();
  const { t } = useTranslation();
  const { user, logout } = useAuthStore();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <aside className="sticky top-0 flex h-screen w-64 flex-col border-r bg-background">
      {/* Logo and Notifications */}
      <div className="flex h-16 items-center justify-between border-b px-6">
        <div className="flex items-center gap-2">
          <Clock className="h-6 w-6 text-primary" />
          <span className="text-lg font-semibold">Time Manager</span>
        </div>
        <div className="flex items-center gap-1">
          <LanguageSelector />
          <NotificationBell />
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 p-4">
        <NavLink to="/" icon={<LayoutDashboard className="h-4 w-4" />} end>
          {t('nav.dashboard')}
        </NavLink>
        <NavLink to="/clock" icon={<Timer className="h-4 w-4" />} end>
          <span className="flex items-center gap-2">
            {t('nav.timeClock')}
            <ClockStatusIndicator />
          </span>
        </NavLink>
        <NavLink to="/absences" icon={<Briefcase className="h-4 w-4" />} end>
          {t('nav.myAbsences')}
        </NavLink>

        {/* Manager+ Section */}
        {(user?.role === UserRole.Manager || user?.role === UserRole.Admin || user?.role === UserRole.SuperAdmin) && (
          <div className="pt-4">
            <p className="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              {t('nav.management')}
            </p>
            <NavLink to="/clock/pending" icon={<ClipboardCheck className="h-4 w-4" />}>
              {t('nav.pendingClocks')}
            </NavLink>
            <NavLink to="/absences/pending" icon={<CalendarCheck className="h-4 w-4" />}>
              {t('nav.pendingAbsences')}
            </NavLink>
            <NavLink to="/absences/calendar" icon={<CalendarDays className="h-4 w-4" />}>
              {t('nav.calendar')}
            </NavLink>
          </div>
        )}

        {/* Admin Only */}
        {(user?.role === UserRole.Admin || user?.role === UserRole.SuperAdmin) && (
          <div className="pt-4">
            <p className="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              {t('nav.admin')}
            </p>
            <NavLink to="/admin/users" icon={<Users className="h-4 w-4" />}>
              {t('nav.users')}
            </NavLink>
            <NavLink to="/admin/teams" icon={<UsersRound className="h-4 w-4" />}>
              {t('nav.teams')}
            </NavLink>
            <NavLink to="/admin/schedules" icon={<Calendar className="h-4 w-4" />}>
              {t('nav.workSchedules')}
            </NavLink>
            <NavLink to="/admin/absence-types" icon={<FileType className="h-4 w-4" />}>
              {t('nav.absenceTypes')}
            </NavLink>
            <NavLink to="/admin/closed-days" icon={<PartyPopper className="h-4 w-4" />}>
              {t('nav.closedDays')}
            </NavLink>
          </div>
        )}

        {/* Super Admin Only */}
        {user?.role === UserRole.SuperAdmin && (
          <div className="pt-4">
            <p className="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
              Super Admin
            </p>
            <NavLink to="/admin/organizations" icon={<Building2 className="h-4 w-4" />}>
              {t('nav.organizations')}
            </NavLink>
            <NavLink to="/admin/audit-logs" icon={<ScrollText className="h-4 w-4" />}>
              {t('nav.auditLogs')}
            </NavLink>
          </div>
        )}

        {/* Settings Section */}
        <div className="pt-4">
          <p className="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            {t('nav.settings')}
          </p>
          <NavLink to="/profile" icon={<User className="h-4 w-4" />}>
            {t('nav.profile')}
          </NavLink>
          <NavLink to="/settings/password" icon={<KeyRound className="h-4 w-4" />}>
            {t('nav.password')}
          </NavLink>
          <NavLink to="/settings/sessions" icon={<MonitorSmartphone className="h-4 w-4" />}>
            {t('nav.sessions')}
          </NavLink>
        </div>
      </nav>

      {/* User Info & Logout */}
      <div className="border-t p-4">
        <div className="mb-3 px-3">
          <p className="text-sm font-medium truncate">
            {user?.first_name} {user?.last_name}
          </p>
          <p className="text-xs text-muted-foreground truncate">{user?.email}</p>
        </div>
        <Button
          variant="ghost"
          className="w-full justify-start gap-3 text-muted-foreground hover:text-foreground"
          onClick={handleLogout}
        >
          <LogOut className="h-4 w-4" />
          {t('auth.logout')}
        </Button>
      </div>
    </aside>
  );
};
