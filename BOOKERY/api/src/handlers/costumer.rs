use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::conn::Database;
use crate::structs::costumer::{Costumer, PayloadCostumer, PayloadUpdateCostumer};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_costumer(
    State(db): State<DB>,
    Json(incoming_costumer): Json<PayloadCostumer>,
) -> ResultStatus<Uuid> {
    match Costumer::create(incoming_costumer) {
        Ok(costumer) => match db.create_costumer(costumer).await {
            Ok(costumer_uuid) => Ok((StatusCode::CREATED, Json(costumer_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_costumer(
    State(db): State<DB>,
    Path(costumer_uuid): Path<Uuid>,
) -> ResultStatus<Costumer> {
    match db.get_costumer(costumer_uuid).await {
        Ok(Some(costumer)) => Ok((StatusCode::OK, Json(costumer))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_costumers(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Costumer>> {
    match db.search_costumers(t.term).await {
        Ok(costumers_vec) => Ok((StatusCode::OK, Json(costumers_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_costumer(
    State(db): State<DB>,
    Json(payload_update_costumer): Json<PayloadUpdateCostumer>,
) -> ResultStatus<Uuid> {
    match db.get_costumer_id(payload_update_costumer.id).await {
        Ok(Some(_costumer_uuid)) => match Costumer::parse(payload_update_costumer) {
            Ok(updated_costumer) => match db.update_costumer(updated_costumer).await {
                Ok(costumer_uuid) => Ok((StatusCode::ACCEPTED, Json(costumer_uuid))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_costumer(
    State(db): State<DB>,
    Json(incoming_struct): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_costumer(incoming_struct.id).await {
        Ok(Some(costumer)) => match db.delete_costumer(costumer.id).await {
            Ok(costumer_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Costumer {costumer_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn count_costumers(State(db): State<DB>) -> ResultStatus<i64> {
    match db.count_costumers().await {
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

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_DOCUMENT: &'static str = "12345678901";
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

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

    async fn create_costumer_on_server() -> TestResponse {
        server()
            .await
            .post("/costumer/create")
            .json(&json!(create_payload_costumer()))
            .await
    }

    #[tokio::test]
    async fn test_create_costumer_get() {
        let res: TestResponse = server().await.get("/costumer/create").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_create_costumer_post_no_data() {
        let res: TestResponse = server().await.post("/costumer/create").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_create_costumer_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/costumer/create")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_costumer_post_valid() {
        let res: TestResponse = create_costumer_on_server().await;

        res.assert_status(StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_costumer_get_empty() {
        let res: TestResponse = server().await.get("/costumer/get/").await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_costumer_get_invalid() {
        let res: TestResponse = server().await.get("/costumer/get/12345").await;

        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_costumer_get_not_found() {
        let example_uuid: Uuid = Uuid::new_v4();
        let res: TestResponse = server()
            .await
            .get(&format!("/costumer/get/{example_uuid}"))
            .await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_costumer_get_found() {
        let costumer_created: TestResponse = create_costumer_on_server().await;

        let costumer_uuid: String = costumer_created.json();

        let res: TestResponse = server()
            .await
            .get(&format!("/costumer/get/{costumer_uuid}"))
            .await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_costumer_post() {
        let res: TestResponse = server()
            .await
            .post(&format!("/costumer/get/{}", Uuid::new_v4()))
            .await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_search_costumers_get_no_param() {
        create_costumer_on_server().await;

        let res: TestResponse = server().await.get("/costumer/search").await;
        res.assert_status_bad_request();

        let res: TestResponse = server().await.get("/costumer/search?").await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_search_costumers_get_found() {
        let create_res: TestResponse = create_costumer_on_server().await;

        let created_costumer_uuid: Uuid = create_res.json();

        let created_costumer: Costumer = server()
            .await
            .get(&format!("/costumer/get/{created_costumer_uuid}"))
            .await
            .json();

        let res: TestResponse = server().await.get("/costumer/search?term").await;
        res.assert_status_ok();
        let res_json: Vec<Costumer> = res.json();
        assert!(res_json.contains(&created_costumer));

        let res: TestResponse = server().await.get("/costumer/search?term=").await;
        res.assert_status_ok();
        let res_json: Vec<Costumer> = res.json();
        assert!(res_json.contains(&created_costumer));

        let res: TestResponse = server().await.get("/costumer/search?term=am").await;
        res.assert_status_ok();
        let res_json: Vec<Costumer> = res.json();
        assert!(res_json.contains(&created_costumer));
    }

    #[tokio::test]
    async fn test_search_costumers_post() {
        let res: TestResponse = server().await.post("/costumer/search?term=am").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_update_costumer_get() {
        let res: TestResponse = server().await.post("/costumer/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_costumer_post_no_data() {
        let res: TestResponse = server().await.post("/costumer/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_costumer_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/costumer/update")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_update_costumer_post_valid() {
        let create_res: TestResponse = create_costumer_on_server().await;

        let created_costumer_uuid: Uuid = create_res.json();

        let payload_update_costumer: PayloadUpdateCostumer = PayloadUpdateCostumer {
            id: created_costumer_uuid,
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let res: TestResponse = server()
            .await
            .post("/costumer/update")
            .json(&json!(payload_update_costumer))
            .await;

        res.assert_status(StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_delete_costumer_get() {
        let res: TestResponse = server().await.post("/costumer/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_costumer_post_no_data() {
        let res: TestResponse = server().await.post("/costumer/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_costumer_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/costumer/delete")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_costumer_post_valid() {
        let create_res: TestResponse = create_costumer_on_server().await;

        let created_costumer_uuid: Uuid = create_res.json();

        let payload_delete_costumer: PayloadUpdateCostumer = PayloadUpdateCostumer {
            id: created_costumer_uuid,
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let res: TestResponse = server()
            .await
            .post("/costumer/delete")
            .json(&json!(payload_delete_costumer))
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_count_costumer_get() {
        let res: TestResponse = server().await.get("/costumer/count").await;

        res.assert_status_ok();
        assert!(res.json::<i64>() >= 0);
    }

    #[tokio::test]
    async fn test_count_costumer_post() {
        let res: TestResponse = server().await.post("/costumer/count").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }
}
