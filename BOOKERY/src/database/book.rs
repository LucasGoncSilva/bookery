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

    pub async fn search_books(&self, terms: String) -> ResultDB<Vec<Book>> {
        let book_vec: Vec<Book> = sqlx::query(
            "
        SELECT id, name, author_uuid, editor, release
        FROM tbl_books
        WHERE name ILIKE $1
        ",
        )
        .bind(format!("%{terms}%"))
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
