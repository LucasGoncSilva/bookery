use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::structs::{BornDate, ConversionError, PersonName};

#[derive(Serialize)]
pub struct Author {
    pub id: Uuid,
    pub name: PersonName,
    pub born: BornDate,
}

#[derive(Deserialize)]
pub struct NewAuthor {
    name: String,
    born: String,
}

impl Author {
    pub fn new(new_author: NewAuthor) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(new_author.name)?;
        let born: BornDate = BornDate::try_from(new_author.born)?;

        let id: Uuid = Uuid::new_v4();

        Ok(Self { id, name, born })
    }
}
