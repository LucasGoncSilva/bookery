use serde::Serialize;

#[derive(Debug)]
pub enum ConversionError {
    TokenTooLong,
    InvalidType,
}

mod person_name {
    #[derive(super::Serialize, Debug, PartialEq, Clone)] // TODO compare bin with and without this params
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
            } else if !token
                .chars()
                .all(|c: char| char::is_ascii_alphabetic(&c) || c == ' ')
            {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(PersonName(token))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::iter::repeat;

        use super::*;

        #[test]
        fn test_create_person_name() {
            let name: String = "Name".to_string();

            let person_name_name: String = name.clone();

            let person_name: PersonName = PersonName::try_from(name).unwrap();

            assert_eq!(person_name, PersonName(person_name_name));
        }

        #[test]
        fn test_pass_person_name_limit() {
            let name: String = repeat("x").take(128).collect();

            PersonName::try_from(name).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_person_name_limit() {
            let name: String = repeat("x").take(129).collect();

            PersonName::try_from(name).unwrap();
        }

        #[test]
        fn test_pass_person_name_charset() {
            let name: String = "abcxyz ABCXYZ".to_string();

            PersonName::try_from(name).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_person_name_charset() {
            let name: String = "abcxyz ABCXYZ 012789".to_string();

            PersonName::try_from(name).unwrap();
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
            if token.chars().count() > 64 {
                return Err(super::ConversionError::TokenTooLong);
            }
            Ok(BookName(token))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::{
            fmt::{Debug, Formatter, Result as FmtResult},
            iter::repeat,
        };

        use super::*;

        impl PartialEq for BookName {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Debug for BookName {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_struct("BookName").field("0", &self.0).finish()
            }
        }

        #[test]
        fn test_create_book_name() {
            let name: String = "Name".to_string();

            let person_name_name: String = name.clone();

            let person_name: BookName = BookName::try_from(name).unwrap();

            assert_eq!(person_name, BookName(person_name_name));
        }

        #[test]
        fn test_pass_book_name_limit() {
            let name: String = repeat("x").take(64).collect();

            BookName::try_from(name).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_book_name_limit() {
            let name: String = repeat("x").take(65).collect();

            BookName::try_from(name).unwrap();
        }

        #[test]
        fn test_pass_book_name_charset() {
            let name: String = "abcxyz ABCXYZ 012789 ,!?([{ 👍".to_string();

            BookName::try_from(name).unwrap();
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
            } else if !token.chars().all(|c: char| char::is_ascii(&c)) {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(EditorName(token))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::{
            fmt::{Debug, Formatter, Result as FmtResult},
            iter::repeat,
        };

        use super::*;

        impl PartialEq for EditorName {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Debug for EditorName {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_struct("EditorName").field("0", &self.0).finish()
            }
        }

        #[test]
        fn test_create_editor_name() {
            let name: String = "Name".to_string();

            let person_name_name: String = name.clone();

            let person_name: EditorName = EditorName::try_from(name).unwrap();

            assert_eq!(person_name, EditorName(person_name_name));
        }

        #[test]
        fn test_pass_editor_name_limit() {
            let name: String = repeat("x").take(64).collect();

            EditorName::try_from(name).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_editor_name_limit() {
            let name: String = repeat("x").take(65).collect();

            EditorName::try_from(name).unwrap();
        }

        #[test]
        fn test_pass_editor_name_charset() {
            let name: String = "abcxyz ABCXYZ 012789 ,!?([{".to_string();

            EditorName::try_from(name).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_editor_name_charset() {
            let name: String = "abcxyz ABCXYZ 012789 ,!?([{ 👍".to_string();

            EditorName::try_from(name).unwrap();
        }
    }
}

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

pub mod author;
pub mod book;

pub use book_name::BookName;
pub use editor_name::EditorName;
pub use person_name::PersonName;
