use sqlx::{postgres::PgRow, Row};
use time::Date;
use uuid::Uuid;

use crate::database::{conn::Database, ResultDB};
use crate::structs::{book::Book, BookName, EditorName};

impl Database {
    pub async fn create_book(&self, book: Book) -> ResultDB<Uuid> {
        let book_uuid: Uuid = sqlx::query(
            "
        INSERT INTO tbl_books (id, name, author_uuid, editor, release)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        ",
        )
        .bind(book.id)
        .bind(book.name.as_str())
        .bind(book.author_uuid)
        .bind(book.editor.as_str())
        .bind(book.release)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(book_uuid)
    }

    pub async fn get_book(&self, book_uuid: Uuid) -> ResultDB<Option<Book>> {
        let book: Option<Book> = sqlx::query(
            "
        SELECT id, name, author_uuid, editor, release
        FROM tbl_books
        WHERE id = $1
        ",
        )
        .bind(book_uuid)
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");
            let editor_parser: String = row.get("editor");

            let id: Uuid = row.get("id");
            let name: BookName = BookName::try_from(name_parser).unwrap();
            let author_uuid: Uuid = row.get("author_uuid");
            let editor: EditorName = EditorName::try_from(editor_parser).unwrap();
            let release: Date = row.get("release");

            Book {
                id,
                name,
                author_uuid,
                editor,
                release,
            }
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(book)
    }

    pub async fn get_book_id(&self, book_uuid: Uuid) -> ResultDB<Option<Uuid>> {
        let book_uuid: Option<Uuid> = sqlx::query(
            "
        SELECT id
        FROM tbl_books
        WHERE id = $1
        ",
        )
        .bind(book_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            id
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(book_uuid)
    }

    pub async fn search_books(&self, token: String) -> ResultDB<Vec<Book>> {
        let book_vec: Vec<Book> = sqlx::query(
            "
        SELECT id, name, author_uuid, editor, release
        FROM tbl_books
        WHERE name ILIKE $1
        OR editor ILIKE $1
        ",
        )
        .bind(format!("%{token}%"))
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");
            let editor_parser: String = row.get("editor");

            let id: Uuid = row.get("id");
            let name: BookName = BookName::try_from(name_parser).unwrap();
            let author_uuid: Uuid = row.get("author_uuid");
            let editor: EditorName = EditorName::try_from(editor_parser).unwrap();
            let release: Date = row.get("release");

            Book {
                id,
                name,
                author_uuid,
                editor,
                release,
            }
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(book_vec)
    }

    pub async fn update_book(&self, book: Book) -> ResultDB<Uuid> {
        let book_uuid: Uuid = sqlx::query(
            "
        UPDATE tbl_books
        SET name = $1, author_uuid = $2, editor = $3, release = $4
        WHERE id = $5
        RETURNING id
        ",
        )
        .bind(book.name.as_str())
        .bind(book.author_uuid)
        .bind(book.editor.as_str())
        .bind(book.release)
        .bind(book.id)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(book_uuid)
    }

    pub async fn delete_book(&self, book_uuid: Uuid) -> ResultDB<Uuid> {
        let book_uuid: Uuid = sqlx::query(
            "
        DELETE FROM tbl_books
        WHERE id = $1
        RETURNING id
        ",
        )
        .bind(book_uuid)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(book_uuid)
    }

    pub async fn count_books(&self) -> ResultDB<i64> {
        let total: i64 = sqlx::query_scalar(
            "
        SELECT count(*) as total
        FROM tbl_books
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
        structs::{
            author::{Author, PayloadAuthor},
            book::{PayloadBook, PayloadUpdateBook},
        },
    };

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_EDITOR: &'static str = "Editor";
    const DEFAULT_RELEASE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    async fn conn_db() -> Database {
        let db_url: String = var("DATABASE_URL").unwrap();
        Database::conn(&db_url).await
    }

    async fn create_book() -> Book {
        let db: Database = conn_db().await;

        let payload_author: PayloadAuthor = PayloadAuthor {
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let author: Author = Author::create(payload_author).unwrap();

        let author_uuid: Uuid = db.create_author(author).await.unwrap();

        let payload_book: PayloadBook = PayloadBook {
            name: DEFAULT_NAME.to_string(),
            editor: DEFAULT_EDITOR.to_string(),
            author_uuid,
            release: DEFAULT_RELEASE.unwrap(),
        };

        Book::create(payload_book).unwrap()
    }

    #[sqlx::test]
    async fn test_create_book() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let book_uuid: Uuid = book.id.clone();

        let sql_result: Uuid = db.create_book(book).await.unwrap();

        assert_eq!(sql_result, book_uuid);
    }

    #[sqlx::test]
    async fn test_get_book_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let book_uuid: Uuid = db.create_book(book.clone()).await.unwrap();

        let sql_result: Book = db.get_book(book_uuid.clone()).await.unwrap().unwrap();

        assert_eq!(sql_result, book);
    }

    #[sqlx::test]
    async fn test_get_book_not_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let sql_result: Option<Book> = db.get_book(book.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_get_book_id_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let book_uuid: Uuid = db.create_book(book.clone()).await.unwrap();

        let sql_result: Uuid = db.get_book_id(book_uuid.clone()).await.unwrap().unwrap();

        assert_eq!(sql_result, book_uuid);
    }

    #[sqlx::test]
    async fn test_get_book_id_not_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let sql_result: Option<Uuid> = db.get_book_id(book.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_search_books_case_sensitive_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let token: QueryURL = QueryURL {
            token: "Nam".to_string(),
        };

        db.create_book(book.clone()).await.unwrap();

        let sql_result: Vec<Book> = db.search_books(token.token).await.unwrap();

        assert!(sql_result.contains(&book));
    }

    #[sqlx::test]
    async fn test_search_books_case_insensitive_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let token: QueryURL = QueryURL {
            token: "nam".to_string(),
        };

        db.create_book(book.clone()).await.unwrap();

        let sql_result: Vec<Book> = db.search_books(token.token).await.unwrap();

        assert!(sql_result.contains(&book));
    }

