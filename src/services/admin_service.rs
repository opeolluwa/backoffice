use crate::services::auth_service::AuthenticationService;
use crate::services::user_service::UserService;
use sqlx::{Pool, Postgres};

#[allow(dead_code)]
pub struct AdminService {
    user_service: UserService,
    authentication_service: AuthenticationService,
}

impl AdminService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        AdminService {
            user_service: UserService::init(pool),
            authentication_service: AuthenticationService::init(pool),
        }
    }
}

pub trait AdminServiceExt {
    fn invite_user();
}

impl AdminServiceExt for AdminService {
    fn invite_user() {
        unimplemented!()
    }
}
