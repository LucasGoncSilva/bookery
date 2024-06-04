use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

struct Database {
    pool: PgPool,
}

impl Database {
    async fn new(url: String) -> Self {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(30)
            .connect(&url)
            .await
            .unwrap();

        Self { pool }
    }
}
