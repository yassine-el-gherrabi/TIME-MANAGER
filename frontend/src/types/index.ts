/**
 * Type Definitions Index
 *
 * Central export point for all TypeScript type definitions.
 */

// Authentication types
export type {
  TokenPair,
  LoginRequest,
  LoginResponse,
  RefreshRequest,
  RefreshResponse,
  LogoutResponse,
  LogoutAllResponse,
  User,
  MeResponse,
  RequestResetRequest,
  RequestResetResponse,
  ResetPasswordRequest,
  ResetPasswordResponse,
  SessionInfo,
  ActiveSessionsResponse,
  JwtClaims,
  AuthState,
  ApiError,
} from './auth';

// User management types
export type {
  InviteStatus,
  UserResponse,
  CreateUserRequest,
  CreateUserResponse,
  UpdateUserRequest,
  ListUsersParams,
  PaginatedUsersResponse,
  ResendInviteResponse,
  DeleteUserResponse,
} from './user';

// Clock types
export type {
  ClockEntryStatus,
  ClockEntry,
  ClockEntryResponse,
  ClockStatus,
  ClockInRequest,
  ClockOutRequest,
  RejectEntryRequest,
  ClockHistoryParams,
  PaginatedClockHistoryResponse,
  PendingEntriesParams,
  PaginatedPendingResponse,
} from './clock';

// Team types
export type {
  Team,
  TeamResponse,
  TeamMemberInfo,
  TeamWithMembers,
  CreateTeamRequest,
  UpdateTeamRequest,
  AddMemberRequest,
  TeamMember,
  ListTeamsParams,
  PaginatedTeamsResponse,
} from './team';

// Schedule types
export type {
  WorkSchedule,
  WorkScheduleDay,
  WorkScheduleWithDays,
  DayConfig,
  CreateScheduleRequest,
  UpdateScheduleRequest,
  AddDayRequest,
  UpdateDayRequest,
  AssignScheduleRequest,
} from './schedule';
export { DAY_LABELS } from './schedule';

// KPI types
export type {
  UserKPIs,
  MemberKPISummary,
  TeamKPIs,
  OrgKPIs,
  PresentUser,
  PresenceOverview,
  ChartDataPoint,
  ChartData,
  KPIQueryParams,
  ChartQueryParams,
  DateRange,
} from './kpi';

// Absence types
export type {
  AbsenceType,
  Absence,
  LeaveBalance,
  Holiday,
  PaginatedAbsences,
  CreateAbsenceTypeRequest,
  UpdateAbsenceTypeRequest,
  CreateAbsenceRequest,
  RejectAbsenceRequest,
  SetBalanceRequest,
  AdjustBalanceRequest,
  CreateHolidayRequest,
  UpdateHolidayRequest,
  AbsenceFilter,
  HolidayFilter,
  BalanceFilter,
} from './absence';
export { AbsenceStatus, STATUS_COLORS, STATUS_LABELS } from './absence';

// Re-export enums
export { UserRole } from './auth';
