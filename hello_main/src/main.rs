use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:7878").await.unwrap();
    axum::serve(listner,app).await.unwrap();

}

async  fn hello() -> &'static str { "Hello, World!.. What's up?" }