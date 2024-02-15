use async_trait::async_trait;
use sqlx::{Pool, Row, Sqlite};
use tracing::info;
use uuid::Uuid;

use crate::model::todo::Todo;

use super::TodoRepository;

#[derive(Debug)]
pub struct SqliteTodoRepository {
    db: Pool<Sqlite>,
}

const TABLE_DDL: &str = r#"
CREATE TABLE IF NOT EXISTS Todo (
    user    BLOB(16)    NOT NULL,
    id      BLOB(16)    NOT NULL,
    name    TEXT        NOT NULL,
    checked BOOLEAN     NOT NULL,

    CONSTRAINT PK_Todo PRIMARY KEY (user, id)
);
"#;

impl SqliteTodoRepository {
    #[tracing::instrument]
    pub async fn new(db: Pool<Sqlite>) -> Self {
        let result = sqlx::query(TABLE_DDL).execute(&db).await.unwrap();
        info!("Created Todo table: {:?}", result);

        Self { db }
    }
}

#[async_trait]
impl TodoRepository for SqliteTodoRepository {
    #[tracing::instrument(skip(self))]
    async fn query(&self, user: Uuid) -> Vec<Todo> {
        sqlx::query("SELECT id, name, checked FROM Todo WHERE user = ?")
            .bind(user.as_bytes().to_vec())
            .fetch_all(&self.db)
            .await
            .unwrap()
            .into_iter()
            .map(|row| Todo {
                id: uuid::Uuid::from_bytes(row.get::<Vec<u8>, &str>("id").try_into().unwrap()),
                name: row.get::<String, &str>("name"),
                checked: row.get::<bool, &str>("checked"),
            })
            .collect()
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, user: Uuid, uuid: Uuid) -> bool {
        sqlx::query("DELETE FROM Todo WHERE user = ? AND id = ?")
            .bind(user.as_bytes().to_vec())
            .bind(uuid.as_bytes().to_vec())
            .execute(&self.db)
            .await
            .unwrap()
            .rows_affected()
            > 0
    }

    #[tracing::instrument(skip(self))]
    async fn insert(&self, user: Uuid, todo: Todo) -> bool {
        sqlx::query("INSERT INTO Todo (user, id, name, checked) VALUES (?, ?, ?, ?)")
            .bind(user.as_bytes().to_vec())
            .bind(todo.id.as_bytes().to_vec())
            .bind(todo.name)
            .bind(todo.checked)
            .execute(&self.db)
            .await
            .unwrap()
            .rows_affected()
            == 1
    }

    #[tracing::instrument(skip(self))]
    async fn update(&self, user: Uuid, uuid: Uuid, checked: bool) -> bool {
        sqlx::query("UPDATE Todo SET checked = ? WHERE user = ? AND id = ?")
            .bind(checked)
            .bind(user.as_bytes().to_vec())
            .bind(uuid.as_bytes().to_vec())
            .execute(&self.db)
            .await
            .unwrap()
            .rows_affected()
            == 1
    }

    #[tracing::instrument(skip(self))]
    async fn clear(&self) {
        sqlx::query("DELETE FROM Todo").execute(&self.db).await.unwrap();
    }
}
