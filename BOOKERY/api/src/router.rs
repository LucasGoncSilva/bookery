use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::conn::Database,
    handlers::{
        author::{
            count_authors, create_author, delete_author, get_author, search_authors, update_author,
        },
        book::{count_books, create_book, delete_book, get_book, search_books, update_book},
        costumer::{
            count_costumers, create_costumer, delete_costumer, get_costumer, search_costumers,
            update_costumer,
        },
        rent::{count_rental, create_rent, delete_rent, get_rent, search_rental, update_rent},
    },
};

pub fn router(db: Arc<Database>) -> Router {
    Router::new()
        // Authors
        .route("/author/create", post(create_author))
        .route("/author/get/:id", get(get_author))
        .route("/author/search", get(search_authors))
        .route("/author/update", post(update_author))
        .route("/author/delete", post(delete_author))
        .route("/author/count", get(count_authors))
        // Books
        .route("/book/create", post(create_book))
        .route("/book/get/:id", get(get_book))
        .route("/book/search", get(search_books))
        .route("/book/update", post(update_book))
        .route("/book/delete", post(delete_book))
        .route("/book/count", get(count_books))
        // Costumers
        .route("/costumer/create", post(create_costumer))
        .route("/costumer/get/:id", get(get_costumer))
        .route("/costumer/search", get(search_costumers))
        .route("/costumer/update", post(update_costumer))
        .route("/costumer/delete", post(delete_costumer))
        .route("/costumer/count", get(count_costumers))
        // Rental
        .route("/rent/create", post(create_rent))
        .route("/rent/get/:id", get(get_rent))
        .route("/rent/search", get(search_rental))
        .route("/rent/update", post(update_rent))
        .route("/rent/delete", post(delete_rent))
        .route("/rent/count", get(count_rental))
        // Database Sync
        .with_state(db)
}
