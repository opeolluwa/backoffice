#![allow(async_fn_in_trait)]

pub mod dto;
pub mod errors;
pub mod models;
pub mod ports;
pub mod services;
pub mod shared;
pub mod utils;

use crate::models::sea_orm_active_enums::FileType;

impl From<String> for FileType {
    fn from(value: String) -> Self {
        match value.as_str() {
            _ => FileType::Others,
        }
    }
}
