/**
 * Password Requirements Component
 *
 * Displays real-time password validation with checkmarks for each requirement.
 * Shows visual feedback as user types their password.
 */

import { PASSWORD_RULES } from '../../config/constants';
import { Check, X } from 'lucide-react';

export interface PasswordValidation {
  minLength: boolean;
  hasUppercase: boolean;
  hasLowercase: boolean;
  hasNumber: boolean;
  hasSpecial: boolean;
}

/**
 * Validate a password against all rules
 */
export const validatePassword = (password: string): PasswordValidation => {
  return {
    minLength: password.length >= PASSWORD_RULES.MIN_LENGTH,
    hasUppercase: /[A-Z]/.test(password),
    hasLowercase: /[a-z]/.test(password),
    hasNumber: /[0-9]/.test(password),
    hasSpecial: new RegExp(`[${PASSWORD_RULES.SPECIAL_CHARS.replace(/[-[\]{}()*+?.,\\^$|#\s]/g, '\\$&')}]`).test(password),
  };
};

/**
 * Check if password meets all requirements
 */
export const isPasswordValid = (password: string): boolean => {
  const validation = validatePassword(password);
  return (
    validation.minLength &&
    (!PASSWORD_RULES.REQUIRE_UPPERCASE || validation.hasUppercase) &&
    (!PASSWORD_RULES.REQUIRE_LOWERCASE || validation.hasLowercase) &&
    (!PASSWORD_RULES.REQUIRE_NUMBER || validation.hasNumber) &&
    (!PASSWORD_RULES.REQUIRE_SPECIAL || validation.hasSpecial)
  );
};

interface RequirementItemProps {
  met: boolean;
  label: string;
}

const RequirementItem = ({ met, label }: RequirementItemProps) => (
  <li className={`flex items-center gap-2 text-sm ${met ? 'text-green-600' : 'text-muted-foreground'}`}>
    {met ? (
      <Check className="h-4 w-4 text-green-600" />
    ) : (
      <X className="h-4 w-4 text-muted-foreground" />
    )}
    <span>{label}</span>
  </li>
);

interface PasswordRequirementsProps {
  password: string;
  showTitle?: boolean;
}

/**
 * Password Requirements Component
 *
 * Shows real-time validation feedback for password requirements.
 */
export const PasswordRequirements = ({ password, showTitle = true }: PasswordRequirementsProps) => {
  const validation = validatePassword(password);

  return (
    <div className="space-y-2">
      {showTitle && (
        <p className="text-sm font-medium text-muted-foreground">Password requirements:</p>
      )}
      <ul className="space-y-1">
        <RequirementItem
          met={validation.minLength}
          label={`At least ${PASSWORD_RULES.MIN_LENGTH} characters`}
        />
        {PASSWORD_RULES.REQUIRE_UPPERCASE && (
          <RequirementItem
            met={validation.hasUppercase}
            label="At least one uppercase letter (A-Z)"
          />
        )}
        {PASSWORD_RULES.REQUIRE_LOWERCASE && (
          <RequirementItem
            met={validation.hasLowercase}
            label="At least one lowercase letter (a-z)"
          />
        )}
        {PASSWORD_RULES.REQUIRE_NUMBER && (
          <RequirementItem
            met={validation.hasNumber}
            label="At least one number (0-9)"
          />
        )}
        {PASSWORD_RULES.REQUIRE_SPECIAL && (
          <RequirementItem
            met={validation.hasSpecial}
            label="At least one special character (!@#$%^&*...)"
          />
        )}
      </ul>
    </div>
  );
};

export default PasswordRequirements;
