use askama_axum::{IntoResponse, Response};
use axum::{
    extract::Path,
    http::{
        header::{self},
        HeaderMap, StatusCode,
    },
    routing::get,
    Router,
};
use blake3::{self, Hash};
use lazy_static::lazy_static;

use crate::assets::{APP_CSS, HTMX_JS, PICO_CSS};

lazy_static! {
    static ref HTMX_JS_HASH: Hash = blake3::hash(HTMX_JS.as_bytes());
    static ref PICO_CSS_HASH: Hash = blake3::hash(PICO_CSS.as_bytes());
    static ref APP_CSS_HASH: Hash = blake3::hash(APP_CSS.as_bytes());
}

pub async fn handle_assets(Path(file): Path<String>, request_headers: HeaderMap) -> Response {
    match file.as_str() {
        "htmx.min.js" => handle_file(
            request_headers,
            HTMX_JS,
            &HTMX_JS_HASH,
            "text/html; charset=utf-8",
        ),
        "pico.min.css" => handle_file(request_headers, PICO_CSS, &PICO_CSS_HASH, "text/css"),
        "app.css" => handle_file(request_headers, APP_CSS, &APP_CSS_HASH, "text/css"),

        _ => (StatusCode::NOT_FOUND, HeaderMap::new(), "").into_response(),
    }
}

pub fn router<T: Clone + Send + Sync + 'static>() -> Router<T> {
    Router::new().route("/assets/*file", get(handle_assets))
}

fn is_same_hash(request_headers: &HeaderMap, hash: &Hash) -> bool {
    request_headers.contains_key(header::IF_NONE_MATCH)
        && request_headers[header::IF_NONE_MATCH] == hash.to_string()
}

fn handle_file(
    request_headers: HeaderMap,
    content: &'static str,
    etag_hash: &Hash,
    content_type: &str,
) -> Response {
    if is_same_hash(&request_headers, etag_hash) {
        return (StatusCode::NOT_MODIFIED, HeaderMap::new(), "").into_response();
    }

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(header::ETAG, etag_hash.to_string().parse().unwrap());

    (StatusCode::OK, headers, content).into_response()
}
