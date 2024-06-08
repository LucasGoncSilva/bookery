use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Error as SqlxErr, PgPool, Pool, Postgres, Row,
};
use time::Date;
use uuid::Uuid;

use crate::structs::{author::Author, PersonName};

type ResultDB<T> = Result<T, SqlxErr>;

pub struct Database {
    pool: PgPool,
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

    pub async fn create_author(&self, author: Author) -> ResultDB<Uuid> {
        let author_uuid: Uuid = sqlx::query(
            "
            INSERT INTO tbl_authors (id, name, born)
            VALUES ($1, $2, $3)
            RETURNING id
        ",
        )
        .bind(author.id)
        .bind(author.name.as_str())
        .bind(author.born)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(author_uuid)
    }

    pub async fn get_author(&self, author_uuid: Uuid) -> ResultDB<Option<Author>> {
        let author: Option<Author> = sqlx::query(
            "
            SELECT id, name, born
            FROM tbl_authors
            WHERE id = $1
        ",
        )
        .bind(author_uuid)
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");

            let id: Uuid = row.get("id");
            let name: PersonName = PersonName::try_from(name_parser).unwrap();
            let born: Date = row.get("born");

            Author { id, name, born }
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(author)
    }

    pub async fn search_authors(&self, terms: String) -> ResultDB<Vec<Author>> {
        let authors_vec: Vec<Author> = sqlx::query(
            "
            SELECT id, name, born
            FROM tbl_authors
            WHERE name ILIKE $1
        ",
        )
        .bind(format!("%{terms}%"))
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");

            let id: Uuid = row.get("id");
            let name: PersonName = PersonName::try_from(name_parser).unwrap();
            let born: Date = row.get("born");

            Author { id, name, born }
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(authors_vec)
    }

    pub async fn delete_author(&self, author_uuid: Uuid) -> ResultDB<Uuid> {
        let author_uuid: Uuid = sqlx::query(
            "
            DELETE FROM tbl_authors
            WHERE id = $1
            RETURNING id
        ",
        )
        .bind(author_uuid)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(author_uuid)
    }

    pub async fn count_authors(&self) -> ResultDB<i64> {
        let total: i64 = sqlx::query_scalar(
            "
            SELECT count(*) as total
            FROM tbl_authors
        ",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(total)
    }
}
