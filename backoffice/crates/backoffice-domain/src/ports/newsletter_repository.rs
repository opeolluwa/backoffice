use crate::errors::database_error::DatabaseError;
use crate::models::newsletter;

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
