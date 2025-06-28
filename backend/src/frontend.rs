// Embed and serve frontend assets in release builds,
// allowing the project to run as a single standalone binary.

#[cfg(any(not(debug_assertions), rust_analyzer))]
use axum::{
  body::Body, http::Response, http::StatusCode, http::Uri, http::header, response::IntoResponse,
};
#[cfg(any(not(debug_assertions), rust_analyzer))]
use rust_embed::Embed;

#[cfg(any(not(debug_assertions), rust_analyzer))]
#[derive(Embed)]
#[folder = "../frontend/build/"]
pub struct FrontendAssets;

#[cfg(any(not(debug_assertions), rust_analyzer))]
pub async fn serve_frontend_asset(uri: Uri) -> impl IntoResponse {
  let file_path = uri.path().trim_start_matches("/");
  let file = FrontendAssets::get(file_path).or_else(|| FrontendAssets::get("index.html"));
  match file {
    Some(content) => Response::builder()
      .header(header::CONTENT_TYPE, content.metadata.mimetype())
      .status(StatusCode::OK)
      .body(Body::from(content.data.into_owned()))
      .unwrap(),
    None => Response::builder()
      .status(StatusCode::NOT_FOUND)
      .body(Body::empty())
      .unwrap(),
  }
}
