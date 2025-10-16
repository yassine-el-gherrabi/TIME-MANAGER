import axios, { AxiosError, type InternalAxiosRequestConfig } from 'axios';
import { toCamelCase, toSnakeCase } from '@/utils/transformers';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api';

export const apiClient = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

/**
 * Clear authentication data from localStorage
 */
export function clearAuthData(): void {
  localStorage.removeItem('token');
  localStorage.removeItem('user');
}

/**
 * Request interceptor
 * - Adds JWT token from localStorage to Authorization header
 * - Transforms request data from camelCase to snake_case
 */
apiClient.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    // Add JWT token if available
    const token = localStorage.getItem('token');
    if (token && config.headers) {
      config.headers.Authorization = `Bearer ${token}`;
    }

    // Transform request data to snake_case
    if (config.data && typeof config.data === 'object') {
      config.data = toSnakeCase(config.data);
    }

    return config;
  },
  (error: AxiosError) => {
    return Promise.reject(error);
  }
);

/**
 * Response interceptor
 * - Transforms response data from snake_case to camelCase
 */
apiClient.interceptors.response.use(
  (response) => {
    // Transform response data to camelCase
    if (response.data && typeof response.data === 'object') {
      response.data = toCamelCase(response.data);
    }
    return response;
  },
  (error: AxiosError) => {
    return Promise.reject(error);
  }
);

/**
 * @deprecated Use handleError from @/utils/errorHandler instead
 * This function is kept for backward compatibility only
 */
export { getErrorMessage as handleApiError } from '@/utils/errorHandler';
