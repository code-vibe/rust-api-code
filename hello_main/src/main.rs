use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_json));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listner,app).await.unwrap();

}

async  fn hello() -> &'static str { "Hello, World!.. What's up?" }

async fn hello_json() -> (StatusCode, Json<Response>){
    let res = Response{
        message: "Hello, World!.. What's up?",
    };
    (StatusCode::OK, Json(res))
}
#[derive(Serialize)]
struct Response {
    message: &'static str,
}