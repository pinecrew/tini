//! Parser module
//!
//! Contains `parse_line` routine to parse single line of ini file
//! and `Parsed` enum for parsing result
use crate::error::ParseError;

/// Enum for storing one of 4 possible `parse_line` results
#[derive(Debug)]
pub enum Parsed {
    /// empty line
    Empty,
    /// [section]
    Section(String),
    /// item = value
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
            None => return Err(ParseError::EmptyKey(index)),
        };
        if key.is_empty() {
            return Err(ParseError::EmptyKey(index));
        }
        // if value is None => empty string
        let value = match pair.next() {
            Some(value) => value.to_owned(),
            None => "".to_owned(),
        };
        return Ok(Parsed::Value(key, value));
    }
    Err(ParseError::IncorrectSyntax(index))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;

    #[test]
    fn comment() -> Result<(), Error> {
        match parse_line(";------", 0)? {
            Parsed::Empty => assert!(true),
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn entry() -> Result<(), Error> {
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
    fn section() -> Result<(), Error> {
        match parse_line("[section]", 0)? {
            Parsed::Section(name) => assert_eq!(name, String::from("section")),
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn weird_name() -> Result<(), Error> {
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
    fn weird_section() -> Result<(), Error> {
        match parse_line("[[abc]] ; omg", 0)? {
            Parsed::Section(name) => assert_eq!(name, String::from("abc")),
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn text_entry() -> Result<(), Error> {
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
    fn incorrect_token() {
        match parse_line("[section = 1, 2 = value", 0) {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn empty_key() {
        match parse_line("= 3", 0) {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn empty_kv() {
        match parse_line("=", 0) {
            Err(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn empty_value() -> Result<(), Error> {
        match parse_line("a =", 0)? {
            Parsed::Value(key, value) => {
                assert_eq!(key, String::from("a"));
                assert_eq!(value.len(), 0);
            }
            _ => assert!(false),
        }
        Ok(())
    }

    #[test]
    fn empty_value_with_comment() -> Result<(), Error> {
        match parse_line("a = ; comment line", 0)? {
            Parsed::Value(key, value) => {
                assert_eq!(key, String::from("a"));
                assert_eq!(value.len(), 0);
            }
            _ => assert!(false),
        }
        Ok(())
    }
}
