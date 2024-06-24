use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::structs::{ConversionError, PersonDocument, PersonName};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Costumer {
    pub id: Uuid,
    pub name: PersonName,
    pub document: PersonDocument,
    #[serde(with = "super::date_format")]
    pub born: Date,
}

#[derive(Deserialize, Serialize)]
pub struct PayloadCostumer {
    pub name: String,
    pub document: String,
    #[serde(with = "super::date_format")]
    pub born: Date,
}

#[derive(Deserialize, Serialize)]
pub struct PayloadUpdateCostumer {
    pub id: Uuid,
    pub name: String,
    pub document: String,
    #[serde(with = "super::date_format")]
    pub born: Date,
}

impl Costumer {
    pub fn create(new_costumer: PayloadCostumer) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(new_costumer.name)?;
        let document: PersonDocument = PersonDocument::try_from(new_costumer.document)?;
        let born: Date = new_costumer.born;
        let id: Uuid = Uuid::new_v4();

        Ok(Self {
            id,
            name,
            document,
            born,
        })
    }

    pub fn parse(costumer: PayloadUpdateCostumer) -> Result<Self, ConversionError> {
        let name: PersonName = PersonName::try_from(costumer.name)?;
        let document: PersonDocument = PersonDocument::try_from(costumer.document)?;
        let born: Date = costumer.born;

        Ok(Self {
            id: costumer.id,
            name,
            document,
            born,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::{error::ComponentRange, Month};

    const DEFAULT_NAME: &'static str = "Name";
    const DEFAULT_DOCUMENT: &'static str = "12345678901";
    const DEFAULT_BORN: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    #[test]
    fn test_create_costumer() {
        let payload_costumer: PayloadCostumer = PayloadCostumer {
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let costumer: Costumer = Costumer::create(payload_costumer).unwrap();

        assert_eq!(
            costumer,
            Costumer {
                id: costumer.id,
                name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                document: PersonDocument::try_from(DEFAULT_DOCUMENT.to_string()).unwrap(),
                born: DEFAULT_BORN.unwrap(),
            }
        );
    }

    #[test]
    fn test_parse_costumer() {
        let payload_update_costumer: PayloadUpdateCostumer = PayloadUpdateCostumer {
            id: Uuid::new_v4(),
            name: DEFAULT_NAME.to_string(),
            document: DEFAULT_DOCUMENT.to_string(),
            born: DEFAULT_BORN.unwrap(),
        };

        let costumer_uuid: Uuid = payload_update_costumer.id.clone();

        let costumer: Costumer = Costumer::parse(payload_update_costumer).unwrap();

        assert_eq!(
            costumer,
            Costumer {
                id: costumer_uuid,
                name: PersonName::try_from(DEFAULT_NAME.to_string()).unwrap(),
                document: PersonDocument::try_from(DEFAULT_DOCUMENT.to_string()).unwrap(),
                born: DEFAULT_BORN.unwrap(),
            }
        );
    }
}
