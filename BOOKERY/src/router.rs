use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::Database,
    handlers::author::{count_authors, create_author, delete_author, get_author, search_authors},
};

pub fn router(db: Arc<Database>) -> Router {
    Router::new()
        .route("/author/new", post(create_author))
        .route("/author/get/:id", get(get_author))
        .route("/author/search", get(search_authors))
        .route("/author/delete", post(delete_author))
        .route("/author/count", get(count_authors))
        .with_state(db)
}
