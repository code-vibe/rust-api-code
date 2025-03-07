use std::sync::Arc;
use super::handlers;
use axum::routing::{delete, get, post, put};
use axum::Router;
use crate::services;
use crate::state::ApplicationState;

pub fn configure(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route(
            "/hello",
            get(handlers::hello::hello).with_state(state.clone()),
        )
        .route(
            "/posts",
            post(handlers::posts::create).with_state(state.clone()),
        )
        .route(
            "/posts",
            get(handlers::posts::list).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            get(handlers::posts::get).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            put(handlers::posts::update).with_state(state.clone()),
        )
        .route(
            "/posts/:id",
            delete(handlers::posts::delete).with_state(state),
        )
}