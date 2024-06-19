use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::conn::Database;
use crate::structs::rent::{PayloadRent, PayloadUpdateRent, Rent};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_rent(
    State(db): State<DB>,
    Json(incoming_rent): Json<PayloadRent>,
) -> ResultStatus<Uuid> {
    match Rent::create(incoming_rent) {
        Ok(rent) => match db.create_rent(rent).await {
            Ok(rent_uuid) => Ok((StatusCode::CREATED, Json(rent_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_rent(State(db): State<DB>, Path(rent_uuid): Path<Uuid>) -> ResultStatus<Rent> {
    match db.get_rent(rent_uuid).await {
        Ok(Some(rent)) => Ok((StatusCode::OK, Json(rent))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_rental(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Rent>> {
    match db.search_rental(t.token).await {
        Ok(rental_vec) => Ok((StatusCode::OK, Json(rental_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_rent(
    State(db): State<DB>,
    Json(payload_update_rent): Json<PayloadUpdateRent>,
) -> ResultStatus<Uuid> {
    match db.get_rent_id(payload_update_rent.id).await {
        Ok(Some(_rent_uuid)) => match Rent::parse(payload_update_rent) {
            Ok(updated_rent) => match db.update_rent(updated_rent).await {
                Ok(rent_uuid) => Ok((StatusCode::ACCEPTED, Json(rent_uuid))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_rent(
    State(db): State<DB>,
    Json(incoming_struct): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_rent(incoming_struct.id).await {
        Ok(Some(rent)) => match db.delete_rent(rent.id).await {
            Ok(rent_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Rent {rent_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn count_rental(State(db): State<DB>) -> ResultStatus<i64> {
    match db.count_rental().await {
        Ok(num) => Ok((StatusCode::OK, Json(num))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env::var;

    use axum::Router;
    use axum_test::{TestResponse, TestServer};
    use serde_json::json;
    use time::{error::ComponentRange, Date, Month};

    use crate::router::router;
    use crate::structs::{author::PayloadAuthor, book::PayloadBook, costumer::PayloadCostumer};

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

    fn create_payload_author() -> PayloadAuthor {
        PayloadAuthor {
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        }
    }

    fn create_payload_costumer() -> PayloadCostumer {
        PayloadCostumer {
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        }
    }

    async fn server() -> TestServer {
        let db_url: String = var("DATABASE_URL").unwrap();
        let db: Database = Database::conn(&db_url).await;

        let app: Router = router(Arc::new(db));

        TestServer::new(app).unwrap()
    }

    async fn create_author_on_server() -> TestResponse {
        server()
            .await
            .post("/author/create")
            .json(&json!(create_payload_author()))
            .await
    }

    async fn create_payload_book() -> PayloadBook {
        let author_uuid: Uuid = create_author_on_server().await.json();

        PayloadBook {
            name: DEFAULT_NAME.to_string(),
            author_uuid,
            editor: DEFAULT_EDITOR.to_string(),
            release: DEFAULT_RELEASE.unwrap(),
        }
    }

    async fn create_book_on_server() -> TestResponse {
        server()
            .await
            .post("/book/create")
            .json(&json!(create_payload_book().await))
            .await
    }

    async fn create_costumer_on_server() -> TestResponse {
        server()
            .await
            .post("/costumer/create")
            .json(&json!(create_payload_costumer()))
            .await
    }

    async fn create_payload_rent() -> PayloadRent {
        let book_uuid: Uuid = create_book_on_server().await.json();
        let costumer_uuid: Uuid = create_costumer_on_server().await.json();

        PayloadRent {
            book_uuid,
            costumer_uuid,
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
        }
    }

    async fn create_rent_on_server() -> TestResponse {
        server()
            .await
            .post("/rent/create")
            .json(&json!(create_payload_rent().await))
            .await
    }

    #[tokio::test]
    async fn test_create_rent_get() {
        let res: TestResponse = server().await.get("/rent/create").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_create_rent_post_no_data() {
        let res: TestResponse = server().await.post("/rent/create").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_create_rent_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/rent/create")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_rent_post_valid() {
        let res: TestResponse = create_rent_on_server().await;

        res.assert_status(StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_rent_get_empty() {
        let res: TestResponse = server().await.get("/rent/get/").await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_rent_get_invalid() {
        let res: TestResponse = server().await.get("/rent/get/12345").await;

        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_rent_get_not_found() {
        let example_uuid: Uuid = Uuid::new_v4();
        let res: TestResponse = server()
            .await
            .get(&format!("/rent/get/{example_uuid}"))
            .await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_rent_get_found() {
        let rent_created: TestResponse = create_rent_on_server().await;

        let rent_uuid: String = rent_created.json();

        let res: TestResponse = server().await.get(&format!("/rent/get/{rent_uuid}")).await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_rent_post() {
        let res: TestResponse = server()
            .await
            .post(&format!("/rent/get/{}", Uuid::new_v4()))
            .await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_search_rental_get_no_param() {
        create_rent_on_server().await;

        let res: TestResponse = server().await.get("/rent/search").await;
        res.assert_status_bad_request();

        let res: TestResponse = server().await.get("/rent/search?").await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_search_rental_get_found() {
        let create_res: TestResponse = create_rent_on_server().await;

        let created_rent_uuid: Uuid = create_res.json();

        let created_rent: Rent = server()
            .await
            .get(&format!("/rent/get/{created_rent_uuid}"))
            .await
            .json();

        let res: TestResponse = server().await.get("/rent/search?token").await;
        res.assert_status_ok();
        let res_json: Vec<Rent> = res.json();
        assert!(res_json.contains(&created_rent));

        let res: TestResponse = server().await.get("/rent/search?token=").await;
        res.assert_status_ok();
        let res_json: Vec<Rent> = res.json();
        assert!(res_json.contains(&created_rent));

        let res: TestResponse = server()
            .await
            .get(&format!(
                "/rent/search?token={}",
                String::from(created_rent.book_uuid).chars().nth(2).unwrap()
            ))
            .await;
        res.assert_status_ok();
        let res_json: Vec<Rent> = res.json();
        assert!(res_json.contains(&created_rent));
    }

    #[tokio::test]
    async fn test_search_rental_post() {
        let res: TestResponse = server().await.post("/rent/search?token=am").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_update_rent_get() {
        let res: TestResponse = server().await.post("/rent/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_rent_post_no_data() {
        let res: TestResponse = server().await.post("/rent/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_rent_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/rent/update")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_update_rent_post_valid() {
        let create_res: TestResponse = create_rent_on_server().await;

        let created_rent_uuid: Uuid = create_res.json();

        let payload_update_rent: PayloadUpdateRent = PayloadUpdateRent {
            id: created_rent_uuid,
            book_uuid: create_book_on_server().await.json(),
            costumer_uuid: create_costumer_on_server().await.json(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let res: TestResponse = server()
            .await
            .post("/rent/update")
            .json(&json!(payload_update_rent))
            .await;

        res.assert_status(StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_delete_rent_get() {
        let res: TestResponse = server().await.post("/rent/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_rent_post_no_data() {
        let res: TestResponse = server().await.post("/rent/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_rent_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/rent/delete")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_rent_post_valid() {
        let create_res: TestResponse = create_rent_on_server().await;

        let created_rent_uuid: Uuid = create_res.json();

        let payload_delete_rent: PayloadUpdateRent = PayloadUpdateRent {
            id: created_rent_uuid.clone(),
            book_uuid: create_book_on_server().await.json(),
            costumer_uuid: create_costumer_on_server().await.json(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let res: TestResponse = server()
            .await
            .post("/rent/delete")
            .json(&json!(payload_delete_rent))
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_count_rent_get() {
        let res: TestResponse = server().await.get("/rent/count").await;

        res.assert_status_ok();
        assert!(res.json::<i64>() >= 0);
    }

    #[tokio::test]
    async fn test_count_rent_post() {
        let res: TestResponse = server().await.post("/rent/count").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }
}
