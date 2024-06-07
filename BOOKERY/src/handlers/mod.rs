use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryURL {
    pub name: String,
}

pub mod author;
