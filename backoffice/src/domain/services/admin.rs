use crate::domain::ports::user_repository::UserRepositoryTrait;
use crate::domain::services::auth::AuthenticationService;
use crate::domain::services::user::UserService;

#[allow(dead_code)]
pub struct AdminService<R: UserRepositoryTrait + Clone> {
    user_service: UserService<R>,
    authentication_service: AuthenticationService<R>,
}

impl<R: UserRepositoryTrait + Clone> AdminService<R> {
    pub fn new(
        user_service: UserService<R>,
        authentication_service: AuthenticationService<R>,
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

impl<R: UserRepositoryTrait + Clone> AdminServiceExt for AdminService<R> {
    fn invite_user() {
        unimplemented!()
    }
}
