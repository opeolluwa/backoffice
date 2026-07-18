use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use sea_orm::prelude::Uuid;

use backoffice_domain::models::{
    invitation::{self, Entity as InvitationEntity},
    sea_orm_active_enums::InvitationStatus,
};
use backoffice_domain::ports::invitation_repository::InvitationRepositoryExt;
use backoffice_domain::errors::database_error::DatabaseError;
use crate::database::repositories::base::Repository;

#[derive(Debug, Clone)]
pub struct InvitationRepository {
    db: DatabaseConnection,
}

impl Repository for InvitationRepository {
    fn init(db: &DatabaseConnection) -> Self {
        Self { db: db.clone() }
    }
}

impl InvitationRepositoryExt for InvitationRepository {
    async fn create_invitation(
        &self,
        email: &str,
        token: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        let model = invitation::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            email: Set(email.to_string()),
            status: Set(Some(InvitationStatus::Pending)),
            token: Set(token.to_string()),
            ..Default::default()
        };
        model.insert(&self.db).await.map_err(DatabaseError::from)
    }

    async fn find_invitation_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        let uuid =
            Uuid::parse_str(identifier).map_err(|e| DatabaseError::InvalidData(e.to_string()))?;
        InvitationEntity::find_by_id(uuid)
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("invitation not found".to_string()))
    }

    async fn find_invitation_by_token(
        &self,
        token: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        InvitationEntity::find()
            .filter(invitation::Column::Token.eq(token))
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("invitation not found".to_string()))
    }

    async fn find_all_invitations(&self) -> Result<Vec<invitation::Model>, DatabaseError> {
        InvitationEntity::find()
            .order_by_desc(invitation::Column::CreatedAt)
            .all(&self.db)
            .await
            .map_err(DatabaseError::from)
    }

    async fn accept_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        let uuid =
            Uuid::parse_str(identifier).map_err(|e| DatabaseError::InvalidData(e.to_string()))?;

        let record = InvitationEntity::find_by_id(uuid)
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("invitation not found".to_string()))?;

        let mut active: invitation::ActiveModel = record.into();
        active.status = Set(Some(InvitationStatus::Accepted));
        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn block_invitation(
        &self,
        identifier: &str,
    ) -> Result<invitation::Model, DatabaseError> {
        let uuid =
            Uuid::parse_str(identifier).map_err(|e| DatabaseError::InvalidData(e.to_string()))?;

        let record = InvitationEntity::find_by_id(uuid)
            .one(&self.db)
            .await
            .map_err(DatabaseError::from)?
            .ok_or_else(|| DatabaseError::NotFound("invitation not found".to_string()))?;

        let mut active: invitation::ActiveModel = record.into();
        active.status = Set(Some(InvitationStatus::Rejected));
        active.update(&self.db).await.map_err(DatabaseError::from)
    }

    async fn delete_invitation(&self, identifier: &str) -> Result<(), DatabaseError> {
        let uuid =
            Uuid::parse_str(identifier).map_err(|e| DatabaseError::InvalidData(e.to_string()))?;
        InvitationEntity::delete_by_id(uuid)
            .exec(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(())
    }

    async fn count_invitations(&self) -> Result<i64, DatabaseError> {
        let count = InvitationEntity::find()
            .count(&self.db)
            .await
            .map_err(DatabaseError::from)?;
        Ok(count as i64)
    }
}
