use crate::{
    errors::service_error::ServiceError, models::newsletter,
    ports::newsletter_repository::NewsletterRepositoryExt,
};

#[derive(Clone)]
pub struct NewsletterService<R: NewsletterRepositoryExt> {
    repo: R,
}

impl<R: NewsletterRepositoryExt> NewsletterService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

pub trait NewsletterServiceExt {
    fn subscribe(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<newsletter::Model, ServiceError>>;

    fn unsubscribe(
        &self,
        email: &str,
    ) -> impl std::future::Future<Output = Result<(), ServiceError>>;
}

impl<R: NewsletterRepositoryExt + Send + Sync> NewsletterServiceExt for NewsletterService<R> {
    async fn subscribe(&self, email: &str) -> Result<newsletter::Model, ServiceError> {
        Ok(self.repo.subscribe(email).await?)
    }

    async fn unsubscribe(&self, email: &str) -> Result<(), ServiceError> {
        Ok(self.repo.unsubscribe(email).await?)
    }
}
