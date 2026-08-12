#![allow(async_fn_in_trait)]

pub mod app;
pub mod background;
pub mod cli;
pub mod config;
pub mod dto;
pub mod errors;
pub mod graphql;
pub mod http;
pub mod models;
pub mod payment;
pub mod repositories;
pub mod services;
pub mod state;
pub mod utils;
// pub mod workers

use crate::models::sea_orm_active_enums::FileType;

impl From<String> for FileType {
    fn from(value: String) -> Self {
        match value.as_str() {
            _ => FileType::Others,
        }
    }
}
