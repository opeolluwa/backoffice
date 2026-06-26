use sea_orm::DatabaseConnection;

use crate::services::user_service::UserService;
use crate::{config::app_config::AppConfig, services::auth_service::AuthenticationService};

#[allow(dead_code)]
pub struct AdminService {
    user_service: UserService,
    authentication_service: AuthenticationService,
}

impl AdminService {
    pub fn init(db: &DatabaseConnection, app_config: &AppConfig) -> Self {
        AdminService {
            user_service: UserService::init(db, app_config),
            authentication_service: AuthenticationService::init(db, app_config),
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
