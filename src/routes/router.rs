use axum::Router;
use sqlx::{Pool, Postgres};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    routes::{
        auth::authentication_routes, marketplace::marketplace_routes, products::product_routes, public::public_routes, users::user_routes
    },
    services::{
        auth_service::AuthenticationService, marketplace_service::MarketplaceService,
        product_service::ProductService, root_service::RootService, user_service::UserService,
    },
    states::services_state::ServicesState,
};

pub fn load_routes(pool: Pool<Postgres>) -> Router {
    let state = ServicesState {
        user_service: UserService::init(&pool),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(&pool),
        marketplace_service: MarketplaceService::init(&pool),
        product_service: ProductService::init(&pool),
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
                .merge(product_routes(state.clone())),
        )
        .fallback_service(serve_dir)
}
