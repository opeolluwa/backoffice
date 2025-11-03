use sqlx::{Pool, Postgres};
use crate::adapters::requests::auth::CreateUserRequest;
use crate::errors::app_error::AppError;
use crate::services::auth_service::{AuthenticationService, AuthenticationServiceTrait};
use crate::services::user_service::{UserService, UserServiceTrait};

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
    fn create_super_admin(&self, super_admin: &CreateUserRequest) -> impl std::future::Future<Output=Result<(), AppError>> + Send;
    fn invite_user();
}


impl AdminServiceExt for AdminService {
    async fn create_super_admin(&self, request: &CreateUserRequest) -> Result<(), AppError> {
        let super_admin = self.user_service.find_user_by_email(&request.email).await.ok();
        if super_admin.is_some() {
            return Ok(());
        }
        self.authentication_service.create_super_admin_user(&request).await.map_err(|err| {
            AppError::OperationFailed(err.to_string())
        })
    }

    fn invite_user() {
        unimplemented!()
    }
}