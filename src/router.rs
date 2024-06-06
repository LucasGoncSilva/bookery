use std::sync::Arc;

use axum::{routing::post, Router};

use crate::{database::Database, handlers::author::create_author};

pub fn router(db: Arc<Database>) -> Router {
    Router::new().route("/author/create", post(create_author)).with_state(db)
}
