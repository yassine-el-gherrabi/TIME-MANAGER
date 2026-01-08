// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
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
    }
}

diesel::joinable!(invite_tokens -> users (user_id));
diesel::joinable!(password_history -> users (user_id));
diesel::joinable!(password_reset_tokens -> users (user_id));
diesel::joinable!(refresh_tokens -> users (user_id));
diesel::joinable!(user_sessions -> refresh_tokens (refresh_token_id));
diesel::joinable!(user_sessions -> users (user_id));
diesel::joinable!(users -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    invite_tokens,
    login_attempts,
    organizations,
    password_history,
    password_reset_tokens,
    refresh_tokens,
    user_sessions,
    users,
);
