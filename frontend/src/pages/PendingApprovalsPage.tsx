/**
 * Pending Approvals Page
 *
 * Manager page for reviewing and approving clock entries.
 */

import { PendingApprovals } from '../components/clock';

export function PendingApprovalsPage() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight">Pending Approvals</h1>
        <p className="text-muted-foreground">
          Review and approve employee clock entries
        </p>
      </div>

      {/* Pending Approvals */}
      <PendingApprovals />
    </div>
  );
}
