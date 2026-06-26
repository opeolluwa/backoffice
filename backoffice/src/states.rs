use async_graphql::dynamic::Schema;
use axum::extract::FromRef;
use seaography::async_graphql;

use crate::services::{
    auth_service::AuthenticationService, country_service::CountryService,
    marketplace_service::MarketplaceService, product_service::ProductService,
    root_service::RootService, team_service::TeamService, user_service::UserService,
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
    pub marketplace_service: MarketplaceService,
    pub product_service: ProductService,
    pub country_service: CountryService,
    pub team_service: TeamService,
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

impl FromRef<ServicesState> for ProductService {
    fn from_ref(input: &ServicesState) -> ProductService {
        input.product_service.clone()
    }
}

impl FromRef<ServicesState> for CountryService {
    fn from_ref(input: &ServicesState) -> CountryService {
        input.country_service.clone()
    }
}

impl FromRef<ServicesState> for TeamService {
    fn from_ref(input: &ServicesState) -> TeamService {
        input.team_service.clone()
    }
}

#[derive(Clone)]
pub struct GraphQlState {
    pub schema: Schema,
    pub endpoint: String,
}
