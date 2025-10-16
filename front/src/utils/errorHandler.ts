import axios, { type AxiosError } from 'axios';
import { toast } from 'sonner';
import { ApiError, ErrorType } from '@/types/errors';
import { ERROR_MAPPING, NOTIFICATION_CONFIG } from '@/config/errorConfig';

/**
 * Transform any error into ApiError
 */
export function transformToApiError(error: unknown): ApiError {
  // Already an ApiError
  if (error instanceof ApiError) {
    return error;
  }

  // Axios HTTP error
  if (axios.isAxiosError(error)) {
    return transformAxiosError(error);
  }

  // Network error (no response)
  if (error instanceof Error && error.message === 'Network Error') {
    return new ApiError(
      ErrorType.NETWORK_ERROR,
      0,
      'Network connection failed. Please check your internet connection.',
      undefined,
      true
    );
  }

  // Generic JavaScript error
  if (error instanceof Error) {
    return new ApiError(ErrorType.UNKNOWN, 0, error.message || 'An unexpected error occurred', {
      originalError: error.stack,
    });
  }

  // Unknown error type
  return new ApiError(ErrorType.UNKNOWN, 0, 'An unexpected error occurred', { error });
}

/**
 * Transform Axios error into ApiError
 */
function transformAxiosError(error: AxiosError): ApiError {
  const status = error.response?.status || 0;
  const responseData = error.response?.data as Record<string, unknown> | undefined;

  // Get error mapping for this status code
  const mapping = ERROR_MAPPING[status];

  if (mapping) {
    // Extract message from response or use default
    const message =
      (typeof responseData?.message === 'string' ? responseData.message : '') ||
      (typeof responseData?.error === 'string' ? responseData.error : '') ||
      mapping.message ||
      error.message;

    return new ApiError(mapping.type, status, message, responseData, mapping.retryable);
  }

  // Unmapped status code
  const errorType = status >= 500 ? ErrorType.SERVER_ERROR : ErrorType.UNKNOWN;

  return new ApiError(
    errorType,
    status,
    (typeof responseData?.message === 'string' ? responseData.message : '') ||
      error.message ||
      'An error occurred',
    responseData,
    status >= 500 // Server errors are retryable
  );
}

/**
 * Show user notification based on error type
 */
export function showUserNotification(error: ApiError, context?: string): void {
  const contextPrefix = context ? `[${context}] ` : '';

  switch (error.type) {
    case ErrorType.AUTHENTICATION:
      toast.error(`${contextPrefix}${error.getUserMessage()}`, {
        description: 'Please check your credentials',
        duration: NOTIFICATION_CONFIG.duration.error,
      });
      break;

    case ErrorType.AUTHORIZATION:
      toast.error(`${contextPrefix}Access Denied`, {
        description: error.getUserMessage(),
        duration: NOTIFICATION_CONFIG.duration.error,
      });
      break;

    case ErrorType.VALIDATION:
      toast.warning(`${contextPrefix}Validation Error`, {
        description: error.getUserMessage(),
        duration: NOTIFICATION_CONFIG.duration.warning,
      });
      break;

    case ErrorType.NOT_FOUND:
      toast.warning(`${contextPrefix}Not Found`, {
        description: error.getUserMessage(),
        duration: NOTIFICATION_CONFIG.duration.warning,
      });
      break;

    case ErrorType.SERVER_ERROR:
      toast.error(`${contextPrefix}Server Error`, {
        description: error.getUserMessage(),
        duration: NOTIFICATION_CONFIG.duration.error,
        action: error.canRetry()
          ? {
              label: 'Retry',
              onClick: () => {
                // Retry logic will be handled by the caller
                toast.info('Retrying...');
              },
            }
          : undefined,
      });
      break;

    case ErrorType.NETWORK_ERROR:
      toast.error(`${contextPrefix}Connection Error`, {
        description: error.getUserMessage(),
        duration: NOTIFICATION_CONFIG.duration.error,
      });
      break;

    default:
      toast.error(`${contextPrefix}Error`, {
        description: error.getUserMessage(),
        duration: NOTIFICATION_CONFIG.duration.error,
      });
  }
}

/**
 * Log error for debugging (in development) or monitoring (in production)
 */
export function logError(error: ApiError, context?: string): void {
  if (import.meta.env.DEV) {
    // Development: Console logging
    console.group(`🔴 Error ${context ? `[${context}]` : ''}`);
    console.error('Type:', error.type);
    console.error('Status:', error.statusCode);
    console.error('Message:', error.message);
    if (error.details) {
      console.error('Details:', error.details);
    }
    console.error('Stack:', error.stack);
    console.groupEnd();
  } else {
    // Production: Send to monitoring service (Sentry, LogRocket, etc.)
    // Example: Sentry.captureException(error);
    // For now, we'll just log to console in production too
    console.error('[Error]', {
      context,
      type: error.type,
      status: error.statusCode,
      message: error.message,
    });
  }
}

/**
 * Report error to monitoring service (optional)
 */
export function reportToMonitoring(error: ApiError): void {
  // Only report server errors and unknown errors
  if (error.type === ErrorType.SERVER_ERROR || error.type === ErrorType.UNKNOWN) {
    // TODO: Integrate with Sentry, DataDog, LogRocket, etc.
    // Example:
    // Sentry.captureException(error, {
    //   tags: { context: _context, type: error.type },
    //   extra: error.details
    // });

    // For now, just log
    if (import.meta.env.PROD) {
      console.error('[Monitoring]', error.toJSON());
    }
  }
}

/**
 * Main error handler - centralized error handling logic
 */
export function handleError(error: unknown, context?: string): ApiError {
  // 1. Transform to ApiError
  const apiError = transformToApiError(error);

  // 2. Log the error
  logError(apiError, context);

  // 3. Show user notification
  showUserNotification(apiError, context);

  // 4. Report to monitoring if needed
  reportToMonitoring(apiError);

  return apiError;
}

/**
 * Extract user-friendly error message
 */
export function getErrorMessage(error: unknown): string {
  const apiError = transformToApiError(error);
  return apiError.getUserMessage();
}
