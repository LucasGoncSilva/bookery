pub enum ConversionError {
    TokenTooLong,
    InvalidType,
}

mod person_name {
    pub struct PersonName(String);

    impl PersonName {
        pub fn as_str(&self) -> String {
            String::from(&self.0)
        }
    }

    impl From<PersonName> for String {
        fn from(value: PersonName) -> String {
            value.0
        }
    }

    impl TryFrom<String> for PersonName {
        type Error = super::ConversionError;

        fn try_from(token: String) -> Result<Self, Self::Error> {
            if token.chars().count() > 120 {
                return Err(super::ConversionError::TokenTooLong);
            } else if !token.chars().all(|c: char| char::is_ascii_alphabetic(&c)) {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(PersonName(token))
        }
    }
}

mod born_date {
    use time::{format_description::FormatItem, macros::format_description, Date};

    const BORN_FORMAT: &[FormatItem<'static>] = format_description!("[year]-[month]-[day]");

    pub struct BornDate(Date);

    impl BornDate {
        pub fn as_str(&self) -> String {
            self.0.format(&BORN_FORMAT).unwrap()
        }
    }

    impl From<BornDate> for Date {
        fn from(value: BornDate) -> Date {
            value.0
        }
    }

    impl TryFrom<String> for BornDate {
        type Error = super::ConversionError;

        fn try_from(token: String) -> Result<Self, Self::Error> {
            match Date::parse(&token, &BORN_FORMAT) {
                Ok(date) => Ok(BornDate(date)),
                Err(_) => Err(super::ConversionError::InvalidType),
            }
        }
    }
}

pub mod author;

pub use born_date::BornDate;
pub use person_name::PersonName;
