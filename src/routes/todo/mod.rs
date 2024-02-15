mod create;
mod delete;
mod index;
mod summary;
mod update;

use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    http::StatusCode,
    routing::{delete, get, patch, post},
    Router,
};
use axum_session::{SessionLayer, SessionSqlitePool, SessionStore};
use uuid::Uuid;
use std::sync::Arc;

pub use create::*;
pub use delete::*;
pub use index::*;
pub use summary::*;
pub use update::*;

use crate::{model::todo::Todo, repository::TodoRepository, state::AppState};

#[inline]
pub fn router(session_store: SessionStore<SessionSqlitePool>) -> Router<AppState> {
    Router::new()
        .route("/", get(handle_index))
        .route("/todo", post(create_todo))
        .route("/todo/:todo_id", patch(update_todo))
        .route("/todo/:todo_id", delete(delete_todo))
        .route("/todo-summary", get(todos_summary))
        .layer(SessionLayer::new(session_store))
}

#[derive(Template)]
#[template(path = "_create-todo.fragment.html")]
pub struct CreateTodoFragment {
    pub todo: Todo,
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "todo.fragment.html")]
pub struct TodoFragment {
    pub todo: Todo,
}

#[derive(Template)]
#[template(path = "todo-form.fragment.html")]
pub struct TodoFormFragment {
    pub oob: bool,
}

#[derive(Template)]
#[template(path = "todo-summary.fragment.html")]
pub struct SummaryFragment {
    pub oob: bool,
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct TodoListIndex<'a> {
    pub title: &'a str,
    pub partial: bool,
    pub todos: Vec<Todo>,
    pub has_user: bool,
    pub integrity_htmx_js: String,
}

#[tracing::instrument(skip(todos))]
pub fn unchecked_todos(todos: &[Todo]) -> usize {
    todos.into_iter().filter(|todo| !todo.checked).count()
}

pub async fn summary_fragment_oob_response(
    user: Uuid,
    ok: bool,
    repository: Arc<dyn TodoRepository + Sync + Send>,
) -> impl IntoResponse {
    (
        if ok {
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        },
        SummaryFragment {
            oob: true,
            todos: repository.query(user).await,
        },
    )
}
