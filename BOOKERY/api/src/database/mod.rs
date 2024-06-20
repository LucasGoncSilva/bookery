use sqlx::error::Error;

type ResultDB<T> = Result<T, Error>;

pub mod author;
pub mod book;
pub mod conn;
pub mod costumer;
pub mod rental;
