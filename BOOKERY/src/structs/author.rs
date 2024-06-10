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
    pub fn create(new_author: PayloadAuthor) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(new_author.name)?;
        let born: Date = new_author.born;
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
