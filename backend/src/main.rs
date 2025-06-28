mod api;
mod frontend;

use std::net::SocketAddr;

use axum::{Router, routing::get};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
  // TODO: Configurable
  let address = SocketAddr::from(([127, 0, 0, 1], 8080));
  let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
  let api_routes = Router::new().route("/hello", get(api::hello));
  #[cfg(all(debug_assertions, not(rust_analyzer)))]
  let app = Router::new().nest("/api", api_routes).layer(cors);
  #[cfg(any(not(debug_assertions), rust_analyzer))]
  let app = Router::new()
    .nest("/api", api_routes)
    .layer(cors)
    .fallback(get(frontend::serve_frontend_asset));
  axum_server::bind(address)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
