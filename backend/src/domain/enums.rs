use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::schema::sql_types::UserRole as UserRoleSqlType;

/// User role enumeration matching the database user_role ENUM
/// Hierarchy: SuperAdmin > Admin > Manager > Employee
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = UserRoleSqlType)]
#[derive(Default)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    Manager,
    #[default]
    Employee,
}

impl ToSql<UserRoleSqlType, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let role_str = match self {
            UserRole::SuperAdmin => "super_admin",
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
            UserRole::Employee => "employee",
        };
        out.write_all(role_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<UserRoleSqlType, Pg> for UserRole {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let role_str = std::str::from_utf8(bytes.as_bytes())?;
        match role_str {
            "super_admin" => Ok(UserRole::SuperAdmin),
            "admin" => Ok(UserRole::Admin),
            "manager" => Ok(UserRole::Manager),
            "employee" => Ok(UserRole::Employee),
            _ => Err(format!("Unrecognized user role: {}", role_str).into()),
        }
    }
}
