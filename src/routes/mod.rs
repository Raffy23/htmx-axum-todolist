use axum::Router;
use axum_session::{SessionSqlitePool, SessionStore};

use crate::state::AppState;

pub mod assets;
pub mod todo;
pub mod session;

#[inline]
pub fn todo_routes(session_store: SessionStore::<SessionSqlitePool>) -> Router<AppState> {
    Router::new()
        .merge(assets::router())
        .merge(todo::router(session_store.clone()))
        .merge(session::router(session_store.clone()))
}
