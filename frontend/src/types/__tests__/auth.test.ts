/**
 * Tests for authentication types
 */

import { describe, it, expect } from 'vitest';
import { UserRole } from '../auth';
import type { User, TokenPair, LoginRequest } from '../auth';

describe('Auth Types', () => {
  describe('UserRole enum', () => {
    it('should have correct role values', () => {
      expect(UserRole.Admin).toBe('Admin');
      expect(UserRole.Manager).toBe('Manager');
      expect(UserRole.Employee).toBe('Employee');
    });
  });

  describe('Type structures', () => {
    it('should accept valid User object', () => {
      const user: User = {
        id: '123e4567-e89b-12d3-a456-426614174000',
        email: 'test@example.com',
        first_name: 'John',
        last_name: 'Doe',
        role: UserRole.Employee,
        organization_id: 'org-1',
        organization_name: 'Test Org',
        organization_timezone: 'Europe/Paris',
        created_at: '2024-01-01T00:00:00Z',
      };

      expect(user.email).toBe('test@example.com');
      expect(user.role).toBe(UserRole.Employee);
    });

    it('should accept valid TokenPair object', () => {
      const tokens: TokenPair = {
        access_token: 'access_token_123',
        refresh_token: 'refresh_token_456',
      };

      expect(tokens.access_token).toBe('access_token_123');
      expect(tokens.refresh_token).toBe('refresh_token_456');
    });

    it('should accept valid LoginRequest object', () => {
      const loginRequest: LoginRequest = {
        email: 'test@example.com',
        password: 'securepassword123',
      };

      expect(loginRequest.email).toBe('test@example.com');
      expect(loginRequest.password).toBe('securepassword123');
    });
  });
});
