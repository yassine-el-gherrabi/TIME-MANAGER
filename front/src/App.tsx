import { AuthProvider } from '@/context/AuthContext';
import { AppRoutes } from '@/routes';
import { Toaster } from '@/components/ui/toaster';

function App() {
  return (
    <AuthProvider>
      <AppRoutes />
      <Toaster />
    </AuthProvider>
  );
}

export default App;
