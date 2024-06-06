use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::Database,
    handlers::author::{create_author, get_author},
};

pub fn router(db: Arc<Database>) -> Router {
    Router::new()
        .route("/author/new", post(create_author))
        .route("/author/get/:id", get(get_author))
        .with_state(db)
}
