use std::sync::Arc;

use crate::domain::ports::user_repository::UserRepositoryTrait;
use crate::domain::services::auth::AuthenticationService;
use crate::domain::services::user::UserService;

#[allow(dead_code)]
pub struct AdminService<R: UserRepositoryTrait> {
    user_service: Arc<UserService<R>>,
    authentication_service: Arc<AuthenticationService<R>>,
}

impl<R: UserRepositoryTrait> AdminService<R> {
    pub fn new(
        user_service: Arc<UserService<R>>,
        authentication_service: Arc<AuthenticationService<R>>,
    ) -> Self {
        AdminService {
            user_service,
            authentication_service,
        }
    }
}

pub trait AdminServiceExt {
    fn invite_user();
}

impl<R: UserRepositoryTrait> AdminServiceExt for AdminService<R> {
    fn invite_user() {
        unimplemented!()
    }
}
