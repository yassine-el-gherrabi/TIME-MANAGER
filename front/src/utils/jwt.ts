/**
 * JWT Token Utilities
 *
 * Provides utilities for JWT token validation and expiration checking.
 */

interface JWTPayload {
  uid: number;
  exp: number;
  iat: number;
}

/**
 * Decode JWT token payload
 *
 * @param token - JWT token string
 * @returns Decoded payload or null if invalid
 */
export function decodeJWT(token: string): JWTPayload | null {
  try {
    // JWT format: header.payload.signature
    const parts = token.split('.');
    if (parts.length !== 3) {
      return null;
    }

    // Decode base64url payload (middle part)
    const payload = parts[1];
    const decoded = atob(payload.replace(/-/g, '+').replace(/_/g, '/'));
    return JSON.parse(decoded);
  } catch (error) {
    console.error('Failed to decode JWT:', error);
    return null;
  }
}

/**
 * Check if JWT token is expired
 *
 * @param token - JWT token string
 * @returns true if token is expired or invalid, false otherwise
 */
export function isTokenExpired(token: string | null): boolean {
  if (!token) {
    return true;
  }

  const payload = decodeJWT(token);
  if (!payload || !payload.exp) {
    return true;
  }

  // exp is in seconds, convert to milliseconds
  const expirationTime = payload.exp * 1000;
  const currentTime = Date.now();

  return currentTime >= expirationTime;
}

/**
 * Get time remaining until token expiration
 *
 * @param token - JWT token string
 * @returns Milliseconds until expiration, or 0 if expired/invalid
 */
export function getTokenTimeRemaining(token: string | null): number {
  if (!token) {
    return 0;
  }

  const payload = decodeJWT(token);
  if (!payload || !payload.exp) {
    return 0;
  }

  const expirationTime = payload.exp * 1000;
  const currentTime = Date.now();
  const remaining = expirationTime - currentTime;

  return remaining > 0 ? remaining : 0;
}

/**
 * Check if token will expire soon
 *
 * @param token - JWT token string
 * @param thresholdMinutes - Minutes threshold to consider "soon" (default: 5)
 * @returns true if token expires within threshold
 */
export function isTokenExpiringSoon(token: string | null, thresholdMinutes: number = 5): boolean {
  const remaining = getTokenTimeRemaining(token);
  const thresholdMs = thresholdMinutes * 60 * 1000;

  return remaining > 0 && remaining <= thresholdMs;
}
