use crate::{
    dto::{CreateComplaintCommand, UpdateComplaintCommand},
    errors::service_error::ServiceError,
    models::{complaints, customers, sea_orm_active_enums::ComplaintStatus},
    repositories::complaint_repository::ComplaintRepositoryExt,
};

#[derive(Clone)]
pub struct ComplaintService<Repo: ComplaintRepositoryExt> {
    repository: Repo,
}

impl<Repo: ComplaintRepositoryExt> ComplaintService<Repo> {
    pub fn new(repository: Repo) -> Self {
        Self { repository }
    }
}

pub trait ComplaintServiceExt {
    async fn create_complaint(
        &self,
        command: CreateComplaintCommand,
    ) -> Result<complaints::Model, ServiceError>;

    async fn find_all_complaints(
        &self,
    ) -> Result<Vec<(complaints::Model, Option<customers::Model>)>, ServiceError>;

    async fn find_complaint_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<(complaints::Model, Option<customers::Model>), ServiceError>;

    async fn update_complaint(
        &self,
        identifier: &str,
        command: UpdateComplaintCommand,
    ) -> Result<complaints::Model, ServiceError>;

    async fn delete_complaint(&self, identifier: &str) -> Result<(), ServiceError>;

    async fn count_complaints(&self) -> Result<i64, ServiceError>;
}

impl<Repo: ComplaintRepositoryExt + Sync + Send> ComplaintServiceExt for ComplaintService<Repo> {
    async fn create_complaint(
        &self,
        command: CreateComplaintCommand,
    ) -> Result<complaints::Model, ServiceError> {
        Ok(self.repository.create_complaint(&command).await?)
    }

    async fn find_all_complaints(
        &self,
    ) -> Result<Vec<(complaints::Model, Option<customers::Model>)>, ServiceError> {
        Ok(self.repository.find_all_complaints().await?)
    }

    async fn find_complaint_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<(complaints::Model, Option<customers::Model>), ServiceError> {
        Ok(self
            .repository
            .find_complaint_by_identifier(identifier)
            .await?)
    }

    async fn update_complaint(
        &self,
        identifier: &str,
        command: UpdateComplaintCommand,
    ) -> Result<complaints::Model, ServiceError> {
        let status = command
            .status
            .as_deref()
            .map(ComplaintStatus::try_from)
            .transpose()
            .map_err(|e| ServiceError::OperationFailed(e.to_string()))?;
        Ok(self
            .repository
            .update_complaint(
                identifier,
                command.subject.as_deref(),
                command.description.as_deref(),
                status,
            )
            .await?)
    }

    async fn delete_complaint(&self, identifier: &str) -> Result<(), ServiceError> {
        Ok(self
            .repository
            .delete_complaint_by_identifier(identifier)
            .await?)
    }

    async fn count_complaints(&self) -> Result<i64, ServiceError> {
        Ok(self.repository.count_complaints().await?)
    }
}
