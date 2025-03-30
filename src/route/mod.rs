pub mod home;

use std::path::PathBuf;

use axum::{
    body::Body,
    http::{header, Response, StatusCode, Uri},
    response::IntoResponse,
};

#[derive(rust_embed::RustEmbed)]
#[folder = "./asset"]
struct Asset;

pub struct StaticFile(pub PathBuf);

impl IntoResponse for StaticFile {
    fn into_response(self) -> Response<Body> {
        let path = self.0.to_str();

        if let Some(path) = path {
            match Asset::get(path) {
                Some(content) => {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
                }
                None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
            }
        } else {
            (StatusCode::NOT_FOUND, "404 Not Found").into_response()
        }
    }
}

pub async fn asset(uri: Uri) -> Result<impl IntoResponse, StatusCode> {
    let path = uri.path().trim_start_matches("/").to_string();
    // HACK Asset fails to find file by relative path
    let path = std::fs::canonicalize(path).map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(StaticFile(path))
}
