use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{extract::State, http::StatusCode, Json};
use uuid::Uuid;

use crate::database::conn::Database;
use crate::structs::book::{Book, PayloadBook, PayloadUpdateBook};

use super::{DeletingStruct, QueryURL};

type DB = Arc<Database>;
type ResultStatus<T> = Result<(StatusCode, Json<T>), StatusCode>;

pub async fn create_book(
    State(db): State<DB>,
    Json(incoming_book): Json<PayloadBook>,
) -> ResultStatus<Uuid> {
    match Book::create(incoming_book) {
        Ok(book) => match db.create_book(book).await {
            Ok(book_uuid) => Ok((StatusCode::CREATED, Json(book_uuid))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::UNPROCESSABLE_ENTITY),
    }
}

pub async fn get_book(State(db): State<DB>, Path(book_uuid): Path<Uuid>) -> ResultStatus<Book> {
    match db.get_book(book_uuid).await {
        Ok(Some(book)) => Ok((StatusCode::OK, Json(book))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn search_books(
    State(db): State<DB>,
    Query(t): Query<QueryURL>,
) -> ResultStatus<Vec<Book>> {
    match db.search_books(t.term).await {
        Ok(books_vec) => Ok((StatusCode::OK, Json(books_vec))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_book(
    State(db): State<DB>,
    Json(payload_update_book): Json<PayloadUpdateBook>,
) -> ResultStatus<Uuid> {
    match db.get_book_id(payload_update_book.id).await {
        Ok(Some(_book_uuid)) => match Book::parse(payload_update_book) {
            Ok(updated_book) => match db.update_book(updated_book).await {
                Ok(book_uuid) => Ok((StatusCode::ACCEPTED, Json(book_uuid))),
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_book(
    State(db): State<DB>,
    Json(incoming_struct): Json<DeletingStruct>,
) -> ResultStatus<String> {
    match db.get_book(incoming_struct.id).await {
        Ok(Some(book)) => match db.delete_book(book.id).await {
            Ok(book_uuid) => Ok((
                StatusCode::NO_CONTENT,
                Json(format!("Book {book_uuid} deleted")),
            )),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn count_books(State(db): State<DB>) -> ResultStatus<i64> {
    match db.count_books().await {
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
    use crate::structs::author::PayloadAuthor;

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_EDITOR: &'static str = "Editor";
    const DEFAULT_RELEASE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

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

    #[tokio::test]
    async fn test_create_book_get() {
        let res: TestResponse = server().await.get("/book/create").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_create_book_post_no_data() {
        let res: TestResponse = server().await.post("/book/create").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_create_book_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/book/create")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_create_book_post_valid() {
        let res: TestResponse = create_book_on_server().await;

        res.assert_status(StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_book_get_empty() {
        let res: TestResponse = server().await.get("/book/get/").await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_book_get_invalid() {
        let res: TestResponse = server().await.get("/book/get/12345").await;

        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_get_book_get_not_found() {
        let example_uuid: Uuid = Uuid::new_v4();
        let res: TestResponse = server()
            .await
            .get(&format!("/book/get/{example_uuid}"))
            .await;

        res.assert_status_not_found();
    }

    #[tokio::test]
    async fn test_get_book_get_found() {
        let book_created: TestResponse = create_book_on_server().await;

        let book_uuid: String = book_created.json();

        let res: TestResponse = server().await.get(&format!("/book/get/{book_uuid}")).await;

        res.assert_status_ok();
    }

    #[tokio::test]
    async fn test_get_book_post() {
        let res: TestResponse = server()
            .await
            .post(&format!("/book/get/{}", Uuid::new_v4()))
            .await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_search_books_get_no_param() {
        create_book_on_server().await;

        let res: TestResponse = server().await.get("/book/search").await;
        res.assert_status_bad_request();

        let res: TestResponse = server().await.get("/book/search?").await;
        res.assert_status_bad_request();
    }

    #[tokio::test]
    async fn test_search_books_get_found() {
        let create_res: TestResponse = create_book_on_server().await;

        let created_book_uuid: Uuid = create_res.json();

        let created_book: Book = server()
            .await
            .get(&format!("/book/get/{created_book_uuid}"))
            .await
            .json();

        let res: TestResponse = server().await.get("/book/search?term").await;
        res.assert_status_ok();
        let res_json: Vec<Book> = res.json();
        assert!(res_json.contains(&created_book));

        let res: TestResponse = server().await.get("/book/search?term=").await;
        res.assert_status_ok();
        let res_json: Vec<Book> = res.json();
        assert!(res_json.contains(&created_book));

        let res: TestResponse = server().await.get("/book/search?term=am").await;
        res.assert_status_ok();
        let res_json: Vec<Book> = res.json();
        assert!(res_json.contains(&created_book));
    }

    #[tokio::test]
    async fn test_search_books_post() {
        let res: TestResponse = server().await.post("/book/search?term=am").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn test_update_book_get() {
        let res: TestResponse = server().await.post("/book/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_book_post_no_data() {
        let res: TestResponse = server().await.post("/book/update").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_update_book_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/book/update")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_update_book_post_valid() {
        let create_res: TestResponse = create_book_on_server().await;

        let created_book_uuid: Uuid = create_res.json();

        let created_book: Book = server()
            .await
            .get(&format!("/book/get/{created_book_uuid}"))
            .await
            .json();

        let created_book_author_uuid: Uuid = created_book.author_uuid;

        let payload_update_book: PayloadUpdateBook = PayloadUpdateBook {
            id: created_book_uuid,
            name: DEFAULT_NAME.to_string(),
            author_uuid: created_book_author_uuid,
            editor: DEFAULT_EDITOR.to_string(),
            release: DEFAULT_RELEASE.unwrap(),
        };

        let res: TestResponse = server()
            .await
            .post("/book/update")
            .json(&json!(payload_update_book))
            .await;

        res.assert_status(StatusCode::ACCEPTED);
    }

    #[tokio::test]
    async fn test_delete_book_get() {
        let res: TestResponse = server().await.post("/book/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_book_post_no_data() {
        let res: TestResponse = server().await.post("/book/delete").await;

        res.assert_status(StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    async fn test_delete_book_post_invalid() {
        let res: TestResponse = server()
            .await
            .post("/book/delete")
            .json(&json!({"name":"Name"}))
            .await;

        res.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_delete_book_post_valid() {
        let create_res: TestResponse = create_book_on_server().await;

        let created_book_uuid: Uuid = create_res.json();

        let created_book: Book = server()
            .await
            .get(&format!("/book/get/{created_book_uuid}"))
            .await
            .json();

        let created_book_author_uuid: Uuid = created_book.author_uuid;

        let payload_delete_book: PayloadUpdateBook = PayloadUpdateBook {
            id: created_book_uuid,
            name: DEFAULT_NAME.to_string(),
            author_uuid: created_book_author_uuid,
            editor: DEFAULT_EDITOR.to_string(),
            release: DEFAULT_RELEASE.unwrap(),
        };

        let res: TestResponse = server()
            .await
            .post("/book/delete")
            .json(&json!(payload_delete_book))
            .await;

        res.assert_status(StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_count_book_get() {
        let res: TestResponse = server().await.get("/book/count").await;

        res.assert_status_ok();
        assert!(res.json::<i64>() >= 0);
    }

    #[tokio::test]
    async fn test_count_book_post() {
        let res: TestResponse = server().await.post("/book/count").await;

        res.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    }
}
