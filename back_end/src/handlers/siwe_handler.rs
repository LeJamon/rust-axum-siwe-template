use crate::cache::{get_nonce, set_nonce};
use crate::models::{
    claims::Claims, error::AppError, jwt::Jwt, siwe_login_payload::SiweLoginPayload,
};
use axum::{http::StatusCode, response::Json};
use hex::{encode, FromHex};
use jsonwebtoken::{encode as encodeJWT, EncodingKey, Header};
use siwe::{generate_nonce, Message, VerificationOpts};
use std::str::FromStr;

/**
* handle a nonce request
* generate a random nonce, using siwe crate and stored it in the cache
*/
pub async fn handle_nonce_request() -> (StatusCode, Json<String>) {
    let random_nonce = generate_nonce();
    set_nonce(random_nonce.clone(), random_nonce.clone()).await;
    (StatusCode::OK, Json(random_nonce))
}

pub async fn handle_siwe_login(
    Json(payload): Json<SiweLoginPayload>,
) -> Result<(StatusCode, Json<String>), AppError> {
    let message = Message::from_str(&payload.message)
        .map_err(|_| AppError::ValidationError("Invalid SIWE message".to_string()))?;
    let signature = Vec::from_hex(&payload.signature)
        .map_err(|_| AppError::ValidationError("Invalid signature format".to_string()))?;
    let nonce = get_nonce(&message.nonce).await.ok_or(AppError::NotFound)?;
    let verification_opts = VerificationOpts {
        nonce: Some(nonce),
        ..Default::default()
    };
    message
        .verify(&signature, &verification_opts)
        .await
        .map_err(|_| AppError::Unauthorized)?;
    let claims = Claims::new(encode(message.address));
    let token = encodeJWT(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|_| AppError::InternalServerError)?;
    Ok((StatusCode::OK, Json(Jwt::new(token).to_string())))
}
