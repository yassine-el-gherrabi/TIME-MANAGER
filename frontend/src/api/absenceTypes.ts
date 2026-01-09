/**
 * Absence Types API Client
 *
 * API methods for absence type management operations.
 */

import { apiRequest } from './client';
import { ABSENCE_TYPE_ENDPOINTS } from '../config/constants';
import type {
  AbsenceType,
  CreateAbsenceTypeRequest,
  UpdateAbsenceTypeRequest,
} from '../types/absence';

/**
 * Absence Types API methods
 */
export const absenceTypesApi = {
  /**
   * List all absence types for the organization
   *
   * @returns List of absence types
   */
  list: async (): Promise<AbsenceType[]> => {
    return apiRequest<AbsenceType[]>({
      method: 'GET',
      url: ABSENCE_TYPE_ENDPOINTS.LIST,
    });
  },

  /**
   * Get a single absence type by ID
   *
   * @param id - Absence type ID
   * @returns Absence type
   */
  get: async (id: string): Promise<AbsenceType> => {
    return apiRequest<AbsenceType>({
      method: 'GET',
      url: ABSENCE_TYPE_ENDPOINTS.GET(id),
    });
  },

  /**
   * Create a new absence type (Admin+)
   *
   * @param data - Absence type creation data
   * @returns Created absence type
   */
  create: async (data: CreateAbsenceTypeRequest): Promise<AbsenceType> => {
    return apiRequest<AbsenceType>({
      method: 'POST',
      url: ABSENCE_TYPE_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing absence type (Admin+)
   *
   * @param id - Absence type ID
   * @param data - Fields to update
   * @returns Updated absence type
   */
  update: async (id: string, data: UpdateAbsenceTypeRequest): Promise<AbsenceType> => {
    return apiRequest<AbsenceType>({
      method: 'PUT',
      url: ABSENCE_TYPE_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete an absence type (Admin+)
   *
   * @param id - Absence type ID
   */
  delete: async (id: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: ABSENCE_TYPE_ENDPOINTS.DELETE(id),
    });
  },

  /**
   * Seed default French absence types (Admin+)
   *
   * @returns Success message
   */
  seed: async (): Promise<{ message: string }> => {
    return apiRequest<{ message: string }>({
      method: 'POST',
      url: ABSENCE_TYPE_ENDPOINTS.SEED,
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listAbsenceTypes,
  get: getAbsenceType,
  create: createAbsenceType,
  update: updateAbsenceType,
  delete: deleteAbsenceType,
  seed: seedAbsenceTypes,
} = absenceTypesApi;
