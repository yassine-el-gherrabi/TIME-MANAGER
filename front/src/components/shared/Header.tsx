import { useAuth } from '@/hooks/useAuth';
import { Button } from '@/components/ui/button';
import { LogOut, Clock } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { handleError } from '@/utils/errorHandler';
import { ROUTE_PATHS } from '@/routes/config';

interface HeaderProps {
  title?: string;
}

export const Header: React.FC<HeaderProps> = ({ title = 'Time Manager' }) => {
  const { logout } = useAuth();
  const navigate = useNavigate();

  const handleLogout = async () => {
    try {
      await logout();
      toast.success('Logged out successfully');
      navigate(ROUTE_PATHS.LOGIN, { replace: true });
    } catch (error) {
      handleError(error, 'Logout');
    }
  };

  return (
    <header className="border-b">
      <div className="container mx-auto flex items-center justify-between px-4 py-4">
        <div className="flex items-center gap-2">
          <Clock className="h-6 w-6" />
          <h1 className="text-xl font-bold">{title}</h1>
        </div>
        <Button onClick={handleLogout} variant="outline" size="sm">
          <LogOut className="mr-2 h-4 w-4" />
          Logout
        </Button>
      </div>
    </header>
  );
};
