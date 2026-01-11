/**
 * useOnboarding Hook
 *
 * Manages the onboarding state for new users.
 * Tracks whether a user has seen the welcome modal using localStorage.
 */

import { useState, useEffect, useCallback } from 'react';
import { STORAGE_KEYS } from '../config/constants';

/**
 * Get the storage key for a specific user
 */
const getStorageKey = (userId: string): string => {
  return `${STORAGE_KEYS.ONBOARDING_SEEN}_${userId}`;
};

/**
 * Check if a user has seen the onboarding
 */
const hasSeenOnboarding = (userId: string): boolean => {
  if (typeof window === 'undefined') return true;
  return localStorage.getItem(getStorageKey(userId)) === 'true';
};

/**
 * Mark onboarding as seen for a user
 */
const markOnboardingSeen = (userId: string): void => {
  if (typeof window === 'undefined') return;
  localStorage.setItem(getStorageKey(userId), 'true');
};

export interface UseOnboardingReturn {
  /** Whether the onboarding modal should be shown */
  showOnboarding: boolean;
  /** Dismiss the onboarding modal and optionally mark as seen */
  dismissOnboarding: (dontShowAgain?: boolean) => void;
  /** Reset onboarding state (useful for testing) */
  resetOnboarding: () => void;
}

/**
 * Hook to manage onboarding state
 *
 * @param userId - The current user's ID
 * @returns Onboarding state and actions
 *
 * @example
 * ```tsx
 * const { showOnboarding, dismissOnboarding } = useOnboarding(user.id);
 *
 * return (
 *   <>
 *     {showOnboarding && (
 *       <WelcomeModal onDismiss={dismissOnboarding} />
 *     )}
 *   </>
 * );
 * ```
 */
export const useOnboarding = (userId: string | undefined): UseOnboardingReturn => {
  const [showOnboarding, setShowOnboarding] = useState(false);

  useEffect(() => {
    if (!userId) {
      setShowOnboarding(false);
      return;
    }

    // Check if user has already seen onboarding
    const hasSeen = hasSeenOnboarding(userId);
    setShowOnboarding(!hasSeen);
  }, [userId]);

  const dismissOnboarding = useCallback((dontShowAgain = true) => {
    setShowOnboarding(false);
    if (dontShowAgain && userId) {
      markOnboardingSeen(userId);
    }
  }, [userId]);

  const resetOnboarding = useCallback(() => {
    if (!userId) return;
    localStorage.removeItem(getStorageKey(userId));
    setShowOnboarding(true);
  }, [userId]);

  return {
    showOnboarding,
    dismissOnboarding,
    resetOnboarding,
  };
};
