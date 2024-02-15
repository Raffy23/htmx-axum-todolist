use async_trait::async_trait;
use uuid::Uuid;

use crate::model::todo::Todo;

pub mod sqlite;

#[async_trait]
pub trait TodoRepository {
    async fn query(&self, user: Uuid) -> Vec<Todo>;
    async fn delete(&self, user: Uuid, uuid: Uuid) -> bool;
    async fn insert(&self, user: Uuid, todo: Todo) -> bool;
    async fn update(&self, user: Uuid, uuid: Uuid, checked: bool) -> bool;
    async fn clear(&self);
}
