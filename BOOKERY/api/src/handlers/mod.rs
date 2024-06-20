use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct QueryURL {
    pub token: String,
}

#[derive(Deserialize)]
pub struct DeletingStruct {
    id: Uuid,
}

pub mod author;
pub mod book;
pub mod costumer;
pub mod rental;
