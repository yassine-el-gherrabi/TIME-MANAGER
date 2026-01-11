/**
 * Teams API Client
 *
 * API methods for team management operations.
 */

import { apiRequest } from './client';
import { TEAM_ENDPOINTS } from '../config/constants';
import type {
  Team,
  TeamResponse,
  TeamWithMembers,
  CreateTeamRequest,
  UpdateTeamRequest,
  AddMemberRequest,
  TeamMember,
  ListTeamsParams,
  PaginatedTeamsResponse,
} from '../types/team';

/**
 * Teams API methods
 */
export const teamsApi = {
  /**
   * List teams with optional filters and pagination
   *
   * @param params - Query parameters for filtering and pagination
   * @returns Paginated list of teams
   */
  list: async (params: ListTeamsParams = {}): Promise<PaginatedTeamsResponse> => {
    const queryParams = new URLSearchParams();

    if (params.page !== undefined) {
      queryParams.set('page', params.page.toString());
    }
    if (params.per_page !== undefined) {
      queryParams.set('per_page', params.per_page.toString());
    }
    if (params.search) {
      queryParams.set('search', params.search);
    }
    if (params.manager_id) {
      queryParams.set('manager_id', params.manager_id);
    }
    if (params.organization_id) {
      queryParams.set('organization_id', params.organization_id);
    }

    const queryString = queryParams.toString();
    const url = queryString ? `${TEAM_ENDPOINTS.LIST}?${queryString}` : TEAM_ENDPOINTS.LIST;

    return apiRequest<PaginatedTeamsResponse>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get a single team by ID
   *
   * @param id - Team ID
   * @param includeMembers - Whether to include members
   * @returns Team details
   */
  get: async (id: string, includeMembers = false): Promise<TeamResponse | TeamWithMembers> => {
    const url = includeMembers
      ? `${TEAM_ENDPOINTS.GET(id)}?include_members=true`
      : TEAM_ENDPOINTS.GET(id);

    return apiRequest<TeamResponse | TeamWithMembers>({
      method: 'GET',
      url,
    });
  },

  /**
   * Get current user's teams
   *
   * @returns List of user's teams
   */
  getMyTeams: async (): Promise<Team[]> => {
    return apiRequest<Team[]>({
      method: 'GET',
      url: TEAM_ENDPOINTS.MY_TEAMS,
    });
  },

  /**
   * Create a new team (Admin+)
   *
   * @param data - Team creation data
   * @returns Created team
   */
  create: async (data: CreateTeamRequest): Promise<Team> => {
    return apiRequest<Team>({
      method: 'POST',
      url: TEAM_ENDPOINTS.CREATE,
      data,
    });
  },

  /**
   * Update an existing team (Admin+)
   *
   * @param id - Team ID
   * @param data - Fields to update
   * @returns Updated team
   */
  update: async (id: string, data: UpdateTeamRequest): Promise<Team> => {
    return apiRequest<Team>({
      method: 'PUT',
      url: TEAM_ENDPOINTS.UPDATE(id),
      data,
    });
  },

  /**
   * Delete a team (Admin+)
   *
   * @param id - Team ID
   */
  delete: async (id: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: TEAM_ENDPOINTS.DELETE(id),
    });
  },

  /**
   * Add a member to a team (Admin+)
   *
   * @param teamId - Team ID
   * @param data - User ID to add
   * @returns Team member record
   */
  addMember: async (teamId: string, data: AddMemberRequest): Promise<TeamMember> => {
    return apiRequest<TeamMember>({
      method: 'POST',
      url: TEAM_ENDPOINTS.ADD_MEMBER(teamId),
      data,
    });
  },

  /**
   * Remove a member from a team (Admin+)
   *
   * @param teamId - Team ID
   * @param userId - User ID to remove
   */
  removeMember: async (teamId: string, userId: string): Promise<void> => {
    return apiRequest<void>({
      method: 'DELETE',
      url: TEAM_ENDPOINTS.REMOVE_MEMBER(teamId, userId),
    });
  },
};

/**
 * Export individual methods for convenience
 */
export const {
  list: listTeams,
  get: getTeam,
  getMyTeams,
  create: createTeam,
  update: updateTeam,
  delete: deleteTeam,
  addMember,
  removeMember,
} = teamsApi;
