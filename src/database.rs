use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Self {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(30)
            .connect(&url)
            .await
            .unwrap();

        sqlx::migrate!("./src/migrations").run(&pool).await.unwrap();

        Self { pool }
    }
}
