use serde::Serialize;

#[derive(Debug)]
pub enum ConversionError {
    TokenTooLong,
    InvalidType,
}

mod person_name {
    #[derive(super::Serialize)]
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

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

pub mod author;

pub use person_name::PersonName;
