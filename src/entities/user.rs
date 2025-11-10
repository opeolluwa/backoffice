use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use sqlx::prelude::FromRow;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum Role {
    Superadmin,
    Admin,
    PrivilegedUser,
    User,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub email: String,
    pub identifier: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub is_active: bool,
    // pub created_at: DateTime<Local>,
    // pub updated_at: Option<DateTime<Local>>
}
