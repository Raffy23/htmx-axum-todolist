use askama_axum::IntoResponse;
use axum::{extract::State, Form};
use axum_session::{Session, SessionSqlitePool};
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

use super::LoginFormFragment;

#[derive(Deserialize)]
pub struct LoginForm {
    pub user_secret: Uuid,
}

pub async fn handle_login(
    State(state): State<AppState>,
    session: Session<SessionSqlitePool>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    session.set("user", form.user_secret);

    LoginFormFragment {
       todos: state.repository.query(form.user_secret).await,
    }
}
