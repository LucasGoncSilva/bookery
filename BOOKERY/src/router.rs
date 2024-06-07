use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::Database,
    handlers::author::{create_author, get_author, search_authors, delete_author},
};

pub fn router(db: Arc<Database>) -> Router {
    Router::new()
        .route("/author/new", post(create_author))
        .route("/author/get/:id", get(get_author))
        .route("/author/search", get(search_authors))
        .route("/author/delete", post(delete_author))
        .with_state(db)
}
