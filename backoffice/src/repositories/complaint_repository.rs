use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};

use crate::errors::database_error::DatabaseError;
use crate::{
    dto::CreateComplaintCommand,
    models::{
        complaints::{self, Entity as ComplaintEntity},
        customers::{self, Entity as CustomerEntity},
        sea_orm_active_enums::ComplaintStatus,
    },
};
use ulid::Ulid;

use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait ComplaintRepositoryExt {
    async fn create_complaint(
        &self,
        command: &CreateComplaintCommand,
    ) -> Result<complaints::Model, DatabaseError>;

    async fn find_all_complaints(
        &self,
    ) -> Result<Vec<(complaints::Model, Option<customers::Model>)>, DatabaseError>;

    async fn find_complaint_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<(complaints::Model, Option<customers::Model>), DatabaseError>;

    async fn update_complaint_status(
        &self,
        identifier: &str,
        status: ComplaintStatus,
    ) -> Result<complaints::Model, DatabaseError>;

    async fn update_complaint<'a>(
        &self,
        identifier: &str,
        subject: Option<&'a str>,
        description: Option<&'a str>,
        status: Option<ComplaintStatus>,
    ) -> Result<complaints::Model, DatabaseError>;

    async fn delete_complaint_by_identifier(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn count_complaints(&self) -> Result<i64, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct ComplaintRepository {
    db: DatabaseConnection,
}

impl Repository for ComplaintRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl ComplaintRepositoryExt for ComplaintRepository {
    async fn create_complaint(
        &self,
        command: &CreateComplaintCommand,
    ) -> Result<complaints::Model, DatabaseError> {
        let model = complaints::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            customer_identifier: Set(command.customer_identifier.clone()),
            order_identifier: Set(command.order_identifier.clone()),
            subject: Set(command.subject.clone()),
            description: Set(command.description.clone()),
            status: Set(Some(ComplaintStatus::Open)),
            ..Default::default()
        };

        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_all_complaints(
        &self,
    ) -> Result<Vec<(complaints::Model, Option<customers::Model>)>, DatabaseError> {
        ComplaintEntity::find()
            .find_also_related(CustomerEntity)
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_complaint_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<(complaints::Model, Option<customers::Model>), DatabaseError> {
        ComplaintEntity::find()
            .filter(complaints::Column::Identifier.eq(identifier))
            .find_also_related(CustomerEntity)
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("complaint not found".to_string()))
    }

    async fn update_complaint_status(
        &self,
        identifier: &str,
        status: ComplaintStatus,
    ) -> Result<complaints::Model, DatabaseError> {
        let existing = ComplaintEntity::find()
            .filter(complaints::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("complaint not found".to_string()))?;

        let mut active: complaints::ActiveModel = existing.into();
        active.status = Set(Some(status));

        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn update_complaint(
        &self,
        identifier: &str,
        subject: Option<&str>,
        description: Option<&str>,
        status: Option<ComplaintStatus>,
    ) -> Result<complaints::Model, DatabaseError> {
        let existing = ComplaintEntity::find()
            .filter(complaints::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("complaint not found".to_string()))?;

        let mut active: complaints::ActiveModel = existing.into();
        if let Some(s) = subject {
            active.subject = Set(s.to_string());
        }
        if let Some(d) = description {
            active.description = Set(d.to_string());
        }
        if let Some(st) = status {
            active.status = Set(Some(st));
        }

        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_complaint_by_identifier(&self, identifier: &str) -> Result<(), DatabaseError> {
        ComplaintEntity::delete_many()
            .filter(complaints::Column::Identifier.eq(identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_complaints(&self) -> Result<i64, DatabaseError> {
        let count = ComplaintEntity::find()
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
