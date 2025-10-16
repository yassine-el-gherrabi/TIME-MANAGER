/**
 * Error Types for API and Business Logic Errors
 */
export const ErrorType = {
  AUTHENTICATION: 'authentication', // Invalid credentials
  AUTHORIZATION: 'authorization', // Insufficient permissions
  VALIDATION: 'validation', // Invalid input data
  NOT_FOUND: 'not_found', // Resource not found
  SERVER_ERROR: 'server_error', // Server-side error
  NETWORK_ERROR: 'network_error', // Network/connectivity issue
  UNKNOWN: 'unknown', // Unclassified error
} as const;

export type ErrorType = (typeof ErrorType)[keyof typeof ErrorType];

/**
 * Custom API Error Class
 * Extends native Error with additional context
 */
export class ApiError extends Error {
  type: ErrorType;
  statusCode: number;
  details?: Record<string, unknown>;
  retryable: boolean;

  constructor(
    type: ErrorType,
    statusCode: number,
    message: string,
    details?: Record<string, unknown>,
    retryable: boolean = false
  ) {
    super(message);
    this.name = 'ApiError';
    this.type = type;
    this.statusCode = statusCode;
    this.details = details;
    this.retryable = retryable;

    // Maintains proper stack trace for where error was thrown
    Object.setPrototypeOf(this, ApiError.prototype);
  }

  /**
   * Check if error is retryable
   */
  canRetry(): boolean {
    return this.retryable;
  }

  /**
   * Get user-friendly message
   */
  getUserMessage(): string {
    return this.message || 'An unexpected error occurred';
  }

  /**
   * Convert to JSON for logging/monitoring
   */
  toJSON() {
    return {
      type: this.type,
      statusCode: this.statusCode,
      message: this.message,
      details: this.details,
      retryable: this.retryable,
      stack: this.stack,
    };
  }
}

/**
 * Type guard to check if error is ApiError
 */
export function isApiError(error: unknown): error is ApiError {
  return error instanceof ApiError;
}

/**
 * Type guard to check if error is network error
 */
export function isNetworkError(error: unknown): boolean {
  if (isApiError(error)) {
    return error.type === ErrorType.NETWORK_ERROR;
  }
  return false;
}

/**
 * Type guard to check if error is authentication error
 */
export function isAuthError(error: unknown): boolean {
  if (isApiError(error)) {
    return error.type === ErrorType.AUTHENTICATION;
  }
  return false;
}
