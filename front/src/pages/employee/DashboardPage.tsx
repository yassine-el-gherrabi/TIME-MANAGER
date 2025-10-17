import { useAuth } from '@/hooks/useAuth';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Header } from '@/components/shared/Header';

export const EmployeeDashboardPage = () => {
  const { user } = useAuth();

  return (
    <div className="min-h-screen bg-background">
      <Header title="Time Manager - Employee" />

      <main className="container mx-auto px-4 py-8">
        <Card>
          <CardHeader>
            <CardTitle>
              Welcome, {user?.firstName} {user?.lastName}!
            </CardTitle>
          </CardHeader>
          <CardContent>
            <p className="text-muted-foreground">
              This is your employee dashboard. Features will be added in the next iterations.
            </p>
            <div className="mt-4 space-y-2">
              <p>
                <strong>Email:</strong> {user?.email}
              </p>
              <p>
                <strong>Role:</strong> {user?.role}
              </p>
            </div>
          </CardContent>
        </Card>
      </main>
    </div>
  );
};
