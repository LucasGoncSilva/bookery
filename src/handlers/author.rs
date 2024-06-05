use std::sync::Arc;

use axum::{extract::State, http::StatusCode};

use crate::database::Database;

pub async fn hello(State(db): State<Arc<Database>>) -> Result<(StatusCode, String), StatusCode> {
    Ok((StatusCode::OK, String::from("Salve")))
}
