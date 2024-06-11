use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    Error as SqlxErr, PgPool, Pool, Postgres, Row,
};
use time::Date;
use uuid::Uuid;

use crate::structs::{author::Author, book::Book, BookName, EditorName, PersonName};

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
