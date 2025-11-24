use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    SuperAdmin,
    Admin,
    PrivilegedUser,
    User,
}

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]
pub struct User {
    pub email: String,
    pub identifier: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
    pub is_active: bool,
    pub role_identifier: String,
}
