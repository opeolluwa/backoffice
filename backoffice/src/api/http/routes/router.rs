use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    config::app_config::AppConfig,
    routes::{
        auth::authentication_routes, country::country_routes, marketplace::marketplace_routes,
        products::product_routes, public::public_routes, teams::team_routes, users::user_routes,
    },
    services::{
        auth_service::AuthenticationService, country_service::CountryService,
        marketplace_service::MarketplaceService, product_service::ProductService,
        root_service::RootService, team_service::TeamService, user_service::UserService,
    },
    states::ServicesState,
};

pub fn load_routes(db: DatabaseConnection, app_config: &AppConfig) -> Router {
    let state = ServicesState {
        user_service: UserService::init(&db, app_config),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(&db, app_config),
        marketplace_service: MarketplaceService::init(&db),
        product_service: ProductService::init(&db),
        country_service: CountryService::init(&db),
        team_service: TeamService::init(&db),
    };

    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    Router::new()
        .nest(
            "/api",
            Router::new()
                .merge(user_routes(state.clone()))
                .merge(public_routes(state.clone()))
                .merge(authentication_routes(state.clone()))
                .merge(marketplace_routes(state.clone()))
                .merge(product_routes(state.clone()))
                .merge(country_routes(state.clone()))
                .merge(team_routes(state.clone())),
        )
        .fallback_service(serve_dir)
}
