/**
 * Schedules API Client
 *
 * API methods for work schedule management operations.
 */

import { apiRequest } from './client';
import { SCHEDULE_ENDPOINTS, USER_ENDPOINTS } from '../config/constants';
import type {
  WorkScheduleDay,
  WorkScheduleWithDays,
  CreateScheduleRequest,
  UpdateScheduleRequest,
  AddDayRequest,
  UpdateDayRequest,
  AssignScheduleRequest,
} from '../types/schedule';

/**
 * Schedules API methods
 */
export const schedulesApi = {
  /**
   * List all schedules for the organization
   *
   * @returns List of schedules with their days
   */
  list: async (): Promise<WorkScheduleWithDays[]> => {
    return apiRequest<WorkScheduleWithDays[]>({
      method: 'GET',
      url: SCHEDULE_ENDPOINTS.LIST,
    });
  },

  /**
   * Get a single schedule by ID
   *
   * @param id - Schedule ID
   * @returns Schedule with its days
   */
  get: async (id: string): Promise<WorkScheduleWithDays> => {
    return apiRequest<WorkScheduleWithDays>({
      method: 'GET',
      url: SCHEDULE_ENDPOINTS.GET(id),
    });
  },

  /**
   * Get current user's schedule
   *
   * @returns User's schedule or null
   */
  getMySchedule: async (): Promise<WorkScheduleWithDays | null> => {
    return apiRequest<WorkScheduleWithDays | null>({
      method: 'GET',
      url: SCHEDULE_ENDPOINTS.MY_SCHEDULE,
    });
  },

  /**
   * Create a new schedule (Admin+)
   *
   * @param data - Schedule creation data
   * @returns Created schedule with days
   */
  create: async (data: CreateScheduleRequest): Promise<WorkScheduleWithDays> => {
    return apiRequest<WorkScheduleWithDays>({
      method: 'POST',
      url: SCHEDULE_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing schedule (Admin+)
   *
   * @param id - Schedule ID
   * @param data - Fields to update
   * @returns Updated schedule with days
   */
  update: async (id: string, data: UpdateScheduleRequest): Promise<WorkScheduleWithDays> => {
    return apiRequest<WorkScheduleWithDays>({
      method: 'PUT',
      url: SCHEDULE_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete a schedule (Admin+)
   *
   * @param id - Schedule ID
   */
  delete: async (id: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: SCHEDULE_ENDPOINTS.DELETE(id),
    });
  },

  /**
   * Add a day to a schedule (Admin+)
   *
   * @param scheduleId - Schedule ID
   * @param data - Day configuration
   * @returns Created schedule day
   */
  addDay: async (scheduleId: string, data: AddDayRequest): Promise<WorkScheduleDay> => {
    return apiRequest<WorkScheduleDay>({
      method: 'POST',
      url: SCHEDULE_ENDPOINTS.ADD_DAY(scheduleId),
      data,
    });
  },

  /**
   * Update a schedule day (Admin+)
   *
   * @param dayId - Day ID
   * @param data - Fields to update
   * @returns Updated schedule day
   */
  updateDay: async (dayId: string, data: UpdateDayRequest): Promise<WorkScheduleDay> => {
    return apiRequest<WorkScheduleDay>({
      method: 'PUT',
      url: SCHEDULE_ENDPOINTS.UPDATE_DAY(dayId),
      data,
    });
  },

  /**
   * Remove a day from a schedule (Admin+)
   *
   * @param dayId - Day ID
   */
  removeDay: async (dayId: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: SCHEDULE_ENDPOINTS.REMOVE_DAY(dayId),
    });
  },

  /**
   * Assign a schedule to a user (Admin+)
   *
   * @param userId - User ID
   * @param data - Schedule assignment data
   */
  assignToUser: async (userId: string, data: AssignScheduleRequest): Promise<void> => {
    return apiRequest<void>({
      method: 'PUT',
      url: USER_ENDPOINTS.ASSIGN_SCHEDULE(userId),
      data,
    });
  },

  /**
   * Remove schedule assignment from a user (Admin+)
   *
   * @param userId - User ID
   */
  unassignFromUser: async (userId: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: USER_ENDPOINTS.ASSIGN_SCHEDULE(userId),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listSchedules,
  get: getSchedule,
  getMySchedule,
  create: createSchedule,
  update: updateSchedule,
  delete: deleteSchedule,
  addDay: addScheduleDay,
  updateDay: updateScheduleDay,
  removeDay: removeScheduleDay,
  assignToUser: assignScheduleToUser,
  unassignFromUser: unassignScheduleFromUser,
} = schedulesApi;
