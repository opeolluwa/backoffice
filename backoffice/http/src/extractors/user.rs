use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, FromRow, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]

pub struct UserProfile {
    pub identifier: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
