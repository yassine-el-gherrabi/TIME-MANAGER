import type { FC } from 'react';
import { Clock } from 'lucide-react';
import { Sheet, SheetContent, SheetHeader, SheetTitle } from '../ui/sheet';
import { SidebarContent } from './Sidebar';
import * as VisuallyHidden from '@radix-ui/react-visually-hidden';

interface MobileSidebarProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export const MobileSidebar: FC<MobileSidebarProps> = ({ open, onOpenChange }) => {
  const handleNavClick = () => {
    onOpenChange(false);
  };

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="left" className="flex w-72 flex-col p-0">
        <SheetHeader className="flex h-14 flex-row items-center gap-2 border-b px-4">
          <Clock className="h-5 w-5 text-primary" />
          <SheetTitle className="text-base">Time Manager</SheetTitle>
        </SheetHeader>
        <VisuallyHidden.Root>
          <p>Navigation menu</p>
        </VisuallyHidden.Root>
        <div className="flex flex-1 flex-col overflow-hidden">
          <SidebarContent onNavClick={handleNavClick} />
        </div>
      </SheetContent>
    </Sheet>
  );
};
