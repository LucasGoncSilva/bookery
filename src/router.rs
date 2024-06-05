use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{database::Database, handlers::author::hello};

pub fn router(db: Arc<Database>) -> Router {
    Router::new().route("/", get(hello)).with_state(db)
}
