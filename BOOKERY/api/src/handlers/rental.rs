use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::conn::Database;
use crate::structs::rental::{
    PayloadRental, PayloadUpdateRental, Rental, RentalWithCostumerAndBook,
};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_rental(
    State(db): State<DB>,
    Json(incoming_rent): Json<PayloadRental>,
) -> ResultStatus<Uuid> {
    match Rental::create(incoming_rent) {
        Ok(rental) => match db.create_rental(rental).await {
            Ok(rental_uuid) => Ok((StatusCode::CREATED, Json(rental_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_rental(
    State(db): State<DB>,
    Path(rental_uuid): Path<Uuid>,
) -> ResultStatus<RentalWithCostumerAndBook> {
    match db.get_rental(rental_uuid).await {
        Ok(Some(rental)) => Ok((StatusCode::OK, Json(rental))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_rental_raw(
    State(db): State<DB>,
    Path(rental_uuid): Path<Uuid>,
) -> ResultStatus<Rental> {
    match db.get_rental_raw(rental_uuid).await {
        Ok(Some(rental)) => Ok((StatusCode::OK, Json(rental))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_rentals_raw(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Rental>> {
    match db.search_rentals_raw(t.token).await {
        Ok(rental_vec) => Ok((StatusCode::OK, Json(rental_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_rental(
    State(db): State<DB>,
    Json(payload_update_rent): Json<PayloadUpdateRental>,
) -> ResultStatus<Uuid> {
    match db.get_rental_id(payload_update_rent.id).await {
        Ok(Some(_rental_uuid)) => match Rental::parse(payload_update_rent) {
            Ok(updated_rent) => match db.update_rental(updated_rent).await {
                Ok(rental_uuid) => Ok((StatusCode::ACCEPTED, Json(rental_uuid))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_rental(
    State(db): State<DB>,
    Json(incoming_struct): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_rental_raw(incoming_struct.id).await {
        Ok(Some(rental)) => match db.delete_rental(rental.id).await {
            Ok(rental_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Rent {rental_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn count_rentals(State(db): State<DB>) -> ResultStatus<i64> {
    match db.count_rentals().await {
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

    async fn create_payload_rental() -> PayloadRental {
        let book_uuid: Uuid = create_book_on_server().await.json();
        let costumer_uuid: Uuid = create_costumer_on_server().await.json();

        PayloadRental {
            book_uuid,
            costumer_uuid,
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
        }
    }

    async fn create_rental_on_server() -> TestResponse {
        server()
            .await
            .post("/rental/create")
            .json(&json!(create_payload_rental().await))
            .await
    }

    #[tokio::test]
    async fn test_create_rental_get() {
        let res: TestResponse = server().await.get("/rental/create").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_create_rental_post_no_data() {
        let res: TestResponse = server().await.post("/rental/create").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_create_rental_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/rental/create")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_rental_post_valid() {
        let res: TestResponse = create_rental_on_server().await;

        res.assert_status(StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_rental_get_empty() {
        let res: TestResponse = server().await.get("/rental/get/").await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_rental_get_invalid() {
        let res: TestResponse = server().await.get("/rental/get/12345").await;

        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_rental_get_not_found() {
        let example_uuid: Uuid = Uuid::new_v4();
        let res: TestResponse = server()
            .await
            .get(&format!("/rental/get/{example_uuid}"))
            .await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_rental_get_found() {
        let rent_created: TestResponse = create_rental_on_server().await;

        let rental_uuid: String = rent_created.json();

        let res: TestResponse = server()
            .await
            .get(&format!("/rental/get/{rental_uuid}"))
            .await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_rental_post() {
        let res: TestResponse = server()
            .await
            .post(&format!("/rental/get/{}", Uuid::new_v4()))
            .await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_get_rental_raw_get_empty() {
        let res: TestResponse = server().await.get("/rental/get-raw/").await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_rental_raw_get_invalid() {
        let res: TestResponse = server().await.get("/rental/get-raw/12345").await;

        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_rental_raw_get_not_found() {
        let example_uuid: Uuid = Uuid::new_v4();
        let res: TestResponse = server()
            .await
            .get(&format!("/rental/get-raw/{example_uuid}"))
            .await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_rental_raw_get_found() {
        let rent_created: TestResponse = create_rental_on_server().await;

        let rental_uuid: String = rent_created.json();

        let res: TestResponse = server()
            .await
            .get(&format!("/rental/get-raw/{rental_uuid}"))
            .await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_rental_raw_post() {
        let res: TestResponse = server()
            .await
            .post(&format!("/rental/get-raw/{}", Uuid::new_v4()))
            .await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_search_rental_raw_get_no_param() {
        create_rental_on_server().await;

        let res: TestResponse = server().await.get("/rental/search-raw").await;
        res.assert_status_bad_request();

        let res: TestResponse = server().await.get("/rental/search-raw?").await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_search_rental_raw_get_found() {
        let create_res: TestResponse = create_rental_on_server().await;

        let created_rental_uuid: Uuid = create_res.json();

        let created_rent: Rental = server()
            .await
            .get(&format!("/rental/get-raw/{created_rental_uuid}"))
            .await
            .json();

        let res: TestResponse = server().await.get("/rental/search-raw?token").await;
        res.assert_status_ok();
        let res_json: Vec<Rental> = res.json();
        assert!(res_json.contains(&created_rent));

        let res: TestResponse = server().await.get("/rental/search-raw?token=").await;
        res.assert_status_ok();
        let res_json: Vec<Rental> = res.json();
        assert!(res_json.contains(&created_rent));

        let res: TestResponse = server()
            .await
            .get(&format!(
                "/rental/search-raw?token={}",
                String::from(created_rent.book_uuid).chars().nth(2).unwrap()
            ))
            .await;
        res.assert_status_ok();
        let res_json: Vec<Rental> = res.json();
        assert!(res_json.contains(&created_rent));
    }

    #[tokio::test]
    async fn test_search_rental_raw_post() {
        let res: TestResponse = server().await.post("/rental/search-raw?token=am").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_update_rental_get() {
        let res: TestResponse = server().await.post("/rental/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_rental_post_no_data() {
        let res: TestResponse = server().await.post("/rental/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_rental_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/rental/update")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_update_rental_post_valid() {
        let create_res: TestResponse = create_rental_on_server().await;

        let created_rental_uuid: Uuid = create_res.json();

        let payload_update_rent: PayloadUpdateRental = PayloadUpdateRental {
            id: created_rental_uuid,
            book_uuid: create_book_on_server().await.json(),
            costumer_uuid: create_costumer_on_server().await.json(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let res: TestResponse = server()
            .await
            .post("/rental/update")
            .json(&json!(payload_update_rent))
            .await;

        res.assert_status(StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_delete_rental_get() {
        let res: TestResponse = server().await.post("/rental/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_rental_post_no_data() {
        let res: TestResponse = server().await.post("/rental/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_rental_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/rental/delete")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_rental_post_valid() {
        let create_res: TestResponse = create_rental_on_server().await;

        let created_rental_uuid: Uuid = create_res.json();

        let payload_delete_rent: PayloadUpdateRental = PayloadUpdateRental {
            id: created_rental_uuid.clone(),
            book_uuid: create_book_on_server().await.json(),
            costumer_uuid: create_costumer_on_server().await.json(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let res: TestResponse = server()
            .await
            .post("/rental/delete")
            .json(&json!(payload_delete_rent))
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_count_rental_get() {
        let res: TestResponse = server().await.get("/rental/count").await;

        res.assert_status_ok();
        assert!(res.json::<i64>() >= 0);
    }

    #[tokio::test]
    async fn test_count_rental_post() {
        let res: TestResponse = server().await.post("/rental/count").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }
}
