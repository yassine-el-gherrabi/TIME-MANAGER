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
  ClosedDay,
  PaginatedAbsences,
  CreateAbsenceTypeRequest,
  UpdateAbsenceTypeRequest,
  CreateAbsenceRequest,
  RejectAbsenceRequest,
  SetBalanceRequest,
  AdjustBalanceRequest,
  CreateClosedDayRequest,
  UpdateClosedDayRequest,
  AbsenceFilter,
  ClosedDayFilter,
  BalanceFilter,
} from './absence';
export { AbsenceStatus, STATUS_COLORS, STATUS_LABELS } from './absence';

// Notification types
export type {
  Notification,
  PaginatedNotifications,
  UnreadCountResponse,
  MarkAllReadResponse,
  NotificationListParams,
} from './notification';
export {
  NotificationType,
  NOTIFICATION_TYPE_LABELS,
  NOTIFICATION_TYPE_ICONS,
  NOTIFICATION_TYPE_COLORS,
} from './notification';

// Audit types
export type {
  AuditUserInfo,
  AuditLog,
  PaginatedAuditLogs,
  AuditLogFilter,
} from './audit';
export {
  AuditAction,
  ENTITY_TYPE_LABELS,
  ACTION_LABELS,
  ACTION_COLORS,
  ACTION_COLORS_DARK,
} from './audit';

// Organization types
export type {
  OrganizationResponse,
  CreateOrganizationRequest,
  UpdateOrganizationRequest,
  ListOrganizationsParams,
  PaginatedOrganizationsResponse,
  DeleteOrganizationResponse,
} from './organization';

// Clock restriction types
export type {
  ClockRestrictionMode,
  ClockOverrideStatus,
  ClockRestriction,
  ClockRestrictionResponse,
  EffectiveRestriction,
  CreateClockRestrictionRequest,
  UpdateClockRestrictionRequest,
  ClockValidationResult,
  ClockOverrideRequest,
  ClockOverrideRequestResponse,
  CreateOverrideRequest,
  ReviewOverrideRequest,
  ClockRestrictionFilter,
  ClockOverrideFilter,
  PaginatedClockRestrictions,
  PaginatedOverrideRequests,
} from './clockRestriction';
export {
  RESTRICTION_MODE_CONFIG,
  OVERRIDE_STATUS_CONFIG,
} from './clockRestriction';

// Break system types
export type {
  BreakTrackingMode,
  BreakPolicy,
  BreakPolicyResponse,
  CreateBreakPolicyRequest,
  UpdateBreakPolicyRequest,
  BreakPolicyFilter,
  PaginatedBreakPolicies,
  BreakWindow,
  BreakWindowResponse,
  CreateBreakWindowRequest,
  BreakEntry,
  BreakEntryResponse,
  StartBreakRequest,
  EndBreakRequest,
  BreakEntryFilter,
  PaginatedBreakEntries,
  BreakStatus,
  EffectiveBreakPolicy,
  BreakDeduction,
} from './break';
export {
  DAYS_OF_WEEK,
  TRACKING_MODE_OPTIONS,
  getDayLabel,
  getTrackingModeLabel,
  formatBreakDuration,
} from './break';

// Re-export enums
export { UserRole } from './auth';
