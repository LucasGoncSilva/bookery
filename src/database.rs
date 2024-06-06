use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Error as SqlxErr, PgPool, Pool, Postgres, Row,
};
use uuid::Uuid;

use crate::structs::{author::Author, BornDate, PersonName};

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

    pub async fn get_author(&self, author_uuid: Uuid) -> Result<Option<Author>, SqlxErr> {
        let author = sqlx::query(
            "
            SELECT id, name, born
            FROM tbl_authors
            WHERE id = $1
        ",
        )
        .bind(String::from(author_uuid))
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");
            let born_parser: String = row.get("born");

            let id: Uuid = Uuid::parse_str(row.get("id")).unwrap();
            let name: PersonName = PersonName::try_from(name_parser).unwrap();
            let born: BornDate = BornDate::try_from(born_parser).unwrap();

            Author { id, name, born }
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(author)
    }
}
