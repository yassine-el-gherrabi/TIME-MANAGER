// Service layer for business logic
// This module contains services that orchestrate business operations

pub mod auth_service;
pub mod brute_force_service;
pub mod clock_service;
pub mod email_service;
pub mod email_templates;
pub mod hibp_service;
pub mod invite_service;
pub mod kpi_service;
pub mod password_expiry_service;
pub mod password_reset_service;
pub mod session_service;
pub mod team_service;
pub mod work_schedule_service;

// Re-export commonly used types
pub use auth_service::AuthService;
pub use brute_force_service::BruteForceService;
pub use clock_service::ClockService;
pub use email_service::EmailService;
pub use hibp_service::HibpService;
pub use invite_service::InviteService;
pub use kpi_service::{
    ChartData, DateRange, Granularity, KPIService, MemberKPISummary, OrgKPIs, PresenceOverview,
    TeamKPIs, UserKPIs,
};
pub use password_expiry_service::{
    PasswordExpiryPolicy, PasswordExpiryService, PasswordExpiryStatus,
};
pub use password_reset_service::PasswordResetService;
pub use session_service::SessionService;
pub use team_service::{CreateTeamRequest, TeamService, UpdateTeamRequest};
pub use work_schedule_service::{
    AddDayRequest, CreateScheduleRequest, UpdateDayRequest, UpdateScheduleRequest,
    WorkScheduleService,
};
