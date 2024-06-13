use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn conn(url: &str) -> Self {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(30)
            .connect(url)
            .await
            .unwrap();

        sqlx::migrate!("./src/migrations").run(&pool).await.unwrap();

        Self { pool }
    }
}
