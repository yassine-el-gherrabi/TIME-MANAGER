import { describe, it, expect, vi, beforeEach } from 'vitest';
import axios, { AxiosError } from 'axios';
import { transformToApiError, handleError, getErrorMessage } from './errorHandler';
import { ApiError, ErrorType } from '@/types/errors';

// Mock sonner toast
vi.mock('sonner', () => ({
  toast: {
    error: vi.fn(),
    warning: vi.fn(),
    info: vi.fn(),
    success: vi.fn(),
  },
}));

describe('errorHandler', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('transformToApiError', () => {
    it('returns ApiError unchanged', () => {
      const apiError = new ApiError(ErrorType.VALIDATION, 400, 'Test error');
      const result = transformToApiError(apiError);

      expect(result).toBe(apiError);
      expect(result.type).toBe(ErrorType.VALIDATION);
    });

    it('transforms Axios 401 error correctly', () => {
      const axiosError = {
        response: {
          status: 401,
          data: { message: 'Invalid credentials' },
        },
        message: 'Request failed',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result).toBeInstanceOf(ApiError);
      expect(result.type).toBe(ErrorType.AUTHENTICATION);
      expect(result.statusCode).toBe(401);
      expect(result.message).toBe('Invalid credentials');
    });

    it('transforms Axios 403 error correctly', () => {
      const axiosError = {
        response: {
          status: 403,
          data: { error: 'Access denied' },
        },
        message: 'Forbidden',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.type).toBe(ErrorType.AUTHORIZATION);
      expect(result.statusCode).toBe(403);
    });

    it('transforms Axios 404 error correctly', () => {
      const axiosError = {
        response: {
          status: 404,
          data: {},
        },
        message: 'Not found',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.type).toBe(ErrorType.NOT_FOUND);
      expect(result.statusCode).toBe(404);
    });

    it('transforms Axios 500 error as retryable', () => {
      const axiosError = {
        response: {
          status: 500,
          data: { message: 'Internal server error' },
        },
        message: 'Server error',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.type).toBe(ErrorType.SERVER_ERROR);
      expect(result.statusCode).toBe(500);
      expect(result.retryable).toBe(true);
    });

    it('transforms Axios 503 error as retryable', () => {
      const axiosError = {
        response: {
          status: 503,
          data: {},
        },
        message: 'Service unavailable',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.type).toBe(ErrorType.SERVER_ERROR);
      expect(result.statusCode).toBe(503);
      expect(result.retryable).toBe(true);
    });

    it('handles Axios error without response', () => {
      const axiosError = {
        response: undefined,
        message: 'Network Error',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.type).toBe(ErrorType.UNKNOWN);
      expect(result.statusCode).toBe(0);
    });

    it('handles network error', () => {
      const networkError = new Error('Network Error');
      vi.spyOn(axios, 'isAxiosError').mockReturnValue(false);

      const result = transformToApiError(networkError);

      expect(result.type).toBe(ErrorType.NETWORK_ERROR);
      expect(result.statusCode).toBe(0);
      expect(result.message).toContain('Network connection failed');
      expect(result.retryable).toBe(true);
    });

    it('handles generic JavaScript error', () => {
      const genericError = new Error('Something went wrong');
      vi.spyOn(axios, 'isAxiosError').mockReturnValue(false);

      const result = transformToApiError(genericError);

      expect(result.type).toBe(ErrorType.UNKNOWN);
      expect(result.message).toBe('Something went wrong');
    });

    it('handles unknown error type', () => {
      const unknownError = { weird: 'object' };
      vi.spyOn(axios, 'isAxiosError').mockReturnValue(false);

      const result = transformToApiError(unknownError);

      expect(result.type).toBe(ErrorType.UNKNOWN);
      expect(result.message).toBe('An unexpected error occurred');
    });

    it('prefers response.data.message over response.data.error', () => {
      const axiosError = {
        response: {
          status: 400,
          data: {
            message: 'Validation failed',
            error: 'Bad request',
          },
        },
        message: 'Request failed',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.message).toBe('Validation failed');
    });

    it('uses response.data.error when message not available', () => {
      const axiosError = {
        response: {
          status: 400,
          data: {
            error: 'Bad request',
          },
        },
        message: 'Request failed',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const result = transformToApiError(axiosError);

      expect(result.message).toBe('Bad request');
    });
  });

  describe('handleError', () => {
    it('transforms, logs, and notifies on error', () => {
      const error = new Error('Test error');
      const consoleSpy = vi.spyOn(console, 'group').mockImplementation(() => {});
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const result = handleError(error, 'TestContext');

      expect(result).toBeInstanceOf(ApiError);
      expect(consoleSpy).toHaveBeenCalled();
      expect(consoleErrorSpy).toHaveBeenCalled();

      consoleSpy.mockRestore();
      consoleErrorSpy.mockRestore();
    });

    it('includes context in logging', () => {
      const error = new Error('Test error');
      const consoleGroupSpy = vi.spyOn(console, 'group').mockImplementation(() => {});

      handleError(error, 'Login');

      // Verify console.group was called with context
      expect(consoleGroupSpy).toHaveBeenCalledWith(expect.stringContaining('Login'));

      consoleGroupSpy.mockRestore();
    });
  });

  describe('getErrorMessage', () => {
    it('extracts message from ApiError', () => {
      const apiError = new ApiError(ErrorType.VALIDATION, 400, 'Validation failed');

      const message = getErrorMessage(apiError);

      expect(message).toBe('Validation failed');
    });

    it('extracts message from Axios error', () => {
      const axiosError = {
        response: {
          status: 401,
          data: { message: 'Unauthorized access' },
        },
        message: 'Request failed',
      } as AxiosError;

      vi.spyOn(axios, 'isAxiosError').mockReturnValue(true);

      const message = getErrorMessage(axiosError);

      expect(message).toBe('Unauthorized access');
    });

    it('extracts message from generic Error', () => {
      const error = new Error('Something broke');
      vi.spyOn(axios, 'isAxiosError').mockReturnValue(false);

      const message = getErrorMessage(error);

      expect(message).toBe('Something broke');
    });

    it('returns default message for unknown error', () => {
      vi.spyOn(axios, 'isAxiosError').mockReturnValue(false);

      const message = getErrorMessage({ weird: 'object' });

      expect(message).toBe('An unexpected error occurred');
    });
  });

  describe('ApiError class methods', () => {
    it('canRetry returns correct retryable status', () => {
      const retryableError = new ApiError(ErrorType.SERVER_ERROR, 500, 'Server error', {}, true);
      const nonRetryableError = new ApiError(ErrorType.VALIDATION, 400, 'Bad request', {}, false);

      expect(retryableError.canRetry()).toBe(true);
      expect(nonRetryableError.canRetry()).toBe(false);
    });

    it('getUserMessage returns message', () => {
      const error = new ApiError(ErrorType.VALIDATION, 400, 'Invalid input');

      expect(error.getUserMessage()).toBe('Invalid input');
    });

    it('getUserMessage returns default for empty message', () => {
      const error = new ApiError(ErrorType.UNKNOWN, 0, '');

      expect(error.getUserMessage()).toBe('An unexpected error occurred');
    });

    it('toJSON serializes error correctly', () => {
      const error = new ApiError(
        ErrorType.VALIDATION,
        400,
        'Validation failed',
        { field: 'email' },
        false
      );

      const json = error.toJSON();

      expect(json).toEqual({
        type: ErrorType.VALIDATION,
        statusCode: 400,
        message: 'Validation failed',
        details: { field: 'email' },
        retryable: false,
        stack: expect.any(String),
      });
    });
  });
});
