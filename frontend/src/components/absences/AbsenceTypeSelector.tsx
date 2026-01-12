/**
 * Absence Type Selector Component
 *
 * Dropdown selector for absence types with color indicators.
 */

import type { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { cn } from '../../lib/utils';
import type { AbsenceType } from '../../types/absence';

interface AbsenceTypeSelectorProps {
  types: AbsenceType[];
  value: string;
  onChange: (typeId: string) => void;
  disabled?: boolean;
  error?: string;
  className?: string;
  placeholder?: string;
}

export const AbsenceTypeSelector: FC<AbsenceTypeSelectorProps> = ({
  types,
  value,
  onChange,
  disabled = false,
  error,
  className,
  placeholder,
}) => {
  const { t } = useTranslation();
  const selectedType = types.find((type) => type.id === value);
  const placeholderText = placeholder || t('absences.selectAbsenceType');

  return (
    <div className={cn('space-y-1', className)}>
      <div className="relative">
        {/* Color indicator for selected type */}
        {selectedType && (
          <div
            className="absolute left-3 top-1/2 -translate-y-1/2 h-3 w-3 rounded-full"
            style={{ backgroundColor: selectedType.color }}
          />
        )}

        <select
          value={value}
          onChange={(e) => onChange(e.target.value)}
          disabled={disabled}
          className={cn(
            'flex h-9 w-full rounded-md border border-input bg-background py-1 text-sm shadow-sm transition-colors',
            'focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring',
            'disabled:cursor-not-allowed disabled:opacity-50',
            selectedType ? 'pl-8 pr-3' : 'px-3',
            error && 'border-destructive focus-visible:ring-destructive'
          )}
        >
          <option value="">{placeholderText}</option>
          {types.map((absenceType) => (
            <option key={absenceType.id} value={absenceType.id}>
              {absenceType.name} {absenceType.affects_balance ? `(${t('absences.affectsBalance')})` : ''}
            </option>
          ))}
        </select>
      </div>

      {error && <p className="text-xs text-destructive">{error}</p>}

      {/* Type details hint */}
      {selectedType && (
        <div className="flex items-center gap-2 text-xs text-muted-foreground">
          <div
            className="h-2 w-2 rounded-full"
            style={{ backgroundColor: selectedType.color }}
          />
          <span>
            {selectedType.requires_approval ? t('absences.requiresApproval') : t('absences.autoApproved')}
            {selectedType.affects_balance && ` • ${t('absences.deductsFromBalance')}`}
            {selectedType.is_paid && ` • ${t('absences.paidLeave')}`}
          </span>
        </div>
      )}
    </div>
  );
};
