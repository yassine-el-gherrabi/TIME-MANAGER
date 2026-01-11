// Repository layer for database access logic
// Provides data access abstractions for all entities

pub mod absence_repository;
pub mod absence_type_repository;
pub mod audit_repository;
pub mod clock_repository;
pub mod closed_day_repository;
pub mod invite_token_repository;
pub mod leave_balance_repository;
pub mod login_attempt_repository;
pub mod notification_repository;
pub mod organization_repository;
pub mod password_history_repository;
pub mod password_reset_repository;
pub mod refresh_token_repository;
pub mod team_repository;
pub mod user_repository;
pub mod user_session_repository;
pub mod work_schedule_repository;

// Re-export repository types for convenience
pub use absence_repository::AbsenceRepository;
pub use absence_type_repository::AbsenceTypeRepository;
pub use audit_repository::AuditRepository;
pub use clock_repository::ClockRepository;
pub use closed_day_repository::ClosedDayRepository;
pub use invite_token_repository::InviteTokenRepository;
pub use leave_balance_repository::LeaveBalanceRepository;
pub use login_attempt_repository::LoginAttemptRepository;
pub use notification_repository::NotificationRepository;
pub use organization_repository::OrganizationRepository;
pub use password_history_repository::PasswordHistoryRepository;
pub use password_reset_repository::PasswordResetRepository;
pub use refresh_token_repository::RefreshTokenRepository;
pub use team_repository::TeamRepository;
pub use user_repository::{User, UserRepository};
pub use user_session_repository::UserSessionRepository;
pub use work_schedule_repository::WorkScheduleRepository;
