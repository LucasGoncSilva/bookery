use sqlx::{postgres::PgRow, Row};
use time::Date;
use uuid::Uuid;

use crate::database::{conn::Database, ResultDB};
use crate::structs::rental::Rent;

impl Database {
    pub async fn create_rental(&self, rent: Rent) -> ResultDB<Uuid> {
        let rent_uuid: Uuid = sqlx::query(
            "
            INSERT INTO tbl_rentals (id, costumer_uuid, book_uuid, borrowed_at, due_date)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        ",
        )
        .bind(rent.id)
        .bind(rent.costumer_uuid)
        .bind(rent.book_uuid)
        .bind(rent.borrowed_at)
        .bind(rent.due_date)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(rent_uuid)
    }

    pub async fn get_rental(&self, rent_uuid: Uuid) -> ResultDB<Option<Rent>> {
        let rent: Option<Rent> = sqlx::query(
            "
            SELECT id, costumer_uuid, book_uuid, borrowed_at, due_date, returned_at
            FROM tbl_rentals
            WHERE id = $1
        ",
        )
        .bind(rent_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            let costumer_uuid: Uuid = row.get("costumer_uuid");
            let book_uuid: Uuid = row.get("book_uuid");
            let borrowed_at: Date = row.get("borrowed_at");
            let due_date: Date = row.get("due_date");
            let returned_at: Option<Date> = row.get("returned_at");

            Rent {
                id,
                costumer_uuid,
                book_uuid,
                borrowed_at,
                due_date,
                returned_at,
            }
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(rent)
    }

    pub async fn get_rental_id(&self, rent_uuid: Uuid) -> ResultDB<Option<Uuid>> {
        let rent_uuid: Option<Uuid> = sqlx::query(
            "
            SELECT id
            FROM tbl_rentals
            WHERE id = $1
        ",
        )
        .bind(rent_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            id
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(rent_uuid)
    }

    pub async fn search_rentals(&self, token: String) -> ResultDB<Vec<Rent>> {
        let costumers_vec: Vec<Rent> = sqlx::query(
            "
            SELECT id, costumer_uuid, book_uuid, borrowed_at, due_date, returned_at
            FROM tbl_rentals
            WHERE costumer_uuid::text ILIKE $1 OR book_uuid::text ILIKE $1
        ",
        )
        .bind(format!("%{token}%"))
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            let costumer_uuid: Uuid = row.get("costumer_uuid");
            let book_uuid: Uuid = row.get("book_uuid");
            let borrowed_at: Date = row.get("borrowed_at");
            let due_date: Date = row.get("due_date");
            let returned_at: Option<Date> = row.get("returned_at");

            Rent {
                id,
                costumer_uuid,
                book_uuid,
                borrowed_at,
                due_date,
                returned_at,
            }
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(costumers_vec)
    }

    pub async fn update_rental(&self, rent: Rent) -> ResultDB<Uuid> {
        let rent_uuid: Uuid = sqlx::query(
            "
            UPDATE tbl_rentals
            SET costumer_uuid = $1, book_uuid = $2, borrowed_at = $3, due_date = $4, returned_at = $5
            WHERE id = $6
            RETURNING id
        ",
        )
        .bind(rent.costumer_uuid)
        .bind(rent.book_uuid)
        .bind(rent.borrowed_at)
        .bind(rent.due_date)
        .bind(rent.returned_at)
        .bind(rent.id)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(rent_uuid)
    }

    pub async fn delete_rental(&self, rent_uuid: Uuid) -> ResultDB<Uuid> {
        let rent_uuid: Uuid = sqlx::query(
            "
            DELETE FROM tbl_rentals
            WHERE id = $1
            RETURNING id
        ",
        )
        .bind(rent_uuid)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(rent_uuid)
    }

    pub async fn count_rentals(&self) -> ResultDB<i64> {
        let total: i64 = sqlx::query_scalar(
            "
            SELECT count(*) as total
            FROM tbl_rentals
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
            book::{Book, PayloadBook},
            costumer::{Costumer, PayloadCostumer},
            rental::{PayloadRent, PayloadUpdateRent},
        },
    };

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_EDITOR: &'static str = "Editor";
    const DEFAULT_DOCUMENT: &'static str = "12345678901";
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    const DEFAULT_RELEASE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    const DEFAULT_BORROWED_DATE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);
    const DEFAULT_DUE_DATE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);
    const DEFAULT_RETURNED_DATE: Result<Date, ComponentRange> =
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

