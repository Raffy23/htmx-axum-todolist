use askama_axum::IntoResponse;
use axum_session::{Session, SessionSqlitePool};
use uuid::Uuid;

use super::SettingsFragment;

pub async fn handle_settings(
    session: Session<SessionSqlitePool>,
) -> impl IntoResponse {
    let user_id: Uuid = {
        if let Some(user_id) = session.get("user") {
            user_id
        } else {
            let new_id = Uuid::new_v4();
            session.set("user", new_id);

            new_id
        }
    };

    SettingsFragment {
        show_settings: true,
        user_id: user_id.to_string()
     }
}
