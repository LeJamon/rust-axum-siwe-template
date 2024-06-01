use axum::Router;
mod example_route;
mod siwe;

pub fn routes() -> Router {
    // to add your routes, add .merge(your_routes::routes())
    Router::new()
        .merge(siwe::routes())
        .merge(example_route::routes())
}
