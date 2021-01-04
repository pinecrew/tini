//! Parser module
//!
//! Contains `parse_line` routine to parse single line of ini file
//! and `Parsed` enum for parsing result
use std::error;
use std::fmt;

/// Enum for storing one of 4 possible `parse_line` results
#[derive(Debug)]
pub enum Parsed {
    // parse error
    Error(ParseError),
    // empty line
    Empty,
    // [section]
    Section(String),
    // item = value
    Value(String, String),
}

#[derive(Debug)]
pub enum ParseError {
    IncorrectSection,
    IncorrectSyntax,
    NoneKey,
    EmptyKey,
}

/// parse single line of ini file
pub fn parse_line(line: &str) -> Parsed {
    let content = match line.split(';').next() {
        Some(value) => value.trim(),
        None => return Parsed::Empty,
    };
    if content.is_empty() {
        return Parsed::Empty;
    }
    // add checks for content
    if content.starts_with('[') {
        if content.ends_with(']') {
            let section_name = content.trim_matches(|c| c == '[' || c == ']').to_owned();
            return Parsed::Section(section_name);
        } else {
            return Parsed::Error(ParseError::IncorrectSection);
        }
    } else if content.contains('=') {
        let mut pair = content.splitn(2, '=').map(|s| s.trim());
        // if key is None => error
        let key = match pair.next() {
            Some(value) => value.to_owned(),
            None => return Parsed::Error(ParseError::NoneKey)
        };
        // if value is None => empty string
        let value = match pair.next() {
            Some(value) => value.to_owned(),
            None => "".to_owned(),
        };
        if key.is_empty() {
            return Parsed::Error(ParseError::EmptyKey);
        }
        return Parsed::Value(key, value);
    }
    return Parsed::Error(ParseError::IncorrectSyntax)
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IncorrectSection => write!(f, "Incorrect section syntax"),
            ParseError::IncorrectSyntax => write!(f, "Incorrect syntax"),
            ParseError::NoneKey => write!(f, "Key is None"),
            ParseError::EmptyKey => write!(f, "Key is empty"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comment() {
        match parse_line(";------") {
            Parsed::Empty => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_entry() {
        match parse_line("name1 = 100 ; comment") {
            Parsed::Value(name, text) => {
                assert_eq!(name, String::from("name1"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_weird_name() {
        match parse_line("_.,:(){}-#@&*| = 100") {
            Parsed::Value(name, text) => {
                assert_eq!(name, String::from("_.,:(){}-#@&*|"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_text_entry() {
        match parse_line("text_name = hello world!") {
            Parsed::Value(name, text) => {
                assert_eq!(name, String::from("text_name"));
                assert_eq!(text, String::from("hello world!"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_incorrect_token() {
        match parse_line("[section = 1, 2 = value") {
            Parsed::Error(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_incorrect_key_value_line() {
        match parse_line("= 3") {
            Parsed::Error(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
