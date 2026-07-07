use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use ulid::Ulid;

use crate::domain::models::sea_orm_active_enums::FileType;
use crate::{
    api::http::extractors::upload::{CreateUploadRequest, UpdateUploadRequest},
    domain::{
        models::uploads::{self, Entity as UploadEntity},
        ports::upload_repository::UploadRepositoryExt,
    },
    errors::database_error::DatabaseError,
    infrastructure::database::repositories::base::Repository,
};

#[derive(Debug, Clone)]
pub struct UploadRepository {
    db: DatabaseConnection,
}

impl Repository for UploadRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl UploadRepositoryExt for UploadRepository {
    async fn create_upload(
        &self,
        request: &CreateUploadRequest,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError> {
        let model = uploads::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            name: Set(request.name.clone()),
            url: Set(request.src.clone()),
            file_type: Set(None), //TODO: Set the file_type to None initially
            file_size: Set(request.size),
            starred: Set(request.starred.unwrap_or(false)),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<uploads::Model, DatabaseError> {
        UploadEntity::find()
            .filter(uploads::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("upload not found".to_string()))
    }

    async fn find_all_uploads(&self) -> Result<Vec<uploads::Model>, DatabaseError> {
        UploadEntity::find()
            .order_by_desc(uploads::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_starred_uploads(&self) -> Result<Vec<uploads::Model>, DatabaseError> {
        UploadEntity::find()
            .filter(uploads::Column::Starred.eq(true))
            .order_by_desc(uploads::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn update_upload(
        &self,
        identifier: &str,
        request: &UpdateUploadRequest,
    ) -> Result<uploads::Model, DatabaseError> {
        let upload = UploadEntity::find()
            .filter(uploads::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("upload not found".to_string()))?;

        let mut active: uploads::ActiveModel = upload.into();
        if let Some(name) = &request.name {
            active.name = Set(name.clone());
        }
        if let Some(starred) = request.starred {
            active.starred = Set(starred);
        }
        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_upload(&self, identifier: &str) -> Result<(), DatabaseError> {
        UploadEntity::delete_many()
            .filter(uploads::Column::Identifier.eq(identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_uploads(&self) -> Result<i64, DatabaseError> {
        let count = UploadEntity::find()
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
