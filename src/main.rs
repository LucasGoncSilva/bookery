use std::{env, sync::Arc};

use axum::Router;

use tokio::net::TcpListener;

mod database;
mod handlers;
mod router;
mod structs;

#[tokio::main]
async fn main() {
    let db_url: String = env::var("DATABASE_URL").unwrap_or(String::from(
        "postgres://postgres:postgres@localhost:5432/postgres",
    ));
    let conn: database::Database = database::Database::conn(&db_url).await;
    let db: Arc<database::Database> = Arc::new(conn);

    let app: Router = router::router(db);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
