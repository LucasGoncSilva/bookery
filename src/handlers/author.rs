use axum::http::StatusCode;

pub async fn hello() -> Result<(StatusCode, String), StatusCode> {
    Ok((StatusCode::OK, String::from("Salve")))
}
