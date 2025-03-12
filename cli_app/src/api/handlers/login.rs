use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::api::request::login::LoginRequest;
use crate::api::response::login::LoginResponse;
use crate::api::response::TokenClaims;
use crate::state::ApplicationState;

pub async fn login(
    State(_state): State<Arc<ApplicationState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims = TokenClaims {
        sub: payload.username,
        exp,
        iat,
    };

    let secret = "secret";

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).unwrap();

    let response = LoginResponse {
        status: "success".to_string(),
        token,
    };

    Ok(Json(response))
}

