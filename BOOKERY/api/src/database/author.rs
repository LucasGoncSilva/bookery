use sqlx::{postgres::PgRow, Row};
use time::Date;
use uuid::Uuid;

use crate::database::{conn::Database, ResultDB};
use crate::structs::{author::Author, PersonName};

impl Database {
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

    pub async fn get_author_id(&self, author_uuid: Uuid) -> ResultDB<Option<Uuid>> {
        let author_uuid: Option<Uuid> = sqlx::query(
            "
            SELECT id
            FROM tbl_authors
            WHERE id = $1
        ",
        )
        .bind(author_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            id
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(author_uuid)
    }

    pub async fn search_authors(&self, token: String) -> ResultDB<Vec<Author>> {
        let authors_vec: Vec<Author> = sqlx::query(
            "
            SELECT id, name, born
            FROM tbl_authors
            WHERE name ILIKE $1
        ",
        )
        .bind(format!("%{token}%"))
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

    pub async fn update_author(&self, author: Author) -> ResultDB<Uuid> {
        let author_uuid: Uuid = sqlx::query(
            "
            UPDATE tbl_authors
            SET name = $1, born = $2
            WHERE id = $3
            RETURNING id
        ",
        )
        .bind(author.name.as_str())
        .bind(author.born)
        .bind(author.id)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(author_uuid)
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::var;

    use time::{error::ComponentRange, Date, Month};

    use crate::{
        handlers::QueryURL,
        structs::author::{PayloadAuthor, PayloadUpdateAuthor},
    };

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    async fn conn_db() -> Database {
        let db_url: String = var("DATABASE_URL").unwrap();
        Database::conn(&db_url).await
    }

    fn create_author() -> Author {
        let payload_author: PayloadAuthor = PayloadAuthor {
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        Author::create(payload_author).unwrap()
    }

    #[sqlx::test]
    async fn test_create_author() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let author_uuid: Uuid = author.id.clone();

        let sql_result: Uuid = db.create_author(author).await.unwrap();

        assert_eq!(sql_result, author_uuid);
    }

    #[sqlx::test]
    async fn test_get_author_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let author_uuid: Uuid = db.create_author(author.clone()).await.unwrap();

        let sql_result: Author = db.get_author(author_uuid.clone()).await.unwrap().unwrap();

        assert_eq!(sql_result, author);
    }

    #[sqlx::test]
    async fn test_get_author_not_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let sql_result: Option<Author> = db.get_author(author.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_get_author_id_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let author_uuid: Uuid = db.create_author(author.clone()).await.unwrap();

        let sql_result: Uuid = db
            .get_author_id(author_uuid.clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(sql_result, author_uuid);
    }

    #[sqlx::test]
    async fn test_get_author_id_not_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let sql_result: Option<Uuid> = db.get_author_id(author.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_search_authors_case_sensitive_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let token: QueryURL = QueryURL {
            token: "Nam".to_string(),
        };

        db.create_author(author.clone()).await.unwrap();

        let sql_result: Vec<Author> = db.search_authors(token.token).await.unwrap();

        assert!(sql_result.contains(&author));
    }

    #[sqlx::test]
    async fn test_search_authors_case_insensitive_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let token: QueryURL = QueryURL {
            token: "nam".to_string(),
        };

        db.create_author(author.clone()).await.unwrap();

        let sql_result: Vec<Author> = db.search_authors(token.token).await.unwrap();

        assert!(sql_result.contains(&author));
    }

    #[sqlx::test]
    async fn test_search_authors_not_found() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let token: QueryURL = QueryURL {
            token: "foo".to_string(),
        };

        db.create_author(author.clone()).await.unwrap();

        let sql_result: Vec<Author> = db.search_authors(token.token).await.unwrap();

        assert!(!sql_result.contains(&author));
    }

    #[sqlx::test]
    async fn test_update_author() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        let sql_author_uuid: Uuid = db.create_author(author.clone()).await.unwrap();

        let payload_update_author: PayloadUpdateAuthor = PayloadUpdateAuthor {
            id: sql_author_uuid.clone(),
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        db.update_author(Author::parse(payload_update_author).unwrap())
            .await
            .unwrap();

        let sql_result: Author = db.get_author(sql_author_uuid).await.unwrap().unwrap();

        assert_eq!(sql_result, author);
    }

    #[sqlx::test]
    async fn test_delete_author() {
        let db: Database = conn_db().await;

        let author: Author = create_author();

        db.create_author(author.clone()).await.unwrap();

        let sql_result_before: Option<Uuid> = db.get_author_id(author.id.clone()).await.unwrap();

        let sql_result_uuid: Uuid = db.delete_author(author.id).await.unwrap();

        let sql_result_after: Option<Uuid> = db.get_author_id(sql_result_uuid).await.unwrap();

        assert!(sql_result_before.is_some());
        assert!(sql_result_after.is_none());
    }

    #[sqlx::test]
    async fn test_count_authors() {
        let db: Database = conn_db().await;

        let sql_result: i64 = db.count_authors().await.unwrap();

        assert!(sql_result >= 0);
    }
}
