use askama_axum::IntoResponse;
use axum::{extract::State, Form};
use axum_session::{Session, SessionSqlitePool};
use serde_derive::Deserialize;
use uuid::Uuid;

use super::CreateTodoFragment;
use crate::model::todo::Todo;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateTodoForm {
    pub todo_content: String,
}

pub async fn create_todo(
    State(state): State<AppState>,
    session: Session<SessionSqlitePool>,
    Form(form): Form<CreateTodoForm>,
) -> impl IntoResponse {
    // NOTE: We create user on first persist !
    let user = {
        if let Some(uuid) = session.get("user") {
            uuid
        } else {
            let uuid = Uuid::new_v4();
            session.set("user", uuid);

            uuid
        }
    };


    let new_todo = Todo {
        id: Uuid::new_v4(),
        name: form.todo_content,
        checked: false,
    };

    state.repository.insert(user, new_todo.clone()).await;
    let todos = state.repository.query(user).await;

    CreateTodoFragment {
        todo: new_todo,
        todos: todos.to_vec(),
    }
}
