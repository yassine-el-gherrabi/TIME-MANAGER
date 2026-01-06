// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_role"))]
    pub struct UserRole;
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
    }
}

diesel::joinable!(users -> organizations (organization_id));

diesel::allow_tables_to_appear_in_same_query!(
    organizations,
    users,
);
