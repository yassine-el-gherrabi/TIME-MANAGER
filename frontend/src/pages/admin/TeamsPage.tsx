/**
 * Teams Management Page
 *
 * Admin page for managing teams.
 */

import { Users } from 'lucide-react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../components/ui/card';

export function TeamsPage() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Teams Management</h1>
        <p className="text-muted-foreground">
          Create and manage teams in your organization
        </p>
      </div>

      {/* Placeholder Card */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            <Users className="h-5 w-5" />
            Teams
          </CardTitle>
          <CardDescription>
            Team management functionality coming soon
          </CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">
            This feature is under development. You will be able to:
          </p>
          <ul className="list-disc list-inside mt-2 text-sm text-muted-foreground space-y-1">
            <li>Create and edit teams</li>
            <li>Assign team managers</li>
            <li>Add and remove team members</li>
            <li>View team statistics</li>
          </ul>
        </CardContent>
      </Card>
    </div>
  );
}
