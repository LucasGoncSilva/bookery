use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::Database;
use crate::structs::author::{Author, NewAuthor};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_author(
    State(db): State<DB>,
    Json(incoming_author): Json<NewAuthor>,
) -> ResultStatus<Uuid> {
    match Author::new(incoming_author) {
        Ok(author) => match db.create_author(author).await {
            Ok(author_uuid) => Ok((StatusCode::CREATED, Json(author_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_author(
    State(db): State<DB>,
    Path(author_uuid): Path<Uuid>,
) -> ResultStatus<Author> {
    match db.get_author(author_uuid).await {
        Ok(Some(author)) => Ok((StatusCode::OK, Json(author))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_authors(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Author>> {
    match db.search_authors(t.name).await {
        Ok(authors_vec) => Ok((StatusCode::OK, Json(authors_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_author(
    State(db): State<DB>,
    Json(author_uuid): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_author(author_uuid.id).await {
        Ok(Some(author)) => match db.delete_author(author.id).await {
            Ok(author_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Author {author_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
