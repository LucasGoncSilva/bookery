use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Error as SqlxErr, PgPool, Pool, Postgres, Row,
};
use uuid::Uuid;

use crate::structs::author::Author;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(url: &str) -> Self {
        let pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(30)
            .connect(url)
            .await
            .unwrap();

        sqlx::migrate!("./src/migrations").run(&pool).await.unwrap();

        Self { pool }
    }

    pub async fn create_author(&self, author: Author) -> Result<Uuid, SqlxErr> {
        let author_uuid: Uuid = sqlx::query(
            "
            INSERT INTO tbl_authors (id, name, born)
            VALUE ($1, $2, $3)
            RETURNING id
        ",
        )
        .bind(String::from(author.id))
        .bind(author.name.as_str())
        .bind(author.born.as_str())
        .map(|row: PgRow| Uuid::parse_str(row.get("id")).unwrap())
        .fetch_one(&self.pool)
        .await?;

        Ok(author_uuid)
    }
}
