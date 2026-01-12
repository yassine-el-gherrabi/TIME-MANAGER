import type { FC } from 'react';
import { Menu, Clock } from 'lucide-react';
import { Button } from '../ui/button';
import { NotificationBell } from '../notifications';
import { LanguageSelector } from '../LanguageSelector';

interface MobileHeaderProps {
  onMenuClick: () => void;
}

export const MobileHeader: FC<MobileHeaderProps> = ({ onMenuClick }) => {
  return (
    <header className="sticky top-0 z-40 flex h-14 items-center justify-between border-b bg-background px-4 lg:hidden">
      <div className="flex items-center gap-3">
        <Button variant="ghost" size="icon" onClick={onMenuClick} aria-label="Open menu">
          <Menu className="h-5 w-5" />
        </Button>
        <div className="flex items-center gap-2">
          <Clock className="h-5 w-5 text-primary" />
          <span className="font-semibold">Time Manager</span>
        </div>
      </div>
      <div className="flex items-center gap-1">
        <LanguageSelector />
        <NotificationBell />
      </div>
    </header>
  );
};
