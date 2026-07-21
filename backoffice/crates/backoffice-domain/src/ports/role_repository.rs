use crate::{errors::database_error::DatabaseError, models::user_roles};

#[cfg_attr(test, mockall::automock)]
#[allow(async_fn_in_trait)]
pub trait RoleRepositoryExt {
    async fn find_role_by_identifier(
        &self,
        identifier: &str,
    ) -> Result<Option<user_roles::Model>, DatabaseError>;

    async fn find_role_by_name(
        &self,
        name: &str,
    ) -> Result<Option<user_roles::Model>, DatabaseError>;

    async fn create_role(
        &self,
        identifier: &str,
        name: &str,
        description: Option<String>,
    ) -> Result<user_roles::Model, DatabaseError>;
}
