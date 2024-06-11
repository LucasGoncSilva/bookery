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
            if token.chars().count() > 128 {
                return Err(super::ConversionError::TokenTooLong);
            } else if !token.chars().all(|c: char| char::is_ascii_alphabetic(&c)) {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(PersonName(token))
        }
    }
}

mod book_name {
    #[derive(super::Serialize)]
    pub struct BookName(String);

    impl BookName {
        pub fn as_str(&self) -> String {
            String::from(&self.0)
        }
    }

    impl From<BookName> for String {
        fn from(value: BookName) -> String {
            value.0
        }
    }

    impl TryFrom<String> for BookName {
        type Error = super::ConversionError;

        fn try_from(token: String) -> Result<Self, Self::Error> {
            if token.chars().count() > 120 {
                return Err(super::ConversionError::TokenTooLong);
            }
            Ok(BookName(token))
        }
    }
}

mod editor_name {
    #[derive(super::Serialize)]
    pub struct EditorName(String);

    impl EditorName {
        pub fn as_str(&self) -> String {
            String::from(&self.0)
        }
    }

    impl From<EditorName> for String {
        fn from(value: EditorName) -> String {
            value.0
        }
    }

    impl TryFrom<String> for EditorName {
        type Error = super::ConversionError;

        fn try_from(token: String) -> Result<Self, Self::Error> {
            if token.chars().count() > 64 {
                return Err(super::ConversionError::TokenTooLong);
            } else if !token.chars().all(|c: char| char::is_ascii_alphabetic(&c)) {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(EditorName(token))
        }
    }
}

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

pub mod author;
pub mod book;

pub use book_name::BookName;
pub use editor_name::EditorName;
pub use person_name::PersonName;
