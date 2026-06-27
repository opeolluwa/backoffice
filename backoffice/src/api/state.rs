use async_graphql::dynamic::Schema;
use axum::extract::FromRef;
use seaography::async_graphql;

use crate::{
    domain::services::{
        auth::AuthenticationService, country::CountryService, marketplace::MarketplaceService,
        product::ProductService, root::RootService, team::TeamService, user::UserService,
    },
    infrastructure::database::repositories::{
        country_repository::CountryRepository, marketplace_repository::MarketplaceRepository,
        product_repository::ProductRepository, team_repository::TeamRepository,
        user_repository::UserRepository,
    },
};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService<UserRepository>,
    pub root_service: RootService,
    pub auth_service: AuthenticationService<UserRepository>,
    pub marketplace_service: MarketplaceService<MarketplaceRepository>,
    pub product_service: ProductService<ProductRepository>,
    pub country_service: CountryService<CountryRepository>,
    pub team_service: TeamService<TeamRepository>,
}

impl FromRef<ServicesState> for UserService<UserRepository> {
    fn from_ref(input: &ServicesState) -> UserService<UserRepository> {
        input.user_service.clone()
    }
}

impl FromRef<ServicesState> for RootService {
    fn from_ref(input: &ServicesState) -> RootService {
        input.root_service.clone()
    }
}

impl FromRef<ServicesState> for AuthenticationService<UserRepository> {
    fn from_ref(input: &ServicesState) -> AuthenticationService<UserRepository> {
        input.auth_service.clone()
    }
}

impl FromRef<ServicesState> for MarketplaceService<MarketplaceRepository> {
    fn from_ref(input: &ServicesState) -> MarketplaceService<MarketplaceRepository> {
        input.marketplace_service.clone()
    }
}

impl FromRef<ServicesState> for ProductService<ProductRepository> {
    fn from_ref(input: &ServicesState) -> ProductService<ProductRepository> {
        input.product_service.clone()
    }
}

impl FromRef<ServicesState> for CountryService<CountryRepository> {
    fn from_ref(input: &ServicesState) -> CountryService<CountryRepository> {
        input.country_service.clone()
    }
}

impl FromRef<ServicesState> for TeamService<TeamRepository> {
    fn from_ref(input: &ServicesState) -> TeamService<TeamRepository> {
        input.team_service.clone()
    }
}

#[derive(Clone)]
pub struct GraphQlState {
    pub schema: Schema,
    pub endpoint: String,
}
