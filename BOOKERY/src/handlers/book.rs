use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::Database;
use crate::structs::book::{Book, PayloadBook, PayloadUpdateBook};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_book(
    State(db): State<DB>,
    Json(incoming_book): Json<PayloadBook>,
) -> ResultStatus<Uuid> {
    match Book::create(incoming_book) {
        Ok(book) => match db.create_book(book).await {
            Ok(book_uuid) => Ok((StatusCode::CREATED, Json(book_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_book(State(db): State<DB>, Path(book_uuid): Path<Uuid>) -> ResultStatus<Book> {
    match db.get_book(book_uuid).await {
        Ok(Some(book)) => Ok((StatusCode::OK, Json(book))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_books(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Book>> {
    match db.search_books(t.name).await {
        Ok(books_vec) => Ok((StatusCode::OK, Json(books_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_book(
    State(db): State<DB>,
    Json(payload_update_book): Json<PayloadUpdateBook>,
) -> ResultStatus<Uuid> {
    match db.get_book_id(payload_update_book.id).await {
        Ok(Some(_book_uuid)) => match Book::parse(payload_update_book) {
            Ok(updated_book) => match db.update_book(updated_book).await {
                Ok(book_uuid) => Ok((StatusCode::ACCEPTED, Json(book_uuid))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_book(
    State(db): State<DB>,
    Json(incoming_struct): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_book(incoming_struct.id).await {
        Ok(Some(book)) => match db.delete_book(book.id).await {
            Ok(book_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Book {book_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn count_books(State(db): State<DB>) -> ResultStatus<i64> {
    match db.count_books().await {
        Ok(num) => Ok((StatusCode::OK, Json(num))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