    #[sqlx::test]
    async fn test_search_books_not_found() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let token: QueryURL = QueryURL {
            token: "foo".to_string(),
        };

        db.create_book(book.clone()).await.unwrap();

        let sql_result: Vec<Book> = db.search_books(token.token).await.unwrap();

        assert!(!sql_result.contains(&book));
    }

    #[sqlx::test]
    async fn test_update_book() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        let sql_book_uuid: Uuid = db.create_book(book.clone()).await.unwrap();

        let payload_update_book: PayloadUpdateBook = PayloadUpdateBook {
            id: sql_book_uuid.clone(),
            name: DEFAULT_NAME.to_string(),
            author_uuid: book.author_uuid.clone(),
            editor: DEFAULT_EDITOR.to_string(),
            release: DEFAULT_RELEASE.unwrap(),
        };

        db.update_book(Book::parse(payload_update_book).unwrap())
            .await
            .unwrap();

        let sql_result: Book = db.get_book(sql_book_uuid).await.unwrap().unwrap();

        assert_eq!(sql_result, book);
    }

    #[sqlx::test]
    async fn test_delete_book() {
        let db: Database = conn_db().await;

        let book: Book = create_book().await;

        db.create_book(book.clone()).await.unwrap();

        let sql_result_before: Option<Uuid> = db.get_book_id(book.id.clone()).await.unwrap();

        let sql_result_uuid: Uuid = db.delete_book(book.id).await.unwrap();

        let sql_result_after: Option<Uuid> = db.get_book_id(sql_result_uuid).await.unwrap();

        assert!(sql_result_before.is_some());
        assert!(sql_result_after.is_none());
    }

    #[sqlx::test]
    async fn test_count_books() {
        let db: Database = conn_db().await;

        let sql_result: i64 = db.count_books().await.unwrap();

        assert!(sql_result >= 0);
    }
}
