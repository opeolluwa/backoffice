pub mod graphql;
pub mod http;
mod state;

pub use http::routes::router::load_routes as load_http_routes;
pub use graphql::query_root::schema as load_graphql_routes;