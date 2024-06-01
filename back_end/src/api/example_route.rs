use crate::handlers::example_handler::handle_example_request;
use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/address", get(handle_example_request))
}
