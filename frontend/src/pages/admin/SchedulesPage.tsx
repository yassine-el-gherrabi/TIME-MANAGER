/**
 * Schedules Management Page
 *
 * Admin page for managing work schedules.
 */

import { Calendar } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../components/ui/card';

export function SchedulesPage() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Work Schedules</h1>
        <p className="text-muted-foreground">
          Create and manage work schedules for your organization
        </p>
      </div>

      {/* Placeholder Card */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Calendar className="h-5 w-5" />
            Schedules
          </CardTitle>
          <CardDescription>
            Schedule management functionality coming soon
          </CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            This feature is under development. You will be able to:
          </p>
          <ul className="list-disc list-inside mt-2 text-sm text-muted-foreground space-y-1">
            <li>Create and edit work schedules</li>
            <li>Define working hours for each day</li>
            <li>Set break times</li>
            <li>Assign schedules to users</li>
          </ul>
        </CardContent>
      </Card>
    </div>
  );
}
