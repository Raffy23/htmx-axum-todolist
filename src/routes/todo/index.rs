use askama_axum::IntoResponse;
use axum::{
    extract::State,
    http::{header, StatusCode},
};
use axum_session::{Session, SessionSqlitePool};
use base64::prelude::*;
use lazy_static::lazy_static;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::assets::HTMX_JS;
use crate::state::AppState;

use super::TodoListIndex;

lazy_static! {
    static ref HTMX_JS_CSP_HASH: String = BASE64_STANDARD.encode(Sha256::digest(HTMX_JS.as_bytes()));
}

pub fn csp_header_value() -> String {
    // styles generated by htmx:
    let inline_styles = ["'sha256-pgn1TCGZX6O77zDvy0oTODMOxemn0oj0LeCnQTRj7Kg='"].join(" ");
    let htmx_csp_hash: &String = &HTMX_JS_CSP_HASH;

    format!(
        r#"base-uri 'none'
        object-src 'none'
        script-src 'sha256-{htmx_csp_hash}' 'unsafe-inline'
        style-src 'self' {inline_styles}
        default-src 'self'
        img-src 'self' data:
        frame-ancestors 'self'
        form-action 'self'
        report-uri /csp-report"#
    )
    .replace("\n", ";")
}

pub async fn handle_index(
    session: Session<SessionSqlitePool>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user: Option<Uuid> = session.get("user");

    let todos = match user {
        Some(uuid) => state.repository.query(uuid).await,
        None => Vec::new(),
    };

    (
        StatusCode::OK,
        [(
            header::CONTENT_SECURITY_POLICY,
            csp_header_value(),
        )],
        TodoListIndex {
            title: "Hello World",
            partial: false,
            todos: todos.to_vec(),
            has_user: user.is_some(),
            integrity_htmx_js: format!("sha256-{}", HTMX_JS_CSP_HASH.as_str()),
        },
    )
}