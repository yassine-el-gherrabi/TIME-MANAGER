import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import type { InternalAxiosRequestConfig, AxiosResponse, AxiosError, AxiosHeaders } from 'axios';
import { apiClient, clearAuthData } from './client';

describe('apiClient', () => {
  beforeEach(() => {
    localStorage.clear();
    vi.clearAllMocks();
  });

  afterEach(() => {
    localStorage.clear();
  });

  describe('configuration', () => {
    it('has correct base URL from environment', () => {
      expect(apiClient.defaults.baseURL).toBeDefined();
      expect(typeof apiClient.defaults.baseURL).toBe('string');
    });

    it('has correct default headers', () => {
      expect(apiClient.defaults.headers['Content-Type']).toBe('application/json');
    });
  });

  describe('clearAuthData', () => {
    it('removes token from localStorage', () => {
      localStorage.setItem('token', 'test-token');

      clearAuthData();

      expect(localStorage.getItem('token')).toBeNull();
    });

    it('handles empty localStorage gracefully', () => {
      expect(() => clearAuthData()).not.toThrow();
    });

    it('only removes token, not other data', () => {
      localStorage.setItem('token', 'test-token');
      localStorage.setItem('other-data', 'some-value');

      clearAuthData();

      expect(localStorage.getItem('token')).toBeNull();
      expect(localStorage.getItem('other-data')).toBe('some-value');
    });
  });

  describe('request interceptor', () => {
    it('adds Authorization header when token exists', () => {
      localStorage.setItem('token', 'jwt-token-123');

      const config: InternalAxiosRequestConfig = {
        headers: {} as AxiosHeaders,
      } as InternalAxiosRequestConfig;

      const interceptor = apiClient.interceptors.request.handlers[0];
      const result = (
        interceptor as {
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }
      ).fulfilled(config);

      expect(result.headers.Authorization).toBe('Bearer jwt-token-123');
    });

    it('does not add Authorization header when no token', () => {
      const config: InternalAxiosRequestConfig = {
        headers: {} as AxiosHeaders,
      } as InternalAxiosRequestConfig;

      const interceptor = apiClient.interceptors.request.handlers[0];
      const result = (
        interceptor as {
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }
      ).fulfilled(config);

      expect(result.headers.Authorization).toBeUndefined();
    });

    it('transforms request data to snake_case', () => {
      const config: InternalAxiosRequestConfig = {
        headers: {} as AxiosHeaders,
        data: {
          firstName: 'John',
          lastName: 'Doe',
          emailAddress: 'john@example.com',
        },
      } as InternalAxiosRequestConfig;

      const interceptor = apiClient.interceptors.request.handlers[0];
      const result = (
        interceptor as {
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }
      ).fulfilled(config);

      expect(result.data).toEqual({
        first_name: 'John',
        last_name: 'Doe',
        email_address: 'john@example.com',
      });
    });

    it('handles null data gracefully', () => {
      const config: InternalAxiosRequestConfig = {
        headers: {} as AxiosHeaders,
        data: null,
      } as InternalAxiosRequestConfig;

      const interceptor = apiClient.interceptors.request.handlers[0];
      const result = (
        interceptor as {
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }
      ).fulfilled(config);

      expect(result.data).toBeNull();
    });

    it('handles non-object data gracefully', () => {
      const config: InternalAxiosRequestConfig = {
        headers: {} as AxiosHeaders,
        data: 'string data',
      } as InternalAxiosRequestConfig;

      const interceptor = apiClient.interceptors.request.handlers[0];
      const result = (
        interceptor as {
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }
      ).fulfilled(config);

      expect(result.data).toBe('string data');
    });

    it('handles request errors', async () => {
      const error: AxiosError = new Error('Request error') as AxiosError;

      const interceptor = apiClient.interceptors.request.handlers[0];

      await expect(
        (interceptor as { rejected: (error: AxiosError) => Promise<never> }).rejected(error)
      ).rejects.toThrow('Request error');
    });

    it('transforms nested objects to snake_case', () => {
      const config: InternalAxiosRequestConfig = {
        headers: {} as AxiosHeaders,
        data: {
          userInfo: {
            firstName: 'John',
            phoneNumber: '123',
          },
        },
      } as InternalAxiosRequestConfig;

      const interceptor = apiClient.interceptors.request.handlers[0];
      const result = (
        interceptor as {
          fulfilled: (config: InternalAxiosRequestConfig) => InternalAxiosRequestConfig;
        }
      ).fulfilled(config);

      expect(result.data).toEqual({
        user_info: {
          first_name: 'John',
          phone_number: '123',
        },
      });
    });
  });

  describe('response interceptor', () => {
    it('transforms response data to camelCase', () => {
      const response: AxiosResponse = {
        data: {
          first_name: 'John',
          last_name: 'Doe',
          email_address: 'john@example.com',
        },
        status: 200,
        statusText: 'OK',
        headers: {},
        config: {} as InternalAxiosRequestConfig,
      };

      const interceptor = apiClient.interceptors.response.handlers[0];
      const result = (
        interceptor as { fulfilled: (response: AxiosResponse) => AxiosResponse }
      ).fulfilled(response);

      expect(result.data).toEqual({
        firstName: 'John',
        lastName: 'Doe',
        emailAddress: 'john@example.com',
      });
    });

    it('handles null response data gracefully', () => {
      const response: AxiosResponse = {
        data: null,
        status: 204,
        statusText: 'No Content',
        headers: {},
        config: {} as InternalAxiosRequestConfig,
      };

      const interceptor = apiClient.interceptors.response.handlers[0];
      const result = (
        interceptor as { fulfilled: (response: AxiosResponse) => AxiosResponse }
      ).fulfilled(response);

      expect(result.data).toBeNull();
    });

    it('handles non-object response data gracefully', () => {
      const response: AxiosResponse = {
        data: 'string response',
        status: 200,
        statusText: 'OK',
        headers: {},
        config: {} as InternalAxiosRequestConfig,
      };

      const interceptor = apiClient.interceptors.response.handlers[0];
      const result = (
        interceptor as { fulfilled: (response: AxiosResponse) => AxiosResponse }
      ).fulfilled(response);

      expect(result.data).toBe('string response');
    });

    it('handles response errors', async () => {
      const error: AxiosError = new Error('Response error') as AxiosError;

      const interceptor = apiClient.interceptors.response.handlers[0];

      await expect(
        (interceptor as { rejected: (error: AxiosError) => Promise<never> }).rejected(error)
      ).rejects.toThrow('Response error');
    });

    it('transforms nested objects to camelCase', () => {
      const response: AxiosResponse = {
        data: {
          user_info: {
            first_name: 'John',
            phone_number: '123',
          },
        },
        status: 200,
        statusText: 'OK',
        headers: {},
        config: {} as InternalAxiosRequestConfig,
      };

      const interceptor = apiClient.interceptors.response.handlers[0];
      const result = (
        interceptor as { fulfilled: (response: AxiosResponse) => AxiosResponse }
      ).fulfilled(response);

      expect(result.data).toEqual({
        userInfo: {
          firstName: 'John',
          phoneNumber: '123',
        },
      });
    });

    it('transforms arrays of objects to camelCase', () => {
      const response: AxiosResponse = {
        data: [
          { first_name: 'John', last_name: 'Doe' },
          { first_name: 'Jane', last_name: 'Smith' },
        ],
        status: 200,
        statusText: 'OK',
        headers: {},
        config: {} as InternalAxiosRequestConfig,
      };

      const interceptor = apiClient.interceptors.response.handlers[0];
      const result = (
        interceptor as { fulfilled: (response: AxiosResponse) => AxiosResponse }
      ).fulfilled(response);

      expect(result.data).toEqual([
        { firstName: 'John', lastName: 'Doe' },
        { firstName: 'Jane', lastName: 'Smith' },
      ]);
    });
  });
});
