import type { FC } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  LayoutDashboard,
  Users,
  KeyRound,
  MonitorSmartphone,
  LogOut,
  Clock,
} from 'lucide-react';
import { NavLink } from './NavLink';
import { Button } from '../ui/button';
import { useAuthStore } from '../../stores/authStore';
import { UserRole } from '../../types/auth';

export const Sidebar: FC = () => {
  const navigate = useNavigate();
  const { user, logout } = useAuthStore();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <aside className="flex h-screen w-64 flex-col border-r bg-background">
      {/* Logo */}
      <div className="flex h-16 items-center gap-2 border-b px-6">
        <Clock className="h-6 w-6 text-primary" />
        <span className="text-lg font-semibold">Time Manager</span>
      </div>

      {/* Navigation */}
      <nav className="flex-1 space-y-1 p-4">
        <NavLink to="/" icon={<LayoutDashboard className="h-4 w-4" />} end>
          Dashboard
        </NavLink>

        {/* Admin Only */}
        {user?.role === UserRole.Admin && (
          <NavLink to="/admin/users" icon={<Users className="h-4 w-4" />}>
            Users
          </NavLink>
        )}

        {/* Settings Section */}
        <div className="pt-4">
          <p className="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
            Settings
          </p>
          <NavLink to="/settings/password" icon={<KeyRound className="h-4 w-4" />}>
            Change Password
          </NavLink>
          <NavLink to="/settings/sessions" icon={<MonitorSmartphone className="h-4 w-4" />}>
            Sessions
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
          Logout
        </Button>
      </div>
    </aside>
  );
};
