
mod handlers;
mod v1;
pub fn configure() -> Router {
    Router::new().nest("/v1", v1::Configure())
}