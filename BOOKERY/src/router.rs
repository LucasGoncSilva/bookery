use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::Database,
    handlers::{
        author::{
            count_authors, create_author, delete_author, get_author, search_authors, update_author,
        },
        book::{count_books, create_book, delete_book, get_book, search_books, update_book},
    },
};

pub fn router(db: Arc<Database>) -> Router {
    Router::new()
        // Authors
        .route("/author/new", post(create_author))
        .route("/author/get/:id", get(get_author))
        .route("/author/search", get(search_authors))
        .route("/author/update", post(update_author))
        .route("/author/delete", post(delete_author))
        .route("/author/count", get(count_authors))
        // Books
        .route("/book/new", post(create_book))
        .route("/book/get/:id", get(get_book))
        .route("/book/search", get(search_books))
        .route("/book/update", post(update_book))
        .route("/book/delete", post(delete_book))
        .route("/book/count", get(count_books))
        // Database Sync
        .with_state(db)
}
