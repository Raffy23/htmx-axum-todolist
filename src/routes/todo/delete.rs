use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use axum_session::{Session, SessionSqlitePool};
use uuid::Uuid;

use crate::state::AppState;

use super::summary_fragment_oob_response;

pub async fn delete_todo(
    Path(todo_id): Path<Uuid>,
    session: Session<SessionSqlitePool>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user = session.get("user").unwrap();
    let todo_deleted = state.repository.delete(user, todo_id).await;

    summary_fragment_oob_response(user, todo_deleted, state.repository).await
}
