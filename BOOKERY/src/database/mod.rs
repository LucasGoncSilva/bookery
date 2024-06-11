use sqlx::error::Error as SqlxErr;

type ResultDB<T> = Result<T, SqlxErr>;

pub mod author;
pub mod book;
pub mod conn;
