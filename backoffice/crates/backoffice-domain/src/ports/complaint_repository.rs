use crate::{
    dto::CreateComplaintCommand,
    errors::database_error::DatabaseError,
    models::{complaints, customers, sea_orm_active_enums::ComplaintStatus},
};

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
