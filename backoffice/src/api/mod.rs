pub mod graphql;
pub mod http;
pub mod state;

pub use graphql::query_root::schema as load_graphql_routes;
pub use http::routes::router::load_routes as load_http_routes;
