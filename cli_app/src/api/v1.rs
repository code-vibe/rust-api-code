use axum::Router;
use axum::routing::get;
use crate::api::handlers;

pub fn configure() -> Router {
    Router::new().route("/hello", get(handlers::hello::hello))
}