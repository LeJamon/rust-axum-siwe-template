use crate::models::claims::Claims;
use axum::http::StatusCode;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct Jwt {
    access_token: String,
    token_type: String,
}

impl Jwt {
    pub fn new(access_token: String) -> Self {
        Self {
            token_type: "Bearer".to_string(),
            access_token,
        }
    }
}

impl fmt::Display for Jwt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.token_type, self.access_token)
    }
}

pub async fn validate_jwt(authorization: &str) -> Result<Claims, (StatusCode, &'static str)> {
    if !authorization.starts_with("Bearer ") {
        return Err((StatusCode::UNAUTHORIZED, "Invalid authorization scheme"));
    }

    let token = authorization.trim_start_matches("Bearer ");
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;
    Ok(token_data.claims)
}
