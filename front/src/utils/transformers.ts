/**
 * Utility functions to transform between snake_case (backend) and camelCase (frontend)
 * These functions are used by API interceptors to automatically transform data
 */

/**
 * Recursively converts object keys from snake_case to camelCase
 */
export function toCamelCase<T = unknown>(obj: unknown): T {
  if (obj === null || obj === undefined) {
    return obj as T;
  }

  if (Array.isArray(obj)) {
    return obj.map((item) => toCamelCase(item)) as T;
  }

  if (typeof obj === 'object' && obj.constructor === Object) {
    return Object.keys(obj).reduce(
      (acc, key) => {
        const camelKey = key.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
        acc[camelKey] = toCamelCase((obj as Record<string, unknown>)[key]);
        return acc;
      },
      {} as Record<string, unknown>
    ) as T;
  }

  return obj as T;
}

/**
 * Recursively converts object keys from camelCase to snake_case
 */
export function toSnakeCase<T = unknown>(obj: unknown): T {
  if (obj === null || obj === undefined) {
    return obj as T;
  }

  if (Array.isArray(obj)) {
    return obj.map((item) => toSnakeCase(item)) as T;
  }

  if (typeof obj === 'object' && obj.constructor === Object) {
    return Object.keys(obj).reduce(
      (acc, key) => {
        const snakeKey = key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
        acc[snakeKey] = toSnakeCase((obj as Record<string, unknown>)[key]);
        return acc;
      },
      {} as Record<string, unknown>
    ) as T;
  }

  return obj as T;
}
