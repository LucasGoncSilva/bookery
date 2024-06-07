use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryURL {
    pub name: Option<String>,
}

pub mod author;
