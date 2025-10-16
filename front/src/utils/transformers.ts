// Utility functions to transform between snake_case (backend) and camelCase (frontend)

import type { User, Team, Clock, WorkingTime } from '@/types/models';
import type {
  UserResponse,
  TeamResponse,
  ClockResponse,
  WorkingTimeResponse,
} from '@/types/api';

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
    return Object.keys(obj).reduce((acc, key) => {
      const camelKey = key.replace(/_([a-z])/g, (_, letter) =>
        letter.toUpperCase()
      );
      acc[camelKey] = toCamelCase((obj as Record<string, unknown>)[key]);
      return acc;
    }, {} as Record<string, unknown>) as T;
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
    return Object.keys(obj).reduce((acc, key) => {
      const snakeKey = key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
      acc[snakeKey] = toSnakeCase((obj as Record<string, unknown>)[key]);
      return acc;
    }, {} as Record<string, unknown>) as T;
  }

  return obj as T;
}

// Specific transformers for type safety

export function transformUser(response: UserResponse): User {
  return {
    id: response.id,
    email: response.email,
    firstName: response.first_name,
    lastName: response.last_name,
    phoneNumber: response.phone_number,
    role: response.role,
    teamId: response.team_id,
    createdAt: response.created_at,
    updatedAt: response.updated_at,
  };
}

export function transformTeam(response: TeamResponse): Team {
  return {
    id: response.id,
    name: response.name,
    description: response.description,
    managerId: response.manager_id,
    memberIds: response.member_ids,
    createdAt: response.created_at,
    updatedAt: response.updated_at,
  };
}

export function transformClock(response: ClockResponse): Clock {
  return {
    id: response.id,
    userId: response.user_id,
    timestamp: response.timestamp,
    status: response.status,
    createdAt: response.created_at,
  };
}

export function transformWorkingTime(response: WorkingTimeResponse): WorkingTime {
  return {
    id: response.id,
    userId: response.user_id,
    startTime: response.start_time,
    endTime: response.end_time,
    duration: response.duration,
    date: response.date,
  };
}
