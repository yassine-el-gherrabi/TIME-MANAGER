/**
 * HTTP Client Configuration
 *
 * Axios-based HTTP client with authentication interceptors,
 * automatic token refresh, and error handling.
 */

import axios, { AxiosInstance, AxiosRequestConfig, AxiosError, InternalAxiosRequestConfig } from 'axios';
import { API_URL, TOKEN_CONFIG } from '../config/constants';
import type { ApiError } from '../types/auth';

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
 * Get CSRF token from cookie
 * Used for double submit cookie CSRF protection
 */
const getCsrfToken = (): string | null => {
  const value = `; ${document.cookie}`;
  const parts = value.split('; csrf_token=');
  if (parts.length === 2) {
    return parts.pop()?.split(';').shift() || null;
  }
  return null;
};

/**
 * Set access token in memory (refresh token is now HttpOnly cookie)
 */
export const setTokens = (tokens: { access_token: string }): void => {
  tokenManager.setAccessToken(tokens.access_token);
};

/**
 * Clear access token from memory
 * Note: HttpOnly refresh_token and csrf_token cookies are cleared by server on logout
 */
export const clearTokens = (): void => {
  tokenManager.clearAccessToken();
};

/**
 * Check if user has a refresh token (by checking CSRF token presence as proxy)
 * Since refresh_token is HttpOnly, we can't read it directly
 */
export const hasRefreshToken = (): boolean => {
  return getCsrfToken() !== null;
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
    withCredentials: true, // Required for HttpOnly cookies
  });

  /**
   * Request interceptor - add access token and CSRF token to headers
   */
  client.interceptors.request.use(
    (config: InternalAxiosRequestConfig) => {
      // Add access token for authenticated requests
      const token = tokenManager.getAccessToken();
      if (token && config.headers) {
        config.headers.Authorization = `Bearer ${token}`;
      }

      // Add CSRF token for mutation requests (POST, PUT, DELETE, PATCH)
      const method = config.method?.toUpperCase();
      if (method && ['POST', 'PUT', 'DELETE', 'PATCH'].includes(method)) {
        const csrfToken = getCsrfToken();
        if (csrfToken && config.headers) {
          config.headers['X-CSRF-Token'] = csrfToken;
        }
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

      // Check if this is an auth endpoint that should NOT trigger token refresh
      // Login and refresh endpoints return 401 for invalid credentials, not expired tokens
      const isAuthEndpoint = originalRequest.url?.includes('/auth/login') ||
                             originalRequest.url?.includes('/auth/refresh');

      // Handle 401 Unauthorized - attempt token refresh (but not for auth endpoints)
      if (error.response?.status === 401 && !originalRequest._retry && !isAuthEndpoint) {
        originalRequest._retry = true;

        try {
          // Check if we have a refresh token (via CSRF token proxy)
          if (!hasRefreshToken()) {
            throw new Error('No refresh token available');
          }

          // Attempt to refresh token (refresh_token sent automatically as HttpOnly cookie)
          const response = await axios.post<{ access_token: string }>(
            `${API_URL}/auth/refresh`,
            {},
            { withCredentials: true }
          );

          const { access_token } = response.data;
          setTokens({ access_token });

          // Retry original request with new token
          if (originalRequest.headers) {
            originalRequest.headers.Authorization = `Bearer ${access_token}`;
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
      // Backend returns: { error: "ErrorType", message: "Generic message", details: "Specific message" }
      // Prefer details (specific), then message, then error type as fallback
      const details = error.response?.data?.details;
      const detailsMessage = typeof details === 'string' ? details : (Array.isArray(details) ? details[0] : undefined);
      const apiError = new ApiErrorClass(
        detailsMessage || error.response?.data?.message || error.response?.data?.error || error.message || 'An error occurred',
        error.response?.status,
        detailsMessage ? [detailsMessage] : undefined
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
