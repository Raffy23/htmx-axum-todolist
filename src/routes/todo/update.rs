use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Form,
};
use axum_session::{Session, SessionSqlitePool};
use serde_derive::Deserialize;
use uuid::Uuid;

use super::summary_fragment_oob_response;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct UpdateTodoForm {
    pub done: Option<String>,
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(todo_id): Path<Uuid>,
    session: Session<SessionSqlitePool>,
    Form(form): Form<UpdateTodoForm>,
) -> impl IntoResponse {
    let user = session.get("user").unwrap();
    let todo_updated = state.repository.update(user, todo_id, form.done.is_some()).await;

    summary_fragment_oob_response(user, todo_updated, state.repository).await
}
