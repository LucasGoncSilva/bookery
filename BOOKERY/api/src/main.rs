use std::{env, sync::Arc};

use axum::Router;

use tokio::net::TcpListener;

mod database;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    let db_url: String = env::var("DATABASE_URL").unwrap_or(String::from(
        "postgres://postgres:postgres@localhost:5432/postgres",
    ));
    let conn: database::conn::Database = database::conn::Database::conn(&db_url).await;
    let db: Arc<database::conn::Database> = Arc::new(conn);

    let app: Router = router::router(db);

    let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
