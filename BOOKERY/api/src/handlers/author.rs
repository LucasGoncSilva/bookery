use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::conn::Database;
use crate::structs::author::{Author, PayloadAuthor, PayloadUpdateAuthor};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_author(
    State(db): State<DB>,
    Json(incoming_author): Json<PayloadAuthor>,
) -> ResultStatus<Uuid> {
    match Author::create(incoming_author) {
        Ok(author) => match db.create_author(author).await {
            Ok(author_uuid) => Ok((StatusCode::CREATED, Json(author_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_author(
    State(db): State<DB>,
    Path(author_uuid): Path<Uuid>,
) -> ResultStatus<Author> {
    match db.get_author(author_uuid).await {
        Ok(Some(author)) => Ok((StatusCode::OK, Json(author))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_authors(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Author>> {
    match db.search_authors(t.term).await {
        Ok(authors_vec) => Ok((StatusCode::OK, Json(authors_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_author(
    State(db): State<DB>,
    Json(payload_update_author): Json<PayloadUpdateAuthor>,
) -> ResultStatus<Uuid> {
    match db.get_author_id(payload_update_author.id).await {
        Ok(Some(_author_uuid)) => match Author::parse(payload_update_author) {
            Ok(updated_author) => match db.update_author(updated_author).await {
                Ok(author_uuid) => Ok((StatusCode::ACCEPTED, Json(author_uuid))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_author(
    State(db): State<DB>,
    Json(incoming_struct): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_author(incoming_struct.id).await {
        Ok(Some(author)) => match db.delete_author(author.id).await {
            Ok(author_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Author {author_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn count_authors(State(db): State<DB>) -> ResultStatus<i64> {
    match db.count_authors().await {
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
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    fn create_payload_author() -> PayloadAuthor {
        PayloadAuthor {
            name: DEFAULT_NAME.to_string(),
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

    #[tokio::test]
    async fn test_create_author_get() {
        let res: TestResponse = server().await.get("/author/create").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_create_author_post_no_data() {
        let res: TestResponse = server().await.post("/author/create").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_create_author_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/author/create")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_author_post_valid() {
        let res: TestResponse = create_author_on_server().await;

        res.assert_status(StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_author_get_empty() {
        let res: TestResponse = server().await.get("/author/get/").await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_author_get_invalid() {
        let res: TestResponse = server().await.get("/author/get/12345").await;

        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_author_get_not_found() {
        let example_uuid: Uuid = Uuid::new_v4();
        let res: TestResponse = server()
            .await
            .get(&format!("/author/get/{example_uuid}"))
            .await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_author_get_found() {
        let author_created: TestResponse = create_author_on_server().await;

        let author_uuid: String = author_created.json();

        let res: TestResponse = server()
            .await
            .get(&format!("/author/get/{author_uuid}"))
            .await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_author_post() {
        let res: TestResponse = server()
            .await
            .post(&format!("/author/get/{}", Uuid::new_v4()))
            .await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_search_authors_get_no_param() {
        create_author_on_server().await;

        let res: TestResponse = server().await.get("/author/search").await;
        res.assert_status_bad_request();

        let res: TestResponse = server().await.get("/author/search?").await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_search_authors_get_found() {
        let create_res: TestResponse = create_author_on_server().await;

        let created_author_uuid: Uuid = create_res.json();

        let created_author: Author = server()
            .await
            .get(&format!("/author/get/{created_author_uuid}"))
            .await
            .json();

        let res: TestResponse = server().await.get("/author/search?term").await;
        res.assert_status_ok();
        let res_json: Vec<Author> = res.json();
        assert!(res_json.contains(&created_author));

        let res: TestResponse = server().await.get("/author/search?term=").await;
        res.assert_status_ok();
        let res_json: Vec<Author> = res.json();
        assert!(res_json.contains(&created_author));

        let res: TestResponse = server().await.get("/author/search?term=am").await;
        res.assert_status_ok();
        let res_json: Vec<Author> = res.json();
        assert!(res_json.contains(&created_author));
    }

    #[tokio::test]
    async fn test_search_authors_post() {
        let res: TestResponse = server().await.post("/author/search?term=am").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_update_author_get() {
        let res: TestResponse = server().await.post("/author/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_author_post_no_data() {
        let res: TestResponse = server().await.post("/author/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_author_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/author/update")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_update_author_post_valid() {
        let create_res: TestResponse = create_author_on_server().await;

        let created_author_uuid: Uuid = create_res.json();

        let payload_update_author: PayloadUpdateAuthor = PayloadUpdateAuthor {
            id: created_author_uuid,
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let res: TestResponse = server()
            .await
            .post("/author/update")
            .json(&json!(payload_update_author))
            .await;

        res.assert_status(StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_delete_author_get() {
        let res: TestResponse = server().await.post("/author/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_author_post_no_data() {
        let res: TestResponse = server().await.post("/author/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_author_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/author/delete")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_author_post_valid() {
        let create_res: TestResponse = create_author_on_server().await;

        let created_author_uuid: Uuid = create_res.json();

        let payload_delete_author: PayloadUpdateAuthor = PayloadUpdateAuthor {
            id: created_author_uuid,
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let res: TestResponse = server()
            .await
            .post("/author/delete")
            .json(&json!(payload_delete_author))
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_count_author_get() {
        let res: TestResponse = server().await.get("/author/count").await;

        res.assert_status_ok();
        assert!(res.json::<i64>() >= 0);
    }

    #[tokio::test]
    async fn test_count_author_post() {
        let res: TestResponse = server().await.post("/author/count").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }
}
