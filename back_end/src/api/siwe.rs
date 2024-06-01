use crate::handlers::siwe_handler::{handle_nonce_request, handle_siwe_login};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route("/nonce", get(handle_nonce_request))
        .route("/login", post(handle_siwe_login))
}
