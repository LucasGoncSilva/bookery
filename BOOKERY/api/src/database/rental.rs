use sqlx::{postgres::PgRow, Row};
use time::Date;
use uuid::Uuid;

use crate::database::{conn::Database, ResultDB};
use crate::structs::rental::{Rental, RentalWithCostumerAndBook};
use crate::structs::{BookName, PersonName};

impl Database {
    pub async fn create_rental(&self, rental: Rental) -> ResultDB<Uuid> {
        let rental_uuid: Uuid = sqlx::query(
            "
            INSERT INTO tbl_rentals (id, costumer_uuid, book_uuid, borrowed_at, due_date)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        ",
        )
        .bind(rental.id)
        .bind(rental.costumer_uuid)
        .bind(rental.book_uuid)
        .bind(rental.borrowed_at)
        .bind(rental.due_date)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(rental_uuid)
    }

    pub async fn get_rental(
        &self,
        rental_uuid: Uuid,
    ) -> ResultDB<Option<RentalWithCostumerAndBook>> {
        let rental: Option<RentalWithCostumerAndBook> = sqlx::query(
            "
            SELECT r.id as id, c.name as costumer_name, b.name as book_name, r.borrowed_at as borrowed_at, r.due_date as due_date, r.returned_at as returned_at
            FROM tbl_rentals r
            JOIN tbl_costumers c
            ON r.costumer_uuid = c.id
            JOIN tbl_books b
            ON r.book_uuid = b.id
            WHERE r.id = $1
        ",
        )
        .bind(rental_uuid)
        .map(|row: PgRow| {
            let rental_costumer_name_parser: String = row.get("costumer_name");
            let rental_book_name_parser: String = row.get("book_name");

            let id: Uuid = row.get("id");
            let costumer_name: PersonName = PersonName::try_from(rental_costumer_name_parser).unwrap();
            let book_name: BookName = BookName::try_from(rental_book_name_parser).unwrap();
            let borrowed_at: Date = row.get("borrowed_at");
            let due_date: Date = row.get("due_date");
            let returned_at: Option<Date> = row.get("returned_at");

            RentalWithCostumerAndBook {
                id,
                costumer_name,
                book_name,
                borrowed_at,
                due_date,
                returned_at,
            }
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(rental)
    }

    pub async fn get_rental_raw(&self, rental_uuid: Uuid) -> ResultDB<Option<Rental>> {
        let rental: Option<Rental> = sqlx::query(
            "
            SELECT id, costumer_uuid, book_uuid, borrowed_at, due_date, returned_at
            FROM tbl_rentals
            WHERE id = $1
        ",
        )
        .bind(rental_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            let costumer_uuid: Uuid = row.get("costumer_uuid");
            let book_uuid: Uuid = row.get("book_uuid");
            let borrowed_at: Date = row.get("borrowed_at");
            let due_date: Date = row.get("due_date");
            let returned_at: Option<Date> = row.get("returned_at");

            Rental {
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

        Ok(rental)
    }

    pub async fn get_rental_id(&self, rental_uuid: Uuid) -> ResultDB<Option<Uuid>> {
        let rental_uuid: Option<Uuid> = sqlx::query(
            "
            SELECT id
            FROM tbl_rentals
            WHERE id = $1
        ",
        )
        .bind(rental_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            id
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(rental_uuid)
    }

    pub async fn search_rentals(&self, token: String) -> ResultDB<Vec<RentalWithCostumerAndBook>> {
        let costumers_vec: Vec<RentalWithCostumerAndBook> = sqlx::query(
            "
            SELECT r.id as id, c.name as costumer_name, b.name as book_name, r.borrowed_at as borrowed_at, r.due_date as due_date, r.returned_at as returned_at
            FROM tbl_rentals r
            JOIN tbl_costumers c
            ON r.costumer_uuid = c.id
            JOIN tbl_books b
            ON r.book_uuid = b.id
            WHERE c.name ILIKE $1 OR b.name ILIKE $1
        ",
        )
        .bind(format!("%{token}%"))
        .map(|row: PgRow| {
            let rental_costumer_name_parser: String = row.get("costumer_name");
            let rental_book_name_parser: String = row.get("book_name");

            let id: Uuid = row.get("id");
            let costumer_name: PersonName =
                PersonName::try_from(rental_costumer_name_parser).unwrap();
            let book_name: BookName = BookName::try_from(rental_book_name_parser).unwrap();
            let borrowed_at: Date = row.get("borrowed_at");
            let due_date: Date = row.get("due_date");
            let returned_at: Option<Date> = row.get("returned_at");

            RentalWithCostumerAndBook {
                id,
                costumer_name,
                book_name,
                borrowed_at,
                due_date,
                returned_at,
            }
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(costumers_vec)
    }

    pub async fn search_rentals_raw(&self, token: String) -> ResultDB<Vec<Rental>> {
        let costumers_vec: Vec<Rental> = sqlx::query(
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

            Rental {
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

    pub async fn update_rental(&self, rental: Rental) -> ResultDB<Uuid> {
        let rental_uuid: Uuid = sqlx::query(
            "
            UPDATE tbl_rentals
            SET costumer_uuid = $1, book_uuid = $2, borrowed_at = $3, due_date = $4, returned_at = $5
            WHERE id = $6
            RETURNING id
        ",
        )
        .bind(rental.costumer_uuid)
        .bind(rental.book_uuid)
        .bind(rental.borrowed_at)
        .bind(rental.due_date)
        .bind(rental.returned_at)
        .bind(rental.id)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(rental_uuid)
    }

    pub async fn delete_rental(&self, rental_uuid: Uuid) -> ResultDB<Uuid> {
        let rental_uuid: Uuid = sqlx::query(
            "
            DELETE FROM tbl_rentals
            WHERE id = $1
            RETURNING id
        ",
        )
        .bind(rental_uuid)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(rental_uuid)
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
            rental::{PayloadRental, PayloadUpdateRental},
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

    async fn create_rental() -> Rental {
        let book_uuid: Uuid = create_book().await.id;
        let costumer_uuid: Uuid = create_costumer().await.id;

        let payload_rental: PayloadRental = PayloadRental {
            book_uuid,
            costumer_uuid,
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
        };

        Rental::create(payload_rental).unwrap()
    }

    #[sqlx::test]
    async fn test_create_rental() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let rental_uuid: Uuid = rental.id.clone();

        let sql_result: Uuid = db.create_rental(rental).await.unwrap();

        assert_eq!(sql_result, rental_uuid);
    }

    #[sqlx::test]
    async fn test_get_rental_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let rental_uuid: Uuid = db.create_rental(rental).await.unwrap();

        let sql_result: RentalWithCostumerAndBook =
            db.get_rental(rental_uuid.clone()).await.unwrap().unwrap();

        assert_eq!(
            sql_result,
            RentalWithCostumerAndBook {
                id: rental_uuid,
                costumer_name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                book_name: BookName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
                due_date: DEFAULT_DUE_DATE.unwrap(),
                returned_at: None,
            }
        );
    }

    #[sqlx::test]
    async fn test_get_rental_not_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let sql_result: Option<RentalWithCostumerAndBook> =
            db.get_rental(rental.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_get_rental_raw_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let rental_uuid: Uuid = db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Rental = db
            .get_rental_raw(rental_uuid.clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(sql_result, rental);
    }

    #[sqlx::test]
    async fn test_get_rental_raw_not_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let sql_result: Option<Rental> = db.get_rental_raw(rental.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_get_rental_id_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let rental_uuid: Uuid = db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Uuid = db
            .get_rental_id(rental_uuid.clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(sql_result, rental_uuid);
    }

    #[sqlx::test]
    async fn test_get_rental_id_not_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let sql_result: Option<Uuid> = db.get_rental_id(rental.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_search_rentals_case_sensitive_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let token: QueryURL = QueryURL {
            token: "Nam".to_string(),
        };

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Vec<RentalWithCostumerAndBook> =
            db.search_rentals(token.token).await.unwrap();

        assert!(sql_result.contains(&RentalWithCostumerAndBook {
            id: rental.id,
            costumer_name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
            book_name: BookName::try_from(DEFAULT_NAME.to_string()).unwrap(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: None,
        }));
    }

    #[sqlx::test]
    async fn test_search_rentals_case_insensitive_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let token: QueryURL = QueryURL {
            token: "nAM".to_string(),
        };

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Vec<RentalWithCostumerAndBook> =
            db.search_rentals(token.token).await.unwrap();

        assert!(sql_result.contains(&RentalWithCostumerAndBook {
            id: rental.id.clone(),
            costumer_name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
            book_name: BookName::try_from(DEFAULT_NAME.to_string()).unwrap(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: None,
        }));
    }

    #[sqlx::test]
    async fn test_search_rentals_not_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let token: QueryURL = QueryURL {
            token: "foo".to_string(),
        };

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Vec<RentalWithCostumerAndBook> =
            db.search_rentals(token.token).await.unwrap();

        assert!(!sql_result.contains(&RentalWithCostumerAndBook {
            id: rental.id.clone(),
            costumer_name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
            book_name: BookName::try_from(DEFAULT_NAME.to_string()).unwrap(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: None,
        }));
    }

    #[sqlx::test]
    async fn test_search_rentals_raw_case_sensitive_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let token: QueryURL = QueryURL {
            token: String::from(rental.costumer_uuid),
        };

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Vec<Rental> = db.search_rentals_raw(token.token).await.unwrap();

        assert!(sql_result.contains(&rental));
    }

    #[sqlx::test]
    async fn test_search_rentals_raw_case_insensitive_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let token: QueryURL = QueryURL {
            token: String::from(rental.book_uuid).to_ascii_uppercase(),
        };

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Vec<Rental> = db.search_rentals_raw(token.token).await.unwrap();

        assert!(sql_result.contains(&rental));
    }

    #[sqlx::test]
    async fn test_search_rentals_raw_not_found() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let token: QueryURL = QueryURL {
            token: "foo".to_string(),
        };

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result: Vec<Rental> = db.search_rentals_raw(token.token).await.unwrap();

        assert!(!sql_result.contains(&rental));
    }

    #[sqlx::test]
    async fn test_update_rental() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        let sql_rental_uuid: Uuid = db.create_rental(rental).await.unwrap();

        let payload_update_rental: PayloadUpdateRental = PayloadUpdateRental {
            id: sql_rental_uuid.clone(),
            book_uuid: create_book().await.id,
            costumer_uuid: create_costumer().await.id,
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let updated_rental: Rental = Rental::parse(payload_update_rental).unwrap();

        db.update_rental(updated_rental.clone()).await.unwrap();

        let sql_result: Rental = db.get_rental_raw(sql_rental_uuid).await.unwrap().unwrap();

        assert_eq!(sql_result, updated_rental);
    }

    #[sqlx::test]
    async fn test_delete_rental() {
        let db: Database = conn_db().await;

        let rental: Rental = create_rental().await;

        db.create_rental(rental.clone()).await.unwrap();

        let sql_result_before: Option<Uuid> = db.get_rental_id(rental.id.clone()).await.unwrap();

        let sql_result_uuid: Uuid = db.delete_rental(rental.id).await.unwrap();

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
