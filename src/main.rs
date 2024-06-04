use axum::Router;

use tokio::net::TcpListener;

mod database;
mod handlers;
mod router;
mod structs;

#[tokio::main]
async fn main() {
    let app: Router = router::router();

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
