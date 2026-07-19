use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::commands::create_module::{CreateModuleParams, generate_module};
use crate::errors::CliError;

#[derive(Deserialize)]
pub struct CreateModuleRequest {
    pub name: String,
    pub description: String,
}

#[derive(serde::Serialize)]
pub struct ApiResponse {
    message: String,
}

pub async fn create_module(
    Json(req): Json<CreateModuleRequest>,
) -> Result<(StatusCode, Json<ApiResponse>), CliError> {
    let params = CreateModuleParams {
        name: req.name,
        description: req.description,
    };

    generate_module(&params)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            message: "Module generated successfully".to_string(),
        }),
    ))
}
