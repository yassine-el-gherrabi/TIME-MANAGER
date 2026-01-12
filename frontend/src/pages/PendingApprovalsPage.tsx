/**
 * Pending Approvals Page
 *
 * Manager page for reviewing and approving clock entries.
 */

import { useTranslation } from 'react-i18next';
import { PendingApprovals } from '../components/clock';

export function PendingApprovalsPage() {
  const { t } = useTranslation();

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-2xl font-bold tracking-tight">{t('clock.pendingApprovals')}</h1>
        <p className="text-muted-foreground">
          {t('clock.reviewApprove')}
        </p>
      </div>

      {/* Pending Approvals */}
      <PendingApprovals />
    </div>
  );
}
