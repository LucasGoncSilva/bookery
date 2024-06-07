use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::Database;
use crate::structs::author::{Author, NewAuthor};

use super::QueryURL;

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

pub async fn get_author(
    State(db): State<Arc<Database>>,
    Path(author_uuid): Path<Uuid>,
) -> Result<(StatusCode, Json<Author>), StatusCode> {
    match db.get_author(author_uuid).await {
        Ok(Some(author)) => Ok((StatusCode::OK, Json(author))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_authors(
    State(db): State<Arc<Database>>,
    Query(t): Query<QueryURL>,
) -> Result<(StatusCode, Json<Vec<Author>>), StatusCode> {
    match db.search_authors(t).await {
        Ok(authors_vec) => Ok((StatusCode::OK, Json(authors_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
