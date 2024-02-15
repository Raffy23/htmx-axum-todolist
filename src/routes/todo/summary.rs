use axum::extract::State;
use axum_session::{Session, SessionSqlitePool};

use super::SummaryFragment;
use crate::state::AppState;

pub async fn todos_summary(State(state): State<AppState>, session: Session<SessionSqlitePool>,) -> SummaryFragment {
    let user = session.get("user").unwrap();

    SummaryFragment {
        oob: false,
        todos: state.repository.query(user).await,
    }
}
