use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::structs::{BookName, ConversionError, PersonName};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Rental {
    pub id: Uuid,
    pub costumer_uuid: Uuid,
    pub book_uuid: Uuid,
    #[serde(with = "super::date_format")]
    pub borrowed_at: Date,
    #[serde(with = "super::date_format")]
    pub due_date: Date,
    #[serde(with = "super::option_date_format")]
    pub returned_at: Option<Date>,
}

#[derive(Serialize, PartialEq, Debug, Deserialize)]
pub struct RentalWithCostumerAndBook {
    pub id: Uuid,
    pub costumer_name: PersonName,
    pub book_name: BookName,
    #[serde(with = "super::date_format")]
    pub borrowed_at: Date,
    #[serde(with = "super::date_format")]
    pub due_date: Date,
    #[serde(with = "super::option_date_format")]
    pub returned_at: Option<Date>,
}

#[derive(Deserialize, Serialize)]
pub struct PayloadRental {
    pub costumer_uuid: Uuid,
    pub book_uuid: Uuid,
    #[serde(with = "super::date_format")]
    pub borrowed_at: Date,
    #[serde(with = "super::date_format")]
    pub due_date: Date,
}

#[derive(Deserialize, Serialize)]
pub struct PayloadUpdateRental {
    pub id: Uuid,
    pub costumer_uuid: Uuid,
    pub book_uuid: Uuid,
    #[serde(with = "super::date_format")]
    pub borrowed_at: Date,
    #[serde(with = "super::date_format")]
    pub due_date: Date,
    #[serde(with = "super::option_date_format")]
    pub returned_at: Option<Date>,
}

impl Rental {
    pub fn create(new_rent: PayloadRental) -> Result<Self, ConversionError> {
        let id: Uuid = Uuid::new_v4();

        Ok(Self {
            id,
            costumer_uuid: new_rent.costumer_uuid,
            book_uuid: new_rent.book_uuid,
            borrowed_at: new_rent.borrowed_at,
            due_date: new_rent.due_date,
            returned_at: None,
        })
    }

    pub fn parse(rent: PayloadUpdateRental) -> Result<Self, ConversionError> {
        Ok(Self {
            id: rent.id,
            costumer_uuid: rent.costumer_uuid,
            book_uuid: rent.book_uuid,
            borrowed_at: rent.borrowed_at,
            due_date: rent.due_date,
            returned_at: rent.returned_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use time::{error::ComponentRange, Month};

    const DEFAULT_BORROWED_DATE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);
    const DEFAULT_DUE_DATE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);
    const DEFAULT_RETURNED_DATE: Result<Date, ComponentRange> =
        Date::from_calendar_date(2000, Month::January, 1);

    #[test]
    fn test_create_rent() {
        let payload_rent: PayloadRental = PayloadRental {
            book_uuid: Uuid::new_v4(),
            costumer_uuid: Uuid::new_v4(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
        };

        let rent: Rental = Rental::create(payload_rent).unwrap();

        assert_eq!(
            rent,
            Rental {
                id: rent.id,
                book_uuid: rent.book_uuid,
                costumer_uuid: rent.costumer_uuid,
                borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
                due_date: DEFAULT_DUE_DATE.unwrap(),
                returned_at: None,
            }
        );
    }

    #[test]
    fn test_parse_rent() {
        let payload_update_rent: PayloadUpdateRental = PayloadUpdateRental {
            id: Uuid::new_v4(),
            book_uuid: Uuid::new_v4(),
            costumer_uuid: Uuid::new_v4(),
            borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
            due_date: DEFAULT_DUE_DATE.unwrap(),
            returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
        };

        let rent: Rental = Rental::parse(payload_update_rent).unwrap();

        assert_eq!(
            rent,
            Rental {
                id: rent.id,
                book_uuid: rent.book_uuid,
                costumer_uuid: rent.costumer_uuid,
                borrowed_at: DEFAULT_BORROWED_DATE.unwrap(),
                due_date: DEFAULT_DUE_DATE.unwrap(),
                returned_at: Some(DEFAULT_RETURNED_DATE.unwrap()),
            }
        );
    }
}
