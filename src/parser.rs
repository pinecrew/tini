//! Parser module
//!
//! Contains `parse_line` routine to parse single line of ini file
//! and `Parsed` enum for parsing result
use crate::error::ParseError;

/// Enum for storing one of 4 possible `parse_line` results
#[derive(Debug)]
pub enum Parsed {
    // empty line
    Empty,
    // [section]
    Section(String),
    // item = value
    Value(String, String),
}

/// parse single line of ini file
pub fn parse_line(line: &str, index: usize) -> Result<Parsed, ParseError> {
    let content = match line.split(';').next() {
        Some(value) => value.trim(),
        None => return Ok(Parsed::Empty),
    };
    if content.is_empty() {
        return Ok(Parsed::Empty);
    }
    // add checks for content
    if content.starts_with('[') {
        if content.ends_with(']') {
            let section_name = content.trim_matches(|c| c == '[' || c == ']').to_owned();
            return Ok(Parsed::Section(section_name));
        }
        return Err(ParseError::IncorrectSection(index));
    }
    if content.contains('=') {
        let mut pair = content.splitn(2, '=').map(|s| s.trim());
        // if key is None => error
        let key = match pair.next() {
            Some(value) => value.to_owned(),
            None => return Err(ParseError::NoneKey(index))
        };
        // if value is None => empty string
        let value = match pair.next() {
            Some(value) => value.to_owned(),
            None => "".to_owned(),
        };
        if key.is_empty() {
            return Err(ParseError::EmptyKey(index));
        }
        return Ok(Parsed::Value(key, value));
    }
    return Err(ParseError::IncorrectSyntax(index))
}

#[cfg(test)]
mod test {
    use crate::error::Error;
    use super::*;

    #[test]
    fn test_comment() -> Result<(), Error> {
        match parse_line(";------", 0)? {
            Parsed::Empty => assert!(true),
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn test_entry() -> Result<(), Error> {
        match parse_line("name1 = 100 ; comment", 0)? {
            Parsed::Value(name, text) => {
                assert_eq!(name, String::from("name1"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn test_weird_name() -> Result<(), Error> {
        match parse_line("_.,:(){}-#@&*| = 100", 0)? {
            Parsed::Value(name, text) => {
                assert_eq!(name, String::from("_.,:(){}-#@&*|"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn test_text_entry() -> Result<(), Error> {
        match parse_line("text_name = hello world!", 0)? {
            Parsed::Value(name, text) => {
                assert_eq!(name, String::from("text_name"));
                assert_eq!(text, String::from("hello world!"));
            }
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn test_incorrect_token() {
        match parse_line("[section = 1, 2 = value", 0) {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_incorrect_key_value_line() {
        match parse_line("= 3", 0) {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }
}
