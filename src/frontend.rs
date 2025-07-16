use axum::body::Body;
use axum::http::Response;
use axum::http::StatusCode;
use axum::http::Uri;
use axum::http::header;
use axum::response::IntoResponse;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "resources/frontend/build/"]
pub struct Frontend;

impl Frontend {
    pub async fn serve(uri: Uri) -> impl IntoResponse {
        let file_path = uri.path().trim_start_matches("/");
        let file = Frontend::get(file_path).or_else(|| Frontend::get("index.html"));
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
}
