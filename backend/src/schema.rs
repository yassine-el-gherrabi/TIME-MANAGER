// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "clock_entry_status"))]
    pub struct ClockEntryStatus;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
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

diesel::joinable!(clock_entries -> organizations (organization_id));
diesel::joinable!(invite_tokens -> users (user_id));
diesel::joinable!(password_history -> users (user_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(team_members -> teams (team_id));
diesel::joinable!(team_members -> users (user_id));
diesel::joinable!(teams -> organizations (organization_id));
diesel::joinable!(teams -> users (manager_id));
diesel::joinable!(user_sessions -> refresh_tokens (refresh_token_id));
diesel::joinable!(user_sessions -> users (user_id));
diesel::joinable!(users -> organizations (organization_id));
diesel::joinable!(users -> work_schedules (work_schedule_id));
diesel::joinable!(work_schedule_days -> work_schedules (work_schedule_id));
diesel::joinable!(work_schedules -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    clock_entries,invite_tokens,login_attempts,organizations,password_history,password_reset_tokens,refresh_tokens,team_members,teams,user_sessions,users,work_schedule_days,work_schedules,);
