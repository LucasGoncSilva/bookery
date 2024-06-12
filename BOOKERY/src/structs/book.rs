use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::structs::{BookName, ConversionError, EditorName};

#[derive(Serialize, Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use time::{error::ComponentRange, Month};

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_EDITOR: &'static str = "Editor";
    const DEFAULT_RELEASE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    #[test]
    fn test_create_book() {
        let payload_book: PayloadBook = PayloadBook {
            name: DEFAULT_NAME.to_string(),
            author_uuid: Uuid::new_v4(),
            editor: DEFAULT_EDITOR.to_string(),
            release: DEFAULT_RELEASE.unwrap(),
        };

        let book: Book = Book::create(payload_book).unwrap();

        assert_eq!(
            book,
            Book {
                id: book.id,
                name: BookName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                author_uuid: book.author_uuid,
                editor: EditorName::try_from(DEFAULT_EDITOR.to_string()).unwrap(),
                release: DEFAULT_RELEASE.unwrap(),
            }
        );
    }

    #[test]
    fn test_parse_book() {
        let payload_update_book: PayloadUpdateBook = PayloadUpdateBook {
            id: Uuid::new_v4(),
            name: DEFAULT_NAME.to_string(),
            author_uuid: Uuid::new_v4(),
            editor: DEFAULT_EDITOR.to_string(),
            release: DEFAULT_RELEASE.unwrap(),
        };

        let book_uuid: Uuid = payload_update_book.id.clone();
        let book_author_uuid: Uuid = payload_update_book.author_uuid.clone();

        let book: Book = Book::parse(payload_update_book).unwrap();

        assert_eq!(
            book,
            Book {
                id: book_uuid,
                name: BookName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                author_uuid: book_author_uuid,
                editor: EditorName::try_from(DEFAULT_EDITOR.to_string()).unwrap(),
                release: DEFAULT_RELEASE.unwrap(),
            }
        );
    }
}