        let book: Book = Book::create(payload_book).unwrap();

        db.create_book(book.clone()).await.unwrap();

        book
    }

    async fn create_costumer() -> Costumer {
        let db: Database = conn_db().await;

        let payload_costumer: PayloadCostumer = PayloadCostumer {
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let costumer: Costumer = Costumer::create(payload_costumer).unwrap();

        db.create_costumer(costumer.clone()).await.unwrap();

        costumer
    }

    async fn create_rental() -> Rent {
        let book_uuid: Uuid = create_book().await.id;
        let costumer_uuid: Uuid = create_costumer().await.id;

        let payload_rent: PayloadRent = PayloadRent {
            book_uuid,
            costumer_uuid,
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
        };

        Rent::create(payload_rent).unwrap()
    }

    #[sqlx::test]
    async fn test_create_rental() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let rent_uuid: Uuid = rent.id.clone();

        let sql_result: Uuid = db.create_rental(rent).await.unwrap();

        assert_eq!(sql_result, rent_uuid);
    }

    #[sqlx::test]
    async fn test_get_rental_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let rent_uuid: Uuid = db.create_rental(rent.clone()).await.unwrap();

        let sql_result: Rent = db.get_rental(rent_uuid.clone()).await.unwrap().unwrap();

        assert_eq!(sql_result, rent);
    }

    #[sqlx::test]
    async fn test_get_rental_not_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let sql_result: Option<Rent> = db.get_rental(rent.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_get_rental_id_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let rent_uuid: Uuid = db.create_rental(rent.clone()).await.unwrap();

        let sql_result: Uuid = db.get_rental_id(rent_uuid.clone()).await.unwrap().unwrap();

        assert_eq!(sql_result, rent_uuid);
    }

    #[sqlx::test]
    async fn test_get_rental_id_not_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let sql_result: Option<Uuid> = db.get_rental_id(rent.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_search_rental_case_sensitive_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let token: QueryURL = QueryURL {
            token: String::from(rent.costumer_uuid),
        };

        db.create_rental(rent.clone()).await.unwrap();

        let sql_result: Vec<Rent> = db.search_rentals(token.token).await.unwrap();

        assert!(sql_result.contains(&rent));
    }

    #[sqlx::test]
    async fn test_search_rental_case_insensitive_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let token: QueryURL = QueryURL {
            token: String::from(rent.book_uuid).to_ascii_uppercase(),
        };

        db.create_rental(rent.clone()).await.unwrap();

        let sql_result: Vec<Rent> = db.search_rentals(token.token).await.unwrap();

        assert!(sql_result.contains(&rent));
    }

    #[sqlx::test]
    async fn test_search_rental_not_found() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let token: QueryURL = QueryURL {
            token: "foo".to_string(),
        };

        db.create_rental(rent.clone()).await.unwrap();

        let sql_result: Vec<Rent> = db.search_rentals(token.token).await.unwrap();

        assert!(!sql_result.contains(&rent));
    }

    #[sqlx::test]
    async fn test_update_rental() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        let sql_rental_uuid: Uuid = db.create_rental(rent).await.unwrap();

        let payload_update_rent: PayloadUpdateRent = PayloadUpdateRent {
            id: sql_rental_uuid.clone(),
            book_uuid: create_book().await.id,
            costumer_uuid: create_costumer().await.id,
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let updated_rent: Rent = Rent::parse(payload_update_rent).unwrap();

        db.update_rental(updated_rent.clone()).await.unwrap();

        let sql_result: Rent = db.get_rental(sql_rental_uuid).await.unwrap().unwrap();

        assert_eq!(sql_result, updated_rent);
    }

    #[sqlx::test]
    async fn test_delete_rental() {
        let db: Database = conn_db().await;

        let rent: Rent = create_rental().await;

        db.create_rental(rent.clone()).await.unwrap();

        let sql_result_before: Option<Uuid> = db.get_rental_id(rent.id.clone()).await.unwrap();

        let sql_result_uuid: Uuid = db.delete_rental(rent.id).await.unwrap();

        let sql_result_after: Option<Uuid> = db.get_rental_id(sql_result_uuid).await.unwrap();

        assert!(sql_result_before.is_some());
        assert!(sql_result_after.is_none());
    }

    #[sqlx::test]
    async fn test_count_rentals() {
        let db: Database = conn_db().await;

        let sql_result: i64 = db.count_rentals().await.unwrap();

        assert!(sql_result >= 0);
    }
}
