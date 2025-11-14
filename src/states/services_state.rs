use axum::extract::FromRef;

use crate::services::{
    auth_service::AuthenticationService, marketplace_service::MarketplaceService, root_service::RootService, user_service::UserService
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
    pub marketplace_service: MarketplaceService,
}

impl FromRef<ServicesState> for UserService {
    fn from_ref(input: &ServicesState) -> UserService {
        input.user_service.clone()
    }
}

impl FromRef<ServicesState> for RootService {
    fn from_ref(input: &ServicesState) -> RootService {
        input.root_service.clone()
    }
}

impl FromRef<ServicesState> for AuthenticationService {
    fn from_ref(input: &ServicesState) -> AuthenticationService {
        input.auth_service.clone()
    }
}

impl FromRef<ServicesState> for MarketplaceService {
    fn from_ref(input: &ServicesState) -> MarketplaceService {
        input.marketplace_service.clone()
    }
}