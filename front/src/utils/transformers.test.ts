import { describe, it, expect } from 'vitest';
import { toCamelCase, toSnakeCase } from './transformers';

describe('transformers', () => {
  describe('toCamelCase', () => {
    it('converts snake_case to camelCase', () => {
      const input = { first_name: 'John', last_name: 'Doe' };
      const expected = { firstName: 'John', lastName: 'Doe' };
      expect(toCamelCase(input)).toEqual(expected);
    });

    it('handles nested objects', () => {
      const input = {
        user_data: {
          first_name: 'John',
          phone_number: '123',
        },
      };
      const expected = {
        userData: {
          firstName: 'John',
          phoneNumber: '123',
        },
      };
      expect(toCamelCase(input)).toEqual(expected);
    });

    it('handles arrays', () => {
      const input = [{ first_name: 'John' }, { first_name: 'Jane' }];
      const expected = [{ firstName: 'John' }, { firstName: 'Jane' }];
      expect(toCamelCase(input)).toEqual(expected);
    });

    it('handles null and undefined', () => {
      expect(toCamelCase(null)).toBeNull();
      expect(toCamelCase(undefined)).toBeUndefined();
    });

    it('handles primitive values', () => {
      expect(toCamelCase('string')).toBe('string');
      expect(toCamelCase(123)).toBe(123);
      expect(toCamelCase(true)).toBe(true);
    });
  });

  describe('toSnakeCase', () => {
    it('converts camelCase to snake_case', () => {
      const input = { firstName: 'John', lastName: 'Doe' };
      const expected = { first_name: 'John', last_name: 'Doe' };
      expect(toSnakeCase(input)).toEqual(expected);
    });

    it('handles nested objects', () => {
      const input = {
        userData: {
          firstName: 'John',
          phoneNumber: '123',
        },
      };
      const expected = {
        user_data: {
          first_name: 'John',
          phone_number: '123',
        },
      };
      expect(toSnakeCase(input)).toEqual(expected);
    });

    it('handles arrays', () => {
      const input = [{ firstName: 'John' }, { firstName: 'Jane' }];
      const expected = [{ first_name: 'John' }, { first_name: 'Jane' }];
      expect(toSnakeCase(input)).toEqual(expected);
    });

    it('handles null and undefined', () => {
      expect(toSnakeCase(null)).toBeNull();
      expect(toSnakeCase(undefined)).toBeUndefined();
    });

    it('handles primitive values', () => {
      expect(toSnakeCase('string')).toBe('string');
      expect(toSnakeCase(123)).toBe(123);
      expect(toSnakeCase(true)).toBe(true);
    });
  });
});
