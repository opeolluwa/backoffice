use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use ulid::Ulid;

use crate::errors::database_error::DatabaseError;
use crate::{
    dto::{CreateEmailCommand, UpdateEmailCommand},
    models::emails::{self, Entity as EmailEntity},
};

use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait EmailRepositoryExt {
    async fn create_email(
        &self,
        command: &CreateEmailCommand,
    ) -> Result<emails::Model, DatabaseError>;

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<emails::Model, DatabaseError>;

    async fn find_all_emails(&self) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn find_emails_by_tag(&self, tag: &str) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn find_starred_emails(&self) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn find_unread_emails(&self) -> Result<Vec<emails::Model>, DatabaseError>;

    async fn update_email(
        &self,
        identifier: &str,
        command: &UpdateEmailCommand,
    ) -> Result<emails::Model, DatabaseError>;

    async fn delete_email(&self, identifier: &str) -> Result<(), DatabaseError>;

    async fn count_emails(&self) -> Result<i64, DatabaseError>;

    async fn count_unread_emails(&self) -> Result<i64, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct EmailRepository {
    db: DatabaseConnection,
}

impl Repository for EmailRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl EmailRepositoryExt for EmailRepository {
    async fn create_email(
        &self,
        command: &CreateEmailCommand,
    ) -> Result<emails::Model, DatabaseError> {
        let model = emails::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            subject: Set(command.subject.clone()),
            body: Set(command.body.clone()),
            sender_email: Set(command.sender_email.clone()),
            recipient_email: Set(command.recipient_email.clone()),
            tag: Set(command.tag.clone()),
            has_attachments: Set(command.has_attachments.unwrap_or(false)),
            data: Set(command.data.clone().map(|v| v.to_string().into())),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_email_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<emails::Model, DatabaseError> {
        EmailEntity::find()
            .filter(emails::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("email not found".to_string()))
    }

    async fn find_all_emails(&self) -> Result<Vec<emails::Model>, DatabaseError> {
        EmailEntity::find()
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_emails_by_tag(&self, tag: &str) -> Result<Vec<emails::Model>, DatabaseError> {
        EmailEntity::find()
            .filter(emails::Column::Tag.eq(tag))
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_starred_emails(&self) -> Result<Vec<emails::Model>, DatabaseError> {
        EmailEntity::find()
            .filter(emails::Column::IsStarred.eq(true))
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn find_unread_emails(&self) -> Result<Vec<emails::Model>, DatabaseError> {
        EmailEntity::find()
            .filter(emails::Column::IsRead.eq(false))
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn update_email(
        &self,
        identifier: &str,
        command: &UpdateEmailCommand,
    ) -> Result<emails::Model, DatabaseError> {
        let email = EmailEntity::find()
            .filter(emails::Column::Identifier.eq(identifier))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("email not found".to_string()))?;

        let mut active: emails::ActiveModel = email.into();
        if let Some(tag) = &command.tag {
            active.tag = Set(Some(tag.clone()));
        }
        if let Some(is_read) = command.is_read {
            active.is_read = Set(is_read);
        }
        if let Some(is_starred) = command.is_starred {
            active.is_starred = Set(is_starred);
        }
        active.updated_at = Set(Some(chrono::Utc::now().fixed_offset()));

        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_email(&self, identifier: &str) -> Result<(), DatabaseError> {
        EmailEntity::delete_many()
            .filter(emails::Column::Identifier.eq(identifier))
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_emails(&self) -> Result<i64, DatabaseError> {
        let count = EmailEntity::find()
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }

    async fn count_unread_emails(&self) -> Result<i64, DatabaseError> {
        let count = EmailEntity::find()
            .filter(emails::Column::IsRead.eq(false))
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
