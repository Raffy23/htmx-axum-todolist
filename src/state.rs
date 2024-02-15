use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::repository::{sqlite::SqliteTodoRepository, TodoRepository};

#[derive(Clone)]
pub struct AppState {
    pub repository: Arc<dyn TodoRepository + Send + Sync>,
}

impl AppState {
    #[tracing::instrument]
    pub async fn default(db: Pool<Sqlite>) -> Self {
        Self {
            repository: Arc::new(SqliteTodoRepository::new(db).await),
        }
    }
}
