/**
 * Reject Reason Modal Component
 *
 * Dialog for entering rejection reason when rejecting an absence request.
 */

import { useState } from 'react';
import type { FC, FormEvent } from 'react';
import { useTranslation } from 'react-i18next';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '../ui/dialog';
import { Button } from '../ui/button';
import { Label } from '../ui/label';

export interface RejectReasonModalProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: (reason: string) => void | Promise<void>;
  loading?: boolean;
  userName?: string;
  absenceType?: string;
}

export const RejectReasonModal: FC<RejectReasonModalProps> = ({
  open,
  onOpenChange,
  onConfirm,
  loading = false,
  userName,
  absenceType,
}) => {
  const { t } = useTranslation();
  const [reason, setReason] = useState('');
  const [error, setError] = useState<string>();

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();

    if (!reason.trim()) {
      setError(t('pendingAbsences.provideReason'));
      return;
    }

    await onConfirm(reason.trim());
    setReason('');
    setError(undefined);
  };

  const handleCancel = () => {
    setReason('');
    setError(undefined);
    onOpenChange(false);
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('pendingAbsences.rejectRequest')}</DialogTitle>
          <DialogDescription>
            {userName && absenceType
              ? t('pendingAbsences.rejectDescription', { userName, absenceType })
              : t('pendingAbsences.reasonRequired')}
          </DialogDescription>
        </DialogHeader>

        <form onSubmit={handleSubmit}>
          <div className="space-y-4 py-4">
            <div className="space-y-2">
              <Label htmlFor="rejection-reason">{t('pendingAbsences.reasonForRejection')}</Label>
              <textarea
                id="rejection-reason"
                value={reason}
                onChange={(e) => {
                  setReason(e.target.value);
                  if (error) setError(undefined);
                }}
                disabled={loading}
                placeholder={t('pendingAbsences.reasonPlaceholder')}
                className="flex min-h-[100px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
                autoFocus
              />
              {error && <p className="text-xs text-destructive">{error}</p>}
            </div>
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={handleCancel}
              disabled={loading}
            >
              {t('common.cancel')}
            </Button>
            <Button
              type="submit"
              variant="destructive"
              disabled={loading}
            >
              {loading ? t('common.rejecting') : t('pendingAbsences.rejectRequest')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};
