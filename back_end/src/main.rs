use axum::Router;
use back_end::api::routes;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/api", routes())
        .layer(CorsLayer::permissive());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
