// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "absence_status"))]
    pub struct AbsenceStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "audit_action"))]
    pub struct AuditAction;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "break_tracking_mode"))]
    pub struct BreakTrackingMode;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "clock_entry_status"))]
    pub struct ClockEntryStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "clock_override_status"))]
    pub struct ClockOverrideStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "clock_restriction_mode"))]
    pub struct ClockRestrictionMode;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "notification_type"))]
    pub struct NotificationType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
}

diesel::table! {
    absence_types (id) {
        id -> Uuid,
        organization_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 20]
        code -> Varchar,
        #[max_length = 7]
        color -> Nullable<Varchar>,
        requires_approval -> Bool,
        affects_balance -> Bool,
        is_paid -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AbsenceStatus;

    absences (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
        type_id -> Uuid,
        start_date -> Date,
        end_date -> Date,
        days_count -> Numeric,
        status -> AbsenceStatus,
        reason -> Nullable<Text>,
        rejection_reason -> Nullable<Text>,
        approved_by -> Nullable<Uuid>,
        approved_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AuditAction;

    audit_logs (id) {
        id -> Uuid,
        organization_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        action -> AuditAction,
        #[max_length = 50]
        entity_type -> Varchar,
        entity_id -> Uuid,
        old_values -> Nullable<Jsonb>,
        new_values -> Nullable<Jsonb>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        #[max_length = 512]
        user_agent -> Nullable<Varchar>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    break_entries (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
        clock_entry_id -> Uuid,
        break_start -> Timestamptz,
        break_end -> Nullable<Timestamptz>,
        duration_minutes -> Nullable<Int4>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BreakTrackingMode;

    break_policies (id) {
        id -> Uuid,
        organization_id -> Uuid,
        team_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        tracking_mode -> BreakTrackingMode,
        notify_missing_break -> Bool,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    break_windows (id) {
        id -> Uuid,
        break_policy_id -> Uuid,
        day_of_week -> Int2,
        window_start -> Time,
        window_end -> Time,
        min_duration_minutes -> Int4,
        max_duration_minutes -> Int4,
        is_mandatory -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClockEntryStatus;

    clock_entries (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
        clock_in -> Timestamptz,
        clock_out -> Nullable<Timestamptz>,
        status -> ClockEntryStatus,
        approved_by -> Nullable<Uuid>,
        approved_at -> Nullable<Timestamptz>,
        notes -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClockOverrideStatus;

    clock_override_requests (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
        clock_entry_id -> Nullable<Uuid>,
        #[max_length = 10]
        requested_action -> Varchar,
        requested_at -> Timestamptz,
        reason -> Text,
        status -> ClockOverrideStatus,
        reviewed_by -> Nullable<Uuid>,
        reviewed_at -> Nullable<Timestamptz>,
        review_notes -> Nullable<Text>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ClockRestrictionMode;

    clock_restrictions (id) {
        id -> Uuid,
        organization_id -> Uuid,
        team_id -> Nullable<Uuid>,
        user_id -> Nullable<Uuid>,
        mode -> ClockRestrictionMode,
        clock_in_earliest -> Nullable<Time>,
        clock_in_latest -> Nullable<Time>,
        clock_out_earliest -> Nullable<Time>,
        clock_out_latest -> Nullable<Time>,
        enforce_schedule -> Bool,
        require_manager_approval -> Bool,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        max_daily_clock_events -> Nullable<Int4>,
    }
}

diesel::table! {
    closed_days (id) {
        id -> Uuid,
        organization_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        date -> Date,
        is_recurring -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    holidays (id) {
        id -> Uuid,
        organization_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        date -> Date,
        is_recurring -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    invite_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 64]
        token_hash -> Varchar,
        expires_at -> Timestamptz,
        used_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    leave_balances (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
        absence_type_id -> Uuid,
        year -> Int4,
        initial_balance -> Numeric,
        used -> Numeric,
        adjustment -> Numeric,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    login_attempts (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 45]
        ip_address -> Varchar,
        attempted_at -> Timestamp,
        successful -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::NotificationType;

    notifications (id) {
        id -> Uuid,
        organization_id -> Uuid,
        user_id -> Uuid,
        #[sql_name = "type"]
        type_ -> NotificationType,
        #[max_length = 255]
        title -> Varchar,
        message -> Text,
        data -> Nullable<Jsonb>,
        read_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
        #[max_length = 100]
        timezone -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    password_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    password_reset_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token_hash -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        used_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    refresh_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 255]
        token_hash -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        revoked_at -> Nullable<Timestamp>,
        last_used_at -> Timestamp,
        #[max_length = 512]
        user_agent -> Nullable<Varchar>,
    }
}

diesel::table! {
    team_members (id) {
        id -> Uuid,
        team_id -> Uuid,
        user_id -> Uuid,
        joined_at -> Timestamptz,
    }
}

diesel::table! {
    teams (id) {
        id -> Uuid,
        organization_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        manager_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        work_schedule_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        refresh_token_id -> Uuid,
        #[max_length = 512]
        user_agent -> Nullable<Varchar>,
        created_at -> Timestamp,
        last_activity -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserRole;

    users (id) {
        id -> Uuid,
        organization_id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        role -> UserRole,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        password_changed_at -> Nullable<Timestamp>,
        password_expires_at -> Nullable<Timestamp>,
        failed_login_attempts -> Int4,
        locked_until -> Nullable<Timestamp>,
        work_schedule_id -> Nullable<Uuid>,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    work_schedule_days (id) {
        id -> Uuid,
        work_schedule_id -> Uuid,
        day_of_week -> Int2,
        start_time -> Time,
        end_time -> Time,
        break_minutes -> Int4,
    }
}

diesel::table! {
    work_schedules (id) {
        id -> Uuid,
        organization_id -> Uuid,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        is_default -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(absence_types -> organizations (organization_id));
diesel::joinable!(absences -> absence_types (type_id));
diesel::joinable!(absences -> organizations (organization_id));
diesel::joinable!(audit_logs -> organizations (organization_id));
diesel::joinable!(audit_logs -> users (user_id));
diesel::joinable!(break_entries -> clock_entries (clock_entry_id));
diesel::joinable!(break_entries -> organizations (organization_id));
diesel::joinable!(break_entries -> users (user_id));
diesel::joinable!(break_policies -> organizations (organization_id));
diesel::joinable!(break_policies -> teams (team_id));
diesel::joinable!(break_policies -> users (user_id));
diesel::joinable!(break_windows -> break_policies (break_policy_id));
diesel::joinable!(clock_entries -> organizations (organization_id));
diesel::joinable!(clock_override_requests -> clock_entries (clock_entry_id));
diesel::joinable!(clock_override_requests -> organizations (organization_id));
diesel::joinable!(clock_restrictions -> organizations (organization_id));
diesel::joinable!(clock_restrictions -> teams (team_id));
diesel::joinable!(clock_restrictions -> users (user_id));
diesel::joinable!(closed_days -> organizations (organization_id));
diesel::joinable!(holidays -> organizations (organization_id));
diesel::joinable!(invite_tokens -> users (user_id));
diesel::joinable!(leave_balances -> absence_types (absence_type_id));
diesel::joinable!(leave_balances -> organizations (organization_id));
diesel::joinable!(leave_balances -> users (user_id));
diesel::joinable!(notifications -> organizations (organization_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(password_history -> users (user_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(team_members -> teams (team_id));
diesel::joinable!(team_members -> users (user_id));
diesel::joinable!(teams -> organizations (organization_id));
diesel::joinable!(teams -> users (manager_id));
diesel::joinable!(teams -> work_schedules (work_schedule_id));
diesel::joinable!(user_sessions -> refresh_tokens (refresh_token_id));
diesel::joinable!(user_sessions -> users (user_id));
diesel::joinable!(users -> organizations (organization_id));
diesel::joinable!(users -> work_schedules (work_schedule_id));
diesel::joinable!(work_schedule_days -> work_schedules (work_schedule_id));
diesel::joinable!(work_schedules -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    absence_types,
    absences,
    audit_logs,
    break_entries,
    break_policies,
    break_windows,
    clock_entries,
    clock_override_requests,
    clock_restrictions,
    closed_days,
    holidays,
    invite_tokens,
    leave_balances,
    login_attempts,
    notifications,
    organizations,
    password_history,
    password_reset_tokens,
    refresh_tokens,
    team_members,
    teams,
    user_sessions,
    users,
    work_schedule_days,
    work_schedules,
);
