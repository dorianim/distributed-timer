use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::{EmbeddedFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "web/build"]
struct WebAssets;

pub async fn web_assets(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    match WebAssets::get(path) {
        Some(file) => response(file, path),
        None => index(),
    }
}

fn index() -> Response {
    match WebAssets::get("200.html") {
        Some(file) => response(file, "200.html"),
        None => not_found(),
    }
}

fn response(file: EmbeddedFile, path: &str) -> Response {
    let body = boxed(Full::from(file.data));
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(body)
        .unwrap()
}

fn not_found() -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(boxed(Full::from("404")))
        .unwrap()
}
