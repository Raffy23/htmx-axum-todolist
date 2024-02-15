use askama_axum::IntoResponse;
use axum_session::{Session, SessionSqlitePool};

use super::LogoutFragment;

pub async fn handle_logout(session: Session<SessionSqlitePool>) -> impl IntoResponse {
    session.destroy();

    LogoutFragment {}
}
