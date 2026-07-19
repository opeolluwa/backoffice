use convert_case::{Casing, Case};

#[derive(Debug, Clone)]
pub struct ModuleConfig {
    pub name: String,
    pub name_pascal: String,
    pub name_plural: String,
    pub name_plural_pascal: String,
    pub description: String,
}

impl ModuleConfig {
    pub fn new(name: &str, description: &str) -> Self {
        let name = name.to_case(Case::Snake);
        let name_pascal = name.to_case(Case::Pascal);
        let name_plural = format!("{}s", name);
        let name_plural_pascal = name_plural.to_case(Case::Pascal);

        Self {
            name,
            name_pascal,
            name_plural,
            name_plural_pascal,
            description: description.to_string(),
        }
    }
}

pub fn port(cfg: &ModuleConfig) -> String {
    format!(
        r#"use crate::{{
    dto::{create_cmd}Command, errors::database_error::DatabaseError, models::{model},
}};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait {port_trait} {{
    async fn create_{name}(
        &self,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, DatabaseError>;

    async fn find_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<{model}::Model, DatabaseError>;

    async fn find_all_{plural}(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<{model}::Model>, DatabaseError>;

    async fn update_{name}_by_identifier(
        &self,
        identifier: &str,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, DatabaseError>;

    async fn delete_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError>;

    async fn count_{plural}(&self, user_identifier: &str) -> Result<i64, DatabaseError>;
}}
"#,
        create_cmd = format!("Create{}", cfg.name_pascal),
        port_trait = format!("{}RepositoryExt", cfg.name_pascal),
        name = cfg.name,
        plural = cfg.name_plural,
        model = cfg.name_plural,
    )
}

pub fn service(cfg: &ModuleConfig) -> String {
    format!(
        r#"use crate::errors::service_error::ServiceError;
use crate::{{
    dto::{create_cmd}Command, models::{model},
    ports::{port_file}::{port_trait},
}};

#[derive(Clone)]
pub struct {service_struct}<R: {port_trait}> {{
    repo: R,
}}

impl<R: {port_trait}> {service_struct}<R> {{
    pub fn new(repo: R) -> Self {{
        Self {{ repo }}
    }}
}}

pub trait {service_trait} {{
    async fn create_{name}(
        &self,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, ServiceError>;

    async fn find_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<{model}::Model, ServiceError>;

    async fn find_all_{plural}(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<{model}::Model>, ServiceError>;

    async fn update_{name}_by_identifier(
        &self,
        identifier: &str,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, ServiceError>;

    async fn delete_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError>;

    async fn count_{plural}(&self, user_identifier: &str) -> Result<i64, ServiceError>;
}}

impl<R: {port_trait} + Send + Sync> {service_trait} for {service_struct}<R> {{
    async fn create_{name}(
        &self,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, ServiceError> {{
        Ok(self.repo.create_{name}(command, user_identifier).await?)
    }}

    async fn find_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<{model}::Model, ServiceError> {{
        Ok(self.repo.find_{name}_by_identifier(identifier, user_identifier).await?)
    }}

    async fn find_all_{plural}(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<{model}::Model>, ServiceError> {{
        Ok(self.repo.find_all_{plural}(user_identifier).await?)
    }}

    async fn update_{name}_by_identifier(
        &self,
        identifier: &str,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, ServiceError> {{
        Ok(self.repo.update_{name}_by_identifier(identifier, command, user_identifier).await?)
    }}

    async fn delete_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), ServiceError> {{
        Ok(self.repo.delete_{name}_by_identifier(identifier, user_identifier).await?)
    }}

    async fn count_{plural}(&self, user_identifier: &str) -> Result<i64, ServiceError> {{
        Ok(self.repo.count_{plural}(user_identifier).await?)
    }}
}}
"#,
        create_cmd = format!("Create{}", cfg.name_pascal),
        port_trait = format!("{}RepositoryExt", cfg.name_pascal),
        service_struct = format!("{}Service", cfg.name_pascal),
        service_trait = format!("{}ServiceExt", cfg.name_pascal),
        port_file = format!("{}_repository", cfg.name),
        name = cfg.name,
        plural = cfg.name_plural,
        model = cfg.name_plural,
    )
}

pub fn repository(cfg: &ModuleConfig) -> String {
    format!(
        r#"use sea_orm::{{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set,
}};
use ulid::Ulid;

use backoffice_domain::errors::database_error::DatabaseError;
use backoffice_domain::{{
    dto::{create_cmd}Command,
    models::{model}::{{self, Entity as {entity}}},
    ports::{port_file}::{port_trait},
}};

use crate::database::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct {repo_struct} {{
    db: DatabaseConnection,
}}

impl Repository for {repo_struct} {{
    fn init(db: &DatabaseConnection) -> Self {{
        Self {{ db: db.clone() }}
    }}
}}

impl {port_trait} for {repo_struct} {{
    async fn create_{name}(
        &self,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, DatabaseError> {{
        let model = {model}::ActiveModel {{
            identifier: Set(Ulid::new().to_string()),
            name: Set(command.name.clone()),
            description: Set(command.description.clone()),
            user_identifier: Set(Some(user_identifier.to_string())),
            ..Default::default()
        }};
        let result = model.insert(&self.db).await.map_err(DatabaseError::from)?;
        Ok(result)
    }}

    async fn find_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<{model}::Model, DatabaseError> {{
        {entity}::find()
            .filter({model}::Column::Identifier.eq(identifier))
            .filter({model}::Column::UserIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("{name} not found".to_string()))
    }}

    async fn find_all_{plural}(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<{model}::Model>, DatabaseError> {{
        {entity}::find()
            .filter({model}::Column::UserIdentifier.eq(user_identifier))
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }}

    async fn update_{name}_by_identifier(
        &self,
        identifier: &str,
        command: &{create_cmd}Command,
        user_identifier: &str,
    ) -> Result<{model}::Model, DatabaseError> {{
        let existing = {entity}::find()
            .filter({model}::Column::Identifier.eq(identifier))
            .filter({model}::Column::UserIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("{name} not found".to_string()))?;

        let mut active: {model}::ActiveModel = existing.into();
        active.name = Set(command.name.clone());
        active.description = Set(command.description.clone());
        active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

        active.update(&self.db).await.map_err(DatabaseError::from)
    }}

    async fn delete_{name}_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError> {{
        {entity}::delete_many()
            .filter({model}::Column::Identifier.eq(identifier))
            .filter({model}::Column::UserIdentifier.eq(user_identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }}

    async fn count_{plural}(&self, user_identifier: &str) -> Result<i64, DatabaseError> {{
        let count = {entity}::find()
            .filter({model}::Column::UserIdentifier.eq(user_identifier))
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }}
}}
"#,
        create_cmd = format!("Create{}", cfg.name_pascal),
        port_trait = format!("{}RepositoryExt", cfg.name_pascal),
        repo_struct = format!("{}Repository", cfg.name_pascal),
        port_file = format!("{}_repository", cfg.name),
        entity = format!("{}Entity", cfg.name_pascal),
        name = cfg.name,
        plural = cfg.name_plural,
        model = cfg.name_plural,
    )
}

pub fn handler(cfg: &ModuleConfig) -> String {
    format!(
        r#"use std::sync::Arc;

use axum::{{
    extract::{{Path, State}},
    http::StatusCode,
}};

use backoffice_domain::dto::{create_cmd}Command;
use backoffice_domain::errors::api_response::ApiResponse;
use backoffice_domain::errors::service_error::ServiceError;
use backoffice_domain::models::{model};
use backoffice_domain::services::{service_file}::{service_trait};

use crate::http::dto::{{api_request::AuthenticatedRequest, jwt::Claims}};
use crate::http::extractors::{extractor_file}::{extractor_struct};
use crate::state::AppState;

fn to_command(req: &{extractor_struct}) -> {create_cmd}Command {{
    {create_cmd}Command {{
        name: req.name.clone(),
        description: req.description.clone(),
    }}
}}

pub async fn create_{name}(
    State(state): State<Arc<AppState>>,
    request: AuthenticatedRequest<{extractor_struct}>,
) -> Result<ApiResponse<{model}::Model>, ServiceError> {{
    let command = to_command(&request.data);
    let result = state
        .services
        .{service_field}
        .create_{name}(&command, &request.claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("{name_pascal} created successfully")
        .status_code(StatusCode::CREATED)
        .data(result)
        .build())
}}

pub async fn find_{name}_by_identifier(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(identifier): axum::extract::Path<String>,
) -> Result<ApiResponse<{model}::Model>, ServiceError> {{
    let result = state
        .services
        .{service_field}
        .find_{name}_by_identifier(&identifier, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("{name_pascal} fetched successfully")
        .data(result)
        .build())
}}

pub async fn find_all_{plural}(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<Vec<{model}::Model>>, ServiceError> {{
    let results = state
        .services
        .{service_field}
        .find_all_{plural}(&claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("{name_pascal}s fetched successfully")
        .data(results)
        .build())
}}

pub async fn count_{plural}(
    State(state): State<Arc<AppState>>,
    claims: Claims,
) -> Result<ApiResponse<i64>, ServiceError> {{
    let count = state
        .services
        .{service_field}
        .count_{plural}(&claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("{name_pascal}s counted successfully")
        .data(count)
        .build())
}}

pub async fn update_{name}_by_identifier(
    State(state): State<Arc<AppState>>,
    Path(identifier): Path<String>,
    AuthenticatedRequest {{ data, claims }}: AuthenticatedRequest<{extractor_struct}>,
) -> Result<ApiResponse<{model}::Model>, ServiceError> {{
    let command = to_command(&data);
    let result = state
        .services
        .{service_field}
        .update_{name}_by_identifier(&identifier, &command, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("{name_pascal} updated successfully")
        .data(result)
        .build())
}}

pub async fn delete_{name}_by_identifier(
    State(state): State<Arc<AppState>>,
    claims: Claims,
    Path(identifier): Path<String>,
) -> Result<ApiResponse<()>, ServiceError> {{
    state
        .services
        .{service_field}
        .delete_{name}_by_identifier(&identifier, &claims.identifier)
        .await?;

    Ok(ApiResponse::builder()
        .message("{name_pascal} deleted successfully")
        .build())
}}
"#,
        create_cmd = format!("Create{}", cfg.name_pascal),
        service_trait = format!("{}ServiceExt", cfg.name_pascal),
        service_field = format!("{}_service", cfg.name),
        extractor_struct = format!("Create{}Request", cfg.name_pascal),
        name = cfg.name,
        name_pascal = cfg.name_pascal,
        plural = cfg.name_plural,
        model = cfg.name_plural,
        service_file = cfg.name,
        extractor_file = cfg.name,
    )
}

pub fn extractor(cfg: &ModuleConfig) -> String {
    format!(
        r#"use validator::Validate;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Create{name}Request {{
    pub name: String,
    pub description: String,
}}
"#,
        name = cfg.name_pascal,
    )
}

pub fn route(cfg: &ModuleConfig) -> String {
    format!(
        r#"use std::sync::Arc;

use axum::middleware;
use axum::routing::{{delete, post, put}};
use axum::{{Router, routing::get}};

use crate::http::handlers::{handler_file}::{{
    count_{plural}, create_{name}, delete_{name}_by_identifier,
    find_all_{plural}, find_{name}_by_identifier, update_{name}_by_identifier,
}};
use crate::http::middlewares::auth::authenticate;
use crate::state::AppState;

pub(super) fn {route_fn}(state: Arc<AppState>) -> Router {{
    let routes = Router::new()
        .route("/", post(create_{name}))
        .route("/", get(find_all_{plural}))
        .route("/{{identifier}}", get(find_{name}_by_identifier))
        .route("/count", get(count_{plural}))
        .route("/{{identifier}}", put(update_{name}_by_identifier))
        .route("/{{identifier}}", delete(delete_{name}_by_identifier));

    Router::new()
        .nest("/{plural}", routes)
        .layer(middleware::from_fn(authenticate))
        .with_state(state)
}}
"#,
        handler_file = cfg.name_plural,
        route_fn = format!("{}_routes", cfg.name),
        name = cfg.name,
        plural = cfg.name_plural,
    )
}

pub fn dto_create(cfg: &ModuleConfig) -> String {
    format!(
        r#"
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Create{name}Command {{
    pub name: String,
    pub description: String,
}}
"#,
        name = cfg.name_pascal,
    )
}

pub fn dto_update(cfg: &ModuleConfig) -> String {
    format!(
        r#"
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update{name}Command {{
    pub name: Option<String>,
    pub description: Option<String>,
}}
"#,
        name = cfg.name_pascal,
    )
}
