import type { FC, ReactNode } from 'react';
import { NavLink as RouterNavLink } from 'react-router-dom';
import { cn } from '../../lib/utils';

export interface NavLinkProps {
  to: string;
  icon: ReactNode;
  children: ReactNode;
  end?: boolean;
}

export const NavLink: FC<NavLinkProps> = ({ to, icon, children, end = false }) => {
  return (
    <RouterNavLink
      to={to}
      end={end}
      className={({ isActive }) =>
        cn(
          'flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors',
          isActive
            ? 'bg-primary text-primary-foreground'
            : 'text-muted-foreground hover:bg-accent hover:text-accent-foreground'
        )
      }
    >
      {icon}
      {children}
    </RouterNavLink>
  );
};
