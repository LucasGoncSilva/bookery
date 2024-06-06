use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::Database;
use crate::structs::author::{Author, NewAuthor};

pub async fn create_author(
    State(db): State<Arc<Database>>,
    Json(incoming_author): Json<NewAuthor>,
) -> Result<(StatusCode, Json<Uuid>), StatusCode> {
    match Author::new(incoming_author) {
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
        Ok(author) => match db.create_author(author).await {
            Ok(author_uuid) => Ok((StatusCode::CREATED, Json(author_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
