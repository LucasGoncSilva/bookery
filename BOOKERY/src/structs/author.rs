use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::structs::{ConversionError, PersonName};

#[derive(Serialize)]
pub struct Author {
    pub id: Uuid,
    pub name: PersonName,
    #[serde(with = "super::date_format")]
    pub born: Date,
}

#[derive(Deserialize)]
pub struct PayloadAuthor {
    name: String,
    #[serde(with = "super::date_format")]
    born: Date,
}

#[derive(Deserialize)]
pub struct PayloadUpdateAuthor {
    pub id: Uuid,
    name: String,
    #[serde(with = "super::date_format")]
    born: Date,
}

impl Author {
    pub fn create(payload_author: PayloadAuthor) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(payload_author.name)?;
        let born: Date = payload_author.born;
        let id: Uuid = Uuid::new_v4();

        Ok(Self { id, name, born })
    }

    pub fn parse(author: PayloadUpdateAuthor) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(author.name)?;
        let born: Date = author.born;

        Ok(Self {
            id: author.id,
            name,
            born,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::{Debug, Formatter, Result as FmtResult};

    use time::{error::ComponentRange, Month};

    impl PartialEq for Author {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
                && self.name.as_str() == other.name.as_str()
                && self.born == other.born
        }
    }

    impl Debug for Author {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_struct("Author")
                .field("id", &self.id)
                .field("name", &self.name.as_str())
                .field("born", &self.born)
                .finish()
        }
    }

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    #[test]
    fn test_create_author() {
        let payload_author: PayloadAuthor = PayloadAuthor {
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let author: Author = Author::create(payload_author).unwrap();

        assert_eq!(
            author,
            Author {
                id: author.id,
                name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                born: DEFAULT_BORN.unwrap(),
            }
        );
    }

    #[test]
    fn test_parse_author() {
        let payload_update_author: PayloadUpdateAuthor = PayloadUpdateAuthor {
            id: Uuid::new_v4(),
            name: DEFAULT_NAME.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let author_uuid: Uuid = payload_update_author.id.clone();

        let author: Author = Author::parse(payload_update_author).unwrap();

        assert_eq!(
            author,
            Author {
                id: author_uuid,
                name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                born: DEFAULT_BORN.unwrap(),
            }
        );
    }
}
