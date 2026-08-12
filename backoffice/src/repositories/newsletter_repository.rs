use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter,
};
use ulid::Ulid;

use crate::{
    errors::database_error::{self, DatabaseError},
    models::newsletter,
};

use crate::repositories::base::Repository;

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait NewsletterRepositoryExt {
    fn subscribe(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<newsletter::Model, DatabaseError>>;

    fn unsubscribe(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<(), DatabaseError>>;
}

#[derive(Debug, Clone)]
pub struct NewsletterRepository {
    db: DatabaseConnection,
}

impl Repository for NewsletterRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl NewsletterRepositoryExt for NewsletterRepository {
    async fn subscribe(&self, email: &str) -> Result<newsletter::Model, DatabaseError> {
        let model = newsletter::ActiveModel {
            identifier: Set(Ulid::new().to_string()),
            email: Set(email.to_owned()),
            subscribed: Set(true),
        };

        model.insert(&self.db).await.map_err(|err: DbErr| {
            let msg = err.to_string();
            if msg.contains("UNIQUE constraint failed") || msg.contains("duplicate key") {
                DatabaseError::DuplicateEntry(email.to_string())
            } else {
                DatabaseError::from(err)
            }
        })
    }

    async fn unsubscribe(&self, email: &str) -> Result<(), database_error::DatabaseError> {
        let Some(subscriber) = newsletter::Entity::find()
            .filter(newsletter::Column::Email.eq(email.trim()))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
        else {
            return Ok(());
        };

        let mut subscriber: newsletter::ActiveModel = subscriber.into();
        subscriber.subscribed = Set(false);
        Ok(())
    }
}
