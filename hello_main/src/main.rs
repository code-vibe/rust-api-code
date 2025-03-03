use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;
use anyhow::Context;
use axum::response::IntoResponse;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7878")
        .await
        .context("failed to bind TCP listener")?    ;

    axum::serve(listener, app).await.context("axum::server failed to start")?;
    Ok(())

}

async  fn hello() -> &'static str { "Hello, World!.. What's up?" }

async fn hello_json() -> Result<(StatusCode, Json<Response>), AppError>{
    let res = Response{
        message: generate_message().context("Failed to generate message")?,
    };
    Ok((StatusCode::OK, Json(res)))
}

//Generates the hello world message
fn generate_message() -> anyhow::Result<&'static str> {
    if rand::random() {
        anyhow::bail!("No message for you");
    }
    Ok("Hello, World!")
}
#[derive(Serialize)]
struct Response {
    message: &'static str,
}

struct AppError(anyhow::Error);

//This allows ? to automatically convert anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        Self(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

