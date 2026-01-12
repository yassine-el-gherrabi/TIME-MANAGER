/**
 * Error Handling Utilities
 *
 * Maps API errors to user-friendly messages
 */

import { ERROR_MESSAGES } from '../config/constants';
import type { ApiErrorClass } from '../api/client';

/**
 * Maps HTTP status codes to user-friendly error messages
 */
export const mapErrorToMessage = (error: unknown): string => {
  // Handle ApiErrorClass instances
  if (error && typeof error === 'object' && 'status' in error) {
    const apiError = error as ApiErrorClass;
    const status = apiError.status;

    // Map status codes to friendly messages
    switch (status) {
      case 400:
        return ERROR_MESSAGES.VALIDATION_ERROR;

      case 401:
        // For login errors, use the backend message if available
        if (apiError.message && apiError.message !== 'Unauthorized') {
          return apiError.message;
        }
        return ERROR_MESSAGES.UNAUTHORIZED;

      case 403:
        return ERROR_MESSAGES.FORBIDDEN;

      case 404:
        // 404 on auth endpoints likely means backend not implemented yet
        return 'Service temporarily unavailable. Please try again later.';

      case 429:
        return ERROR_MESSAGES.RATE_LIMIT;

      case 500:
      case 502:
      case 503:
      case 504:
        return ERROR_MESSAGES.SERVER_ERROR;

      default:
        // If backend provides a custom error message, use it
        if (apiError.message && !apiError.message.includes('status code')) {
          return apiError.message;
        }
        return ERROR_MESSAGES.NETWORK_ERROR;
    }
  }

  // Handle Error instances
  if (error instanceof Error) {
    // Network errors (no response from server)
    if (error.message.includes('Network Error') || error.message.includes('ECONNREFUSED')) {
      return ERROR_MESSAGES.NETWORK_ERROR;
    }

    // Timeout errors
    if (error.message.includes('timeout')) {
      return 'Request timeout. Please check your connection and try again.';
    }

    // Don't show technical error messages to users
    if (error.message.includes('status code')) {
      return ERROR_MESSAGES.SERVER_ERROR;
    }

    // If it's a user-friendly message, show it
    return error.message;
  }

  // Fallback for unknown errors
  return 'An unexpected error occurred. Please try again.';
};

/**
 * Extract validation errors from API response
 */
export const extractValidationErrors = (error: unknown): Record<string, string> | null => {
  if (error && typeof error === 'object' && 'details' in error) {
    const apiError = error as ApiErrorClass;

    if (apiError.details && Array.isArray(apiError.details)) {
      // Transform array of error messages to field -> message map
      const fieldErrors: Record<string, string> = {};

      apiError.details.forEach((detail) => {
        // Expected format: "field: error message" or just "error message"
        const match = detail.match(/^(\w+):\s*(.+)$/);
        if (match) {
          const [, field, message] = match;
          fieldErrors[field] = message;
        }
      });

      return Object.keys(fieldErrors).length > 0 ? fieldErrors : null;
    }
  }

  return null;
};
