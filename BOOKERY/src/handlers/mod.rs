use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct QueryURL {
    pub name: String,
}

#[derive(Deserialize)]
pub struct DeletingStruct {
    id: Uuid,
}

pub mod author;
pub mod book;
