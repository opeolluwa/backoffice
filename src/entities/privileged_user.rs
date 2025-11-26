use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[ts(export)]

pub struct PrivilegedUser {
    pub email: Option<String>,
    pub identifier: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub is_active: bool,
    pub role_name: String,
    pub role_description: Option<String>,
}
