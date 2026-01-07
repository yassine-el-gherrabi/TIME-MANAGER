/**
 * HTTP Client Configuration
 *
 * Axios-based HTTP client with authentication interceptors,
 * automatic token refresh, and error handling.
 */

import axios, { AxiosInstance, AxiosRequestConfig, AxiosError, InternalAxiosRequestConfig } from 'axios';
import { API_URL, STORAGE_KEYS, TOKEN_CONFIG } from '../config/constants';
import type { ApiError, TokenPair } from '../types/auth';

/**
 * Custom error class for API errors
 */
export class ApiErrorClass extends Error {
  public status?: number;
  public details?: string[];

  constructor(message: string, status?: number, details?: string[]) {
    super(message);
    this.name = 'ApiError';
    this.status = status;
    this.details = details;
  }
}

/**
 * Token management in memory (not localStorage for access tokens)
 */
class TokenManager {
  private accessToken: string | null = null;
  private tokenExpiry: number | null = null;

  setAccessToken(token: string): void {
    this.accessToken = token;
    // Calculate expiry time (access token valid for 15 minutes)
    this.tokenExpiry = Date.now() + TOKEN_CONFIG.ACCESS_TOKEN_LIFETIME;
  }

  getAccessToken(): string | null {
    // Check if token is expired
    if (this.tokenExpiry && Date.now() >= this.tokenExpiry) {
      this.clearAccessToken();
      return null;
    }
    return this.accessToken;
  }

  clearAccessToken(): void {
    this.accessToken = null;
    this.tokenExpiry = null;
  }

  shouldRefresh(): boolean {
    if (!this.tokenExpiry) return false;
    // Refresh if token expires within threshold (2 minutes)
    return Date.now() >= this.tokenExpiry - TOKEN_CONFIG.REFRESH_THRESHOLD;
  }
}

const tokenManager = new TokenManager();

/**
 * Get refresh token from localStorage
 */
export const getRefreshToken = (): string | null => {
  return localStorage.getItem(STORAGE_KEYS.REFRESH_TOKEN);
};

/**
 * Set refresh token in localStorage
 */
export const setRefreshToken = (token: string): void => {
  localStorage.setItem(STORAGE_KEYS.REFRESH_TOKEN, token);
};

/**
 * Remove refresh token from localStorage
 */
export const removeRefreshToken = (): void => {
  localStorage.removeItem(STORAGE_KEYS.REFRESH_TOKEN);
};

/**
 * Set token pair (access in memory, refresh in localStorage)
 */
export const setTokens = (tokens: TokenPair): void => {
  tokenManager.setAccessToken(tokens.access_token);
  setRefreshToken(tokens.refresh_token);
};

/**
 * Clear all tokens
 */
export const clearTokens = (): void => {
  tokenManager.clearAccessToken();
  removeRefreshToken();
  localStorage.removeItem(STORAGE_KEYS.USER);
};

/**
 * Create base axios instance
 */
const createApiClient = (): AxiosInstance => {
  const client = axios.create({
    baseURL: API_URL,
    timeout: 30000,
    headers: {
      'Content-Type': 'application/json',
    },
  });

  /**
   * Request interceptor - add access token to headers
   */
  client.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
      const token = tokenManager.getAccessToken();
      if (token && config.headers) {
        config.headers.Authorization = `Bearer ${token}`;
      }
      return config;
    },
    (error) => {
      return Promise.reject(error);
    }
  );

  /**
   * Response interceptor - handle token refresh and errors
   */
  client.interceptors.response.use(
    (response) => response,
    async (error: AxiosError<ApiError>) => {
      const originalRequest = error.config as InternalAxiosRequestConfig & { _retry?: boolean };

      // Handle 401 Unauthorized - attempt token refresh
      if (error.response?.status === 401 && !originalRequest._retry) {
        originalRequest._retry = true;

        try {
          const refreshToken = getRefreshToken();
          if (!refreshToken) {
            throw new Error('No refresh token available');
          }

          // Attempt to refresh token
          const response = await axios.post<{ tokens: TokenPair }>(
            `${API_URL}/auth/refresh`,
            { refresh_token: refreshToken }
          );

          const { tokens } = response.data;
          setTokens(tokens);

          // Retry original request with new token
          if (originalRequest.headers) {
            originalRequest.headers.Authorization = `Bearer ${tokens.access_token}`;
          }
          return client(originalRequest);
        } catch (refreshError) {
          // Refresh failed - clear tokens and redirect to login
          clearTokens();
          window.location.href = '/login';
          return Promise.reject(refreshError);
        }
      }

      // Transform error to ApiError
      const apiError = new ApiErrorClass(
        error.response?.data?.error || error.message || 'An error occurred',
        error.response?.status,
        error.response?.data?.details
      );

      return Promise.reject(apiError);
    }
  );

  return client;
};

/**
 * Main API client instance
 */
export const apiClient = createApiClient();

/**
 * Export token manager for use in auth store
 */
export { tokenManager };

/**
 * Generic API request wrapper with type safety
 */
export const apiRequest = async <T>(
  config: AxiosRequestConfig
): Promise<T> => {
  const response = await apiClient.request<T>(config);
  return response.data;
};
