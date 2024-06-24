use std::sync::Arc;

use axum::{http::StatusCode, Json};
use serde::Deserialize;
use uuid::Uuid;

use crate::database::conn::Database;

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

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
