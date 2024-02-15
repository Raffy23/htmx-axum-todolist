use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub checked: bool,
}
