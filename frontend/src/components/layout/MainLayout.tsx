import { useState, type FC, type ReactNode } from 'react';
import { Sidebar } from './Sidebar';
import { MobileHeader } from './MobileHeader';
import { MobileSidebar } from './MobileSidebar';

export interface MainLayoutProps {
  children: ReactNode;
}

export const MainLayout: FC<MainLayoutProps> = ({ children }) => {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);

  return (
    <div className="flex min-h-screen flex-col bg-muted/30 lg:flex-row">
      {/* Mobile Header - visible on mobile only */}
      <MobileHeader onMenuClick={() => setMobileMenuOpen(true)} />

      {/* Mobile Sidebar Drawer */}
      <MobileSidebar open={mobileMenuOpen} onOpenChange={setMobileMenuOpen} />

      {/* Desktop Sidebar - hidden on mobile */}
      <Sidebar />

      {/* Main Content */}
      <main className="flex-1 overflow-auto p-4 lg:p-6">{children}</main>
    </div>
  );
};
