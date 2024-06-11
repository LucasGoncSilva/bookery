use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::structs::{BookName, ConversionError, EditorName};

#[derive(Serialize)]
pub struct Book {
    pub id: Uuid,
    pub name: BookName,
    pub author_uuid: Uuid,
    pub editor: EditorName,
    #[serde(with = "super::date_format")]
    pub release: Date,
}

#[derive(Deserialize)]
pub struct PayloadBook {
    pub name: String,
    pub author_uuid: Uuid,
    pub editor: String,
    #[serde(with = "super::date_format")]
    pub release: Date,
}

#[derive(Deserialize)]
pub struct PayloadUpdateBook {
    pub id: Uuid,
    pub name: String,
    pub author_uuid: Uuid,
    pub editor: String,
    #[serde(with = "super::date_format")]
    pub release: Date,
}

impl Book {
    pub fn create(new_book: PayloadBook) -> Result<Self, ConversionError> {
        let name: BookName = BookName::try_from(new_book.name)?;
        let editor: EditorName = EditorName::try_from(new_book.editor)?;
        let release: Date = new_book.release;
        let author_uuid: Uuid = new_book.author_uuid;
        let id: Uuid = Uuid::new_v4();

        Ok(Self {
            id,
            name,
            author_uuid,
            editor,
            release,
        })
    }

    pub fn parse(book: PayloadUpdateBook) -> Result<Self, ConversionError> {
        let name: BookName = BookName::try_from(book.name)?;
        let editor: EditorName = EditorName::try_from(book.editor)?;
        let release: Date = book.release;
        let author_uuid: Uuid = book.author_uuid;

        Ok(Self {
            id: book.id,
            name,
            author_uuid,
            editor,
            release,
        })
    }
}
