// Database models for Time Manager application
// This module contains Diesel models for database entities

pub mod absence;
pub mod absence_type;
pub mod audit_log;
pub mod claims;
pub mod organization;
pub mod clock_entry;
pub mod closed_day;
pub mod invite_token;
pub mod leave_balance;
pub mod login_attempt;
pub mod notification;
pub mod password_history;
pub mod password_reset_token;
pub mod refresh_token;
pub mod team;
pub mod team_member;
pub mod token_pair;
pub mod user;
pub mod user_session;
pub mod work_schedule;

// Re-export commonly used types
pub use absence::{
    Absence, AbsenceFilter, AbsenceResponse, AbsenceUpdate, NewAbsence, PaginatedAbsences,
};
pub use absence_type::{AbsenceType, AbsenceTypeResponse, AbsenceTypeUpdate, NewAbsenceType};
pub use claims::Claims;
pub use clock_entry::{
    ClockEntry, ClockEntryResponse, ClockEntryUpdate, ClockFilter, ClockStatus, NewClockEntry,
    PaginatedClockEntries,
};
pub use closed_day::{ClosedDay, ClosedDayFilter, ClosedDayResponse, ClosedDayUpdate, NewClosedDay};
pub use invite_token::{InviteToken, NewInviteToken};
pub use login_attempt::{LoginAttempt, NewLoginAttempt};
pub use notification::{
    NewNotification, Notification, NotificationResponse, NotificationUpdate,
    PaginatedNotifications, UnreadCountResponse,
};
pub use password_history::{NewPasswordHistory, PasswordHistory};
pub use password_reset_token::{NewPasswordResetToken, PasswordResetToken};
pub use refresh_token::{NewRefreshToken, RefreshToken};
pub use leave_balance::{
    LeaveBalance, LeaveBalanceFilter, LeaveBalanceResponse, LeaveBalanceUpdate, NewLeaveBalance,
};
pub use team::{NewTeam, Team, TeamFilter, TeamResponse, TeamUpdate, TeamWithMembers};
pub use team_member::{NewTeamMember, TeamMember};
pub use token_pair::TokenPair;
pub use user::{NewUser, PaginatedUsers, Pagination, UserFilter, UserResponse, UserUpdate};
pub use user_session::{NewUserSession, UserSession};
pub use work_schedule::{
    DayConfig, NewWorkSchedule, NewWorkScheduleDay, WorkSchedule, WorkScheduleDay,
    WorkScheduleDayUpdate, WorkScheduleUpdate, WorkScheduleWithDays,
};
pub use audit_log::{
    AuditContext, AuditLog, AuditLogFilter, AuditLogResponse, AuditUserInfo, NewAuditLog,
    PaginatedAuditLogs,
};
pub use organization::{
    CreateOrganizationRequest, NewOrganization, Organization, OrganizationFilter,
    OrganizationPagination, OrganizationResponse, OrganizationUpdate, PaginatedOrganizations,
    UpdateOrganizationRequest,
};
