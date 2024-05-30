use axum::{routing::get, Router};

use crate::handlers::author::hello;

pub fn router() -> Router {
    Router::new().route("/", get(hello))
}
