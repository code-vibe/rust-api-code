use axum::http::StatusCode;

pub async fn hello() -> Result<String, StatusCode> {
    Ok("Hello world!".to_string())
}