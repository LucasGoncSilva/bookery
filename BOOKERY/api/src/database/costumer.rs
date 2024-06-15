use sqlx::{postgres::PgRow, Row};
use time::Date;
use uuid::Uuid;

use crate::database::{conn::Database, ResultDB};
use crate::structs::{costumer::Costumer, PersonDocument, PersonName};

impl Database {
    pub async fn create_costumer(&self, costumer: Costumer) -> ResultDB<Uuid> {
        let costumer_uuid: Uuid = sqlx::query(
            "
            INSERT INTO tbl_costumers (id, name, document, born)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ",
        )
        .bind(costumer.id)
        .bind(costumer.name.as_str())
        .bind(costumer.document.as_str())
        .bind(costumer.born)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(costumer_uuid)
    }

    pub async fn get_costumer(&self, costumer_uuid: Uuid) -> ResultDB<Option<Costumer>> {
        let costumer: Option<Costumer> = sqlx::query(
            "
            SELECT id, name, document, born
            FROM tbl_costumers
            WHERE id = $1
        ",
        )
        .bind(costumer_uuid)
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");
            let document_parser: String = row.get("document");

            let id: Uuid = row.get("id");
            let name: PersonName = PersonName::try_from(name_parser).unwrap();
            let document: PersonDocument = PersonDocument::try_from(document_parser).unwrap();
            let born: Date = row.get("born");

            Costumer {
                id,
                name,
                document,
                born,
            }
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(costumer)
    }

    pub async fn get_costumer_id(&self, costumer_uuid: Uuid) -> ResultDB<Option<Uuid>> {
        let costumer_uuid: Option<Uuid> = sqlx::query(
            "
            SELECT id
            FROM tbl_costumers
            WHERE id = $1
        ",
        )
        .bind(costumer_uuid)
        .map(|row: PgRow| {
            let id: Uuid = row.get("id");
            id
        })
        .fetch_optional(&self.pool)
        .await?;

        Ok(costumer_uuid)
    }

    pub async fn search_costumers(&self, terms: String) -> ResultDB<Vec<Costumer>> {
        let costumers_vec: Vec<Costumer> = sqlx::query(
            "
            SELECT id, name, document, born
            FROM tbl_costumers
            WHERE name ILIKE $1
        ",
        )
        .bind(format!("%{terms}%"))
        .map(|row: PgRow| {
            let name_parser: String = row.get("name");
            let document_parser: String = row.get("document");

            let id: Uuid = row.get("id");
            let name: PersonName = PersonName::try_from(name_parser).unwrap();
            let document: PersonDocument = PersonDocument::try_from(document_parser).unwrap();
            let born: Date = row.get("born");

            Costumer {
                id,
                name,
                document,
                born,
            }
        })
        .fetch_all(&self.pool)
        .await?;

        Ok(costumers_vec)
    }

    pub async fn update_costumer(&self, costumer: Costumer) -> ResultDB<Uuid> {
        let costumer_uuid: Uuid = sqlx::query(
            "
            UPDATE tbl_costumers
            SET name = $1, document = $2, born = $3
            WHERE id = $4
            RETURNING id
        ",
        )
        .bind(costumer.name.as_str())
        .bind(costumer.document.as_str())
        .bind(costumer.born)
        .bind(costumer.id)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(costumer_uuid)
    }

    pub async fn delete_costumer(&self, costumer_uuid: Uuid) -> ResultDB<Uuid> {
        let costumer_uuid: Uuid = sqlx::query(
            "
            DELETE FROM tbl_costumers
            WHERE id = $1
            RETURNING id
        ",
        )
        .bind(costumer_uuid)
        .map(|row: PgRow| {
            let uuid: Uuid = row.get("id");
            uuid
        })
        .fetch_one(&self.pool)
        .await?;

        Ok(costumer_uuid)
    }

    pub async fn count_costumers(&self) -> ResultDB<i64> {
        let total: i64 = sqlx::query_scalar(
            "
            SELECT count(*) as total
            FROM tbl_costumers
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
        structs::costumer::{PayloadCostumer, PayloadUpdateCostumer},
    };

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_DOCUMENT: &'static str = "12345678901";
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    async fn conn_db() -> Database {
        let db_url: String = var("DATABASE_URL").unwrap();
        Database::conn(&db_url).await
    }

    fn create_costumer() -> Costumer {
        let payload_costumer: PayloadCostumer = PayloadCostumer {
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        Costumer::create(payload_costumer).unwrap()
    }

    #[sqlx::test]
    async fn test_create_costumer() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let costumer_uuid: Uuid = costumer.id.clone();

        let sql_result: Uuid = db.create_costumer(costumer).await.unwrap();

        assert_eq!(sql_result, costumer_uuid);
    }

    #[sqlx::test]
    async fn test_get_costumer_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let costumer_uuid: Uuid = db.create_costumer(costumer.clone()).await.unwrap();

        let sql_result: Costumer = db
            .get_costumer(costumer_uuid.clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(sql_result, costumer);
    }

    #[sqlx::test]
    async fn test_get_costumer_not_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let sql_result: Option<Costumer> = db.get_costumer(costumer.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_get_costumer_id_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let costumer_uuid: Uuid = db.create_costumer(costumer.clone()).await.unwrap();

        let sql_result: Uuid = db
            .get_costumer_id(costumer_uuid.clone())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(sql_result, costumer_uuid);
    }

    #[sqlx::test]
    async fn test_get_costumer_id_not_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let sql_result: Option<Uuid> = db.get_costumer_id(costumer.id.clone()).await.unwrap();

        assert!(sql_result.is_none());
    }

    #[sqlx::test]
    async fn test_search_costumers_case_sensitive_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let term: QueryURL = QueryURL {
            term: "Nam".to_string(),
        };

        db.create_costumer(costumer.clone()).await.unwrap();

        let sql_result: Vec<Costumer> = db.search_costumers(term.term).await.unwrap();

        assert!(sql_result.contains(&costumer));
    }

    #[sqlx::test]
    async fn test_search_costumers_case_insensitive_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let term: QueryURL = QueryURL {
            term: "nam".to_string(),
        };

        db.create_costumer(costumer.clone()).await.unwrap();

        let sql_result: Vec<Costumer> = db.search_costumers(term.term).await.unwrap();

        assert!(sql_result.contains(&costumer));
    }

    #[sqlx::test]
    async fn test_search_costumers_not_found() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let term: QueryURL = QueryURL {
            term: "foo".to_string(),
        };

        db.create_costumer(costumer.clone()).await.unwrap();

        let sql_result: Vec<Costumer> = db.search_costumers(term.term).await.unwrap();

        assert!(!sql_result.contains(&costumer));
    }

    #[sqlx::test]
    async fn test_update_costumer() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        let sql_costumer_uuid: Uuid = db.create_costumer(costumer.clone()).await.unwrap();

        let payload_update_costumer: PayloadUpdateCostumer = PayloadUpdateCostumer {
            id: sql_costumer_uuid.clone(),
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        db.update_costumer(Costumer::parse(payload_update_costumer).unwrap())
            .await
            .unwrap();

        let sql_result: Costumer = db.get_costumer(sql_costumer_uuid).await.unwrap().unwrap();

        assert_eq!(sql_result, costumer);
    }

    #[sqlx::test]
    async fn test_delete_costumer() {
        let db: Database = conn_db().await;

        let costumer: Costumer = create_costumer();

        db.create_costumer(costumer.clone()).await.unwrap();

        let sql_result_before: Option<Uuid> =
            db.get_costumer_id(costumer.id.clone()).await.unwrap();

        let sql_result_uuid: Uuid = db.delete_costumer(costumer.id).await.unwrap();

        let sql_result_after: Option<Uuid> = db.get_costumer_id(sql_result_uuid).await.unwrap();

        assert!(sql_result_before.is_some());
        assert!(sql_result_after.is_none());
    }

    #[sqlx::test]
    async fn test_count_costumers() {
        let db: Database = conn_db().await;

        let sql_result: i64 = db.count_costumers().await.unwrap();

        assert!(sql_result >= 0);
    }
}
