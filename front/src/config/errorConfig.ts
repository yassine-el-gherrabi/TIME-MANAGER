import { ErrorType } from '@/types/errors';

/**
 * HTTP Status Code to Error Type Mapping
 */
export const ERROR_MAPPING: Record<
  number,
  { type: ErrorType; retryable: boolean; message?: string }
> = {
  // 4xx - Client Errors
  400: {
    type: ErrorType.VALIDATION,
    retryable: false,
    message: 'Invalid request. Please check your input.',
  },
  401: {
    type: ErrorType.AUTHENTICATION,
    retryable: false,
    message: 'Authentication failed. Please check your credentials.',
  },
  403: {
    type: ErrorType.AUTHORIZATION,
    retryable: false,
    message: 'Access denied. You do not have permission.',
  },
  404: {
    type: ErrorType.NOT_FOUND,
    retryable: false,
    message: 'Resource not found.',
  },
  422: {
    type: ErrorType.VALIDATION,
    retryable: false,
    message: 'Validation failed. Please check your input.',
  },
  429: {
    type: ErrorType.VALIDATION,
    retryable: true, // Can retry after rate limit cooldown
    message: 'Too many requests. Please try again later.',
  },

  // 5xx - Server Errors
  500: {
    type: ErrorType.SERVER_ERROR,
    retryable: true,
    message: 'Server error. Please try again later.',
  },
  502: {
    type: ErrorType.SERVER_ERROR,
    retryable: true,
    message: 'Bad gateway. Please try again later.',
  },
  503: {
    type: ErrorType.SERVER_ERROR,
    retryable: true,
    message: 'Service unavailable. Please try again later.',
  },
  504: {
    type: ErrorType.SERVER_ERROR,
    retryable: true,
    message: 'Gateway timeout. Please try again later.',
  },
};

/**
 * Public routes where authentication errors should NOT trigger logout
 */
export const PUBLIC_ROUTES = ['/login', '/register', '/forgot-password', '/reset-password'];

/**
 * Retry configuration
 */
export const RETRY_CONFIG = {
  maxRetries: 3,
  initialDelayMs: 1000,
  backoffMultiplier: 2, // Exponential backoff: 1s, 2s, 4s
  maxDelayMs: 10000, // Maximum delay between retries
};

/**
 * Error notification configuration
 */
export const NOTIFICATION_CONFIG = {
  duration: {
    error: 5000,
    warning: 4000,
    success: 3000,
    info: 3000,
  },
  position: 'top-right' as const,
};

/**
 * Check if route is public (no auth required)
 */
export function isPublicRoute(path: string): boolean {
  return PUBLIC_ROUTES.some((route) => path.startsWith(route));
}

/**
 * Check if error should trigger logout
 */
export function shouldLogoutOn401(path: string, hasToken: boolean): boolean {
  // Don't logout if on public route
  if (isPublicRoute(path)) {
    return false;
  }

  // Only logout if user has a token (means they were authenticated)
  return hasToken;
}
