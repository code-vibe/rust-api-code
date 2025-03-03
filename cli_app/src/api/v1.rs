use std::sync::Arc;
use super::handlers;
use axum::routing::get;
use axum::Router;
use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state)
        )
}