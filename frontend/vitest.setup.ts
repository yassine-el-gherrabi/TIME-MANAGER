import { expect, afterEach, vi } from 'vitest';
import { cleanup } from '@testing-library/react';
import * as matchers from '@testing-library/jest-dom/matchers';

// Extend Vitest's expect with jest-dom matchers
expect.extend(matchers);

// i18n translations for testing
const translations: Record<string, string> = {
  'auth.login': 'Login',
  'auth.signIn': 'Sign in',
  'auth.email': 'Email',
  'auth.password': 'Password',
  'auth.enterCredentials': 'Enter your credentials to access your account',
  'auth.forgotPassword': 'Forgot password?',
  'auth.loggingIn': 'Logging in...',
  'auth.welcomeBack': 'Welcome back',
  'common.loading': 'Logging in...',
  'validation.required': 'This field is required',
  'validation.invalidEmail': 'Invalid email format',
  'validation.passwordRequired': 'Password is required',
  'dashboard.title': 'Dashboard',
  'nav.dashboard': 'Dashboard',
};

// Mock react-i18next at module level
vi.mock('react-i18next', () => ({
  useTranslation: () => ({
    t: (key: string) => translations[key] || key,
    i18n: {
      changeLanguage: vi.fn().mockResolvedValue(undefined),
      language: 'en',
    },
  }),
  Trans: ({ children }: { children: React.ReactNode }) => children,
  initReactI18next: {
    type: '3rdParty',
    init: vi.fn(),
  },
  I18nextProvider: ({ children }: { children: React.ReactNode }) => children,
}));

// Cleanup after each test
afterEach(() => {
  cleanup();
});
