/**
 * Team Types
 *
 * TypeScript type definitions for team management.
 */

/**
 * Team from API
 */
export interface Team {
  id: string;
  organization_id: string;
  name: string;
  description: string | null;
  manager_id: string | null;
  created_at: string;
  updated_at: string;
}

/**
 * Team response with member count
 */
export interface TeamResponse {
  id: string;
  organization_id: string;
  name: string;
  description: string | null;
  manager_id: string | null;
  member_count: number;
  created_at: string;
  updated_at: string;
}

/**
 * Team member info
 */
export interface TeamMemberInfo {
  user_id: string;
  email: string;
  first_name: string;
  last_name: string;
  joined_at: string;
}

/**
 * Team with members
 */
export interface TeamWithMembers {
  team: Team;
  members: TeamMemberInfo[];
}

/**
 * Create team request
 */
export interface CreateTeamRequest {
  name: string;
  description?: string;
  manager_id?: string;
}

/**
 * Update team request
 */
export interface UpdateTeamRequest {
  name?: string;
  description?: string | null;
  manager_id?: string | null;
}

/**
 * Add member request
 */
export interface AddMemberRequest {
  user_id: string;
}

/**
 * Team member response
 */
export interface TeamMember {
  id: string;
  team_id: string;
  user_id: string;
  joined_at: string;
}

/**
 * List teams query parameters
 */
export interface ListTeamsParams {
  page?: number;
  per_page?: number;
  search?: string;
  manager_id?: string;
}

/**
 * Paginated teams response
 */
export interface PaginatedTeamsResponse {
  teams: TeamResponse[];
  total: number;
  page: number;
  per_page: number;
}
