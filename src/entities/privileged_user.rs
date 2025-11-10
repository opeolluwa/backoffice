use crate::entities::user_role::UserRole;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PrivilegedUser {
    pub email: Option<String>,
    pub identifier: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub role_name: String,
    pub role_description: Option<String>,
}
