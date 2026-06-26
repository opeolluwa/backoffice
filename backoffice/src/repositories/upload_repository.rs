use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use ulid::Ulid;

use crate::{
    adapters::requests::upload::{CreateUploadRequest, UpdateUploadRequest},
    entities::uploads::{self, Entity as UploadEntity},
    errors::database_error::DatabaseError,
    repositories::base::Repository,
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

pub(crate) trait UploadRepositoryExt {
    async fn create_upload(
        &self,
        request: &CreateUploadRequest,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn find_all_uploads(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<uploads::Model>, DatabaseError>;

    async fn find_starred_uploads(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<uploads::Model>, DatabaseError>;

    async fn update_upload(
        &self,
        identifier: &str,
        request: &UpdateUploadRequest,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError>;

    async fn delete_upload(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError>;

    async fn count_uploads(&self, user_identifier: &str) -> Result<i64, DatabaseError>;
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
            src: Set(request.src.clone()),
            file_type: Set(request.file_type.clone()),
            size: Set(request.size),
            starred: Set(request.starred.unwrap_or(false)),
            user_identifier: Set(Some(user_identifier.to_string())),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_upload_by_identifier(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError> {
        UploadEntity::find()
            .filter(uploads::Column::Identifier.eq(identifier))
            .filter(uploads::Column::UserIdentifier.eq(user_identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("upload not found".to_string()))
    }

    async fn find_all_uploads(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<uploads::Model>, DatabaseError> {
        UploadEntity::find()
            .filter(uploads::Column::UserIdentifier.eq(user_identifier))
            .order_by_desc(uploads::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_starred_uploads(
        &self,
        user_identifier: &str,
    ) -> Result<Vec<uploads::Model>, DatabaseError> {
        UploadEntity::find()
            .filter(uploads::Column::UserIdentifier.eq(user_identifier))
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
        user_identifier: &str,
    ) -> Result<uploads::Model, DatabaseError> {
        let upload = UploadEntity::find()
            .filter(uploads::Column::Identifier.eq(identifier))
            .filter(uploads::Column::UserIdentifier.eq(user_identifier))
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

    async fn delete_upload(
        &self,
        identifier: &str,
        user_identifier: &str,
    ) -> Result<(), DatabaseError> {
        UploadEntity::delete_many()
            .filter(uploads::Column::Identifier.eq(identifier))
            .filter(uploads::Column::UserIdentifier.eq(user_identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_uploads(&self, user_identifier: &str) -> Result<i64, DatabaseError> {
        let count = UploadEntity::find()
            .filter(uploads::Column::UserIdentifier.eq(user_identifier))
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
