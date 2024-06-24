use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConversionError {
    TokenTooLong,
    TokenIncompatibleSize,
    InvalidType,
}

mod person_name {
    #[derive(super::Serialize, super::Deserialize, Debug, PartialEq, Clone)] // TODO compare bin with and without this params
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
            if token.len() > 128 {
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
    #[derive(super::Serialize, super::Deserialize, Debug, PartialEq, Clone)]
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
            if token.len() > 64 {
                return Err(super::ConversionError::TokenTooLong);
            }
            Ok(BookName(token))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::iter::repeat;

        use super::*;

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
    #[derive(super::Serialize, super::Deserialize, Debug, PartialEq, Clone)]
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
            if token.len() > 64 {
                return Err(super::ConversionError::TokenTooLong);
            } else if !token.chars().all(|c: char| char::is_ascii(&c)) {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(EditorName(token))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::iter::repeat;

        use super::*;

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

mod person_document {
    #[derive(super::Serialize, super::Deserialize, Debug, PartialEq, Clone)]
    pub struct PersonDocument(String);

    impl PersonDocument {
        pub fn as_str(&self) -> String {
            String::from(&self.0)
        }
    }

    impl From<PersonDocument> for String {
        fn from(value: PersonDocument) -> String {
            value.0
        }
    }

    impl TryFrom<String> for PersonDocument {
        type Error = super::ConversionError;

        fn try_from(token: String) -> Result<Self, Self::Error> {
            if token.len() != 11 {
                return Err(super::ConversionError::TokenIncompatibleSize);
            } else if !token.chars().all(|c: char| char::is_ascii_digit(&c)) {
                return Err(super::ConversionError::InvalidType);
            }

            Ok(PersonDocument(token))
        }
    }

    #[cfg(test)]
    mod tests {
        use std::iter::repeat;

        use super::*;

        #[test]
        fn test_create_person_document() {
            let document: String = "00000000000".to_string();

            let person_document_document: String = document.clone();

            let person_document: PersonDocument = PersonDocument::try_from(document).unwrap();

            assert_eq!(person_document, PersonDocument(person_document_document));
        }

        #[test]
        fn test_pass_person_document_limit() {
            let document: String = repeat("0").take(11).collect();

            PersonDocument::try_from(document).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_person_document_limit_below_expected() {
            let document: String = repeat("0").take(10).collect();

            PersonDocument::try_from(document).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_person_document_limit_above_expected() {
            let document: String = repeat("0").take(12).collect();

            PersonDocument::try_from(document).unwrap();
        }

        #[test]
        fn test_pass_person_document_charset() {
            let document: String = "12345678901".to_string();

            PersonDocument::try_from(document).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_fail_person_document_charset() {
            let document: String = "a0000000000".to_string();

            PersonDocument::try_from(document).unwrap();
        }
    }
}

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

mod option_date_format {
    use super::date_format;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Iso8601;
    use time::Date;

    pub fn serialize<S>(date: &Option<Date>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => date_format::serialize(date, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                let date = Date::parse(&s, &Iso8601::DEFAULT).map_err(serde::de::Error::custom)?;
                Ok(Some(date))
            }
            None => Ok(None),
        }
    }
}

pub mod author;
pub mod book;
pub mod costumer;
pub mod rental;

pub use book_name::BookName;
pub use editor_name::EditorName;
pub use person_document::PersonDocument;
pub use person_name::PersonName;
