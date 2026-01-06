use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// User role enumeration matching the database user_role ENUM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum UserRole {
    Admin,
    Manager,
    Employee,
}

impl ToSql<Text, Pg> for UserRole {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        let role_str = match self {
            UserRole::Admin => "admin",
            UserRole::Manager => "manager",
            UserRole::Employee => "employee",
        };
        out.write_all(role_str.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for UserRole {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        let role_str = std::str::from_utf8(bytes.as_bytes())?;
        match role_str {
            "admin" => Ok(UserRole::Admin),
            "manager" => Ok(UserRole::Manager),
            "employee" => Ok(UserRole::Employee),
            _ => Err(format!("Unrecognized user role: {}", role_str).into()),
        }
    }
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Employee
    }
}
