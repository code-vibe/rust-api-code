use serde::{Deserialize, Serialize};

pub mod post;
pub mod login;



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}