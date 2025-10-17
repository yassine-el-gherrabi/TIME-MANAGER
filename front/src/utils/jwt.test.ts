import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { decodeJWT, isTokenExpired, getTokenTimeRemaining, isTokenExpiringSoon } from './jwt';

// Helper to create a mock JWT token
function createMockToken(payload: Record<string, unknown>): string {
  const header = btoa(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const payloadEncoded = btoa(JSON.stringify(payload));
  const signature = 'mock-signature';
  return `${header}.${payloadEncoded}.${signature}`;
}

describe('jwt utilities', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2024-01-01T12:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('decodeJWT', () => {
    it('decodes a valid JWT token', () => {
      const payload = { uid: 123, exp: 1704110400, iat: 1704106800 };
      const token = createMockToken(payload);

      const decoded = decodeJWT(token);

      expect(decoded).toEqual(payload);
    });

    it('returns null for invalid token format', () => {
      expect(decodeJWT('invalid')).toBeNull();
      expect(decodeJWT('invalid.token')).toBeNull();
      expect(decodeJWT('')).toBeNull();
    });

    it('returns null for malformed base64', () => {
      const token = 'header.!!!invalid-base64!!!.signature';
      expect(decodeJWT(token)).toBeNull();
    });

    it('handles base64url encoding (- and _ characters)', () => {
      // Create payload with characters that need base64url encoding
      const payload = {
        uid: 123,
        exp: 1704110400,
        iat: 1704106800,
        data: 'test+data/with=special',
      };
      const payloadStr = JSON.stringify(payload);
      const payloadBase64url = btoa(payloadStr).replace(/\+/g, '-').replace(/\//g, '_');
      const token = `header.${payloadBase64url}.signature`;

      const decoded = decodeJWT(token);
      expect(decoded).toEqual(payload);
    });
  });

  describe('isTokenExpired', () => {
    it('returns true for null token', () => {
      expect(isTokenExpired(null)).toBe(true);
    });

    it('returns true for empty token', () => {
      expect(isTokenExpired('')).toBe(true);
    });

    it('returns true for invalid token', () => {
      expect(isTokenExpired('invalid-token')).toBe(true);
    });

    it('returns true for token without exp field', () => {
      const token = createMockToken({ uid: 123, iat: 1704106800 });
      expect(isTokenExpired(token)).toBe(true);
    });

    it('returns true for expired token', () => {
      // Current time: 2024-01-01T12:00:00Z (1704110400000)
      // Token exp: 2024-01-01T11:00:00Z (1704106800) - 1 hour ago
      const payload = { uid: 123, exp: 1704106800, iat: 1704103200 };
      const token = createMockToken(payload);

      expect(isTokenExpired(token)).toBe(true);
    });

    it('returns false for valid non-expired token', () => {
      // Current time: 2024-01-01T12:00:00Z (1704110400000)
      // Token exp: 2024-01-01T13:00:00Z (1704114000) - 1 hour from now
      const payload = { uid: 123, exp: 1704114000, iat: 1704110400 };
      const token = createMockToken(payload);

      expect(isTokenExpired(token)).toBe(false);
    });

    it('returns true for token expiring exactly now', () => {
      // Current time: 2024-01-01T12:00:00Z (1704110400000)
      // Token exp: 2024-01-01T12:00:00Z (1704110400) - exactly now
      const payload = { uid: 123, exp: 1704110400, iat: 1704106800 };
      const token = createMockToken(payload);

      expect(isTokenExpired(token)).toBe(true);
    });
  });

  describe('getTokenTimeRemaining', () => {
    it('returns 0 for null token', () => {
      expect(getTokenTimeRemaining(null)).toBe(0);
    });

    it('returns 0 for invalid token', () => {
      expect(getTokenTimeRemaining('invalid')).toBe(0);
    });

    it('returns 0 for token without exp field', () => {
      const token = createMockToken({ uid: 123 });
      expect(getTokenTimeRemaining(token)).toBe(0);
    });

    it('returns 0 for expired token', () => {
      // Token expired 1 hour ago
      const payload = { uid: 123, exp: 1704106800, iat: 1704103200 };
      const token = createMockToken(payload);

      expect(getTokenTimeRemaining(token)).toBe(0);
    });

    it('returns correct time remaining for valid token', () => {
      // Current time: 2024-01-01T12:00:00Z (1704110400000)
      // Token exp: 2024-01-01T13:00:00Z (1704114000) - 1 hour from now
      const payload = { uid: 123, exp: 1704114000, iat: 1704110400 };
      const token = createMockToken(payload);

      const remaining = getTokenTimeRemaining(token);
      expect(remaining).toBe(3600000); // 1 hour in milliseconds
    });

    it('returns correct time for token expiring in minutes', () => {
      // Token expires in 5 minutes
      const fiveMinutesFromNow = Math.floor(Date.now() / 1000) + 300;
      const payload = { uid: 123, exp: fiveMinutesFromNow, iat: 1704110400 };
      const token = createMockToken(payload);

      const remaining = getTokenTimeRemaining(token);
      expect(remaining).toBe(300000); // 5 minutes in milliseconds
    });
  });

  describe('isTokenExpiringSoon', () => {
    it('returns false for null token', () => {
      expect(isTokenExpiringSoon(null)).toBe(false);
    });

    it('returns false for expired token', () => {
      const payload = { uid: 123, exp: 1704106800, iat: 1704103200 };
      const token = createMockToken(payload);

      expect(isTokenExpiringSoon(token)).toBe(false);
    });

    it('returns true for token expiring within default threshold (5 min)', () => {
      // Token expires in 3 minutes
      const threeMinutesFromNow = Math.floor(Date.now() / 1000) + 180;
      const payload = { uid: 123, exp: threeMinutesFromNow, iat: 1704110400 };
      const token = createMockToken(payload);

      expect(isTokenExpiringSoon(token)).toBe(true);
    });

    it('returns false for token expiring after threshold', () => {
      // Token expires in 10 minutes (beyond 5 min default threshold)
      const tenMinutesFromNow = Math.floor(Date.now() / 1000) + 600;
      const payload = { uid: 123, exp: tenMinutesFromNow, iat: 1704110400 };
      const token = createMockToken(payload);

      expect(isTokenExpiringSoon(token)).toBe(false);
    });

    it('respects custom threshold', () => {
      // Token expires in 8 minutes
      const eightMinutesFromNow = Math.floor(Date.now() / 1000) + 480;
      const payload = { uid: 123, exp: eightMinutesFromNow, iat: 1704110400 };
      const token = createMockToken(payload);

      // Should not expire soon with 5 min threshold
      expect(isTokenExpiringSoon(token, 5)).toBe(false);

      // Should expire soon with 10 min threshold
      expect(isTokenExpiringSoon(token, 10)).toBe(true);
    });

    it('returns true for token expiring exactly at threshold', () => {
      // Token expires in exactly 5 minutes
      const fiveMinutesFromNow = Math.floor(Date.now() / 1000) + 300;
      const payload = { uid: 123, exp: fiveMinutesFromNow, iat: 1704110400 };
      const token = createMockToken(payload);

      expect(isTokenExpiringSoon(token, 5)).toBe(true);
    });

    it('returns false for token expiring in 1 second over threshold', () => {
      // Token expires in 5 minutes and 1 second
      const justOverThreshold = Math.floor(Date.now() / 1000) + 301;
      const payload = { uid: 123, exp: justOverThreshold, iat: 1704110400 };
      const token = createMockToken(payload);

      expect(isTokenExpiringSoon(token, 5)).toBe(false);
    });
  });
});
