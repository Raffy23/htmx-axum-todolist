use askama::Template;
use axum::{
    routing::{get, post},
    Router,
};
use axum_session::{SessionLayer, SessionSqlitePool, SessionStore};

mod login;
mod logout;
mod settings;

pub use login::*;
pub use logout::*;
pub use settings::*;

use crate::{model::todo::Todo, state::AppState};
use crate::routes::todo::unchecked_todos;

pub fn router(session_store: SessionStore<SessionSqlitePool>) -> Router<AppState> {
    Router::new()
        .route("/login", post(handle_login))
        .route("/logout", post(handle_logout))
        .route("/settings", get(handle_settings))
        .layer(SessionLayer::new(session_store))
}

#[derive(Template)]
#[template(path = "_login.fragment.html")]
pub struct LoginFormFragment {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "settings.fragment.html")]
pub struct SettingsFragment {
    pub show_settings: bool,
    pub user_id: String,
}

#[derive(Template)]
#[template(path = "_logout.fragment.html")]
pub struct LogoutFragment {}
