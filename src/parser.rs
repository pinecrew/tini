pub enum Data {
    Section(String),
    Pair(String, String),
    Empty,
    Error,
}

#[derive(Debug)]
enum State {
    Init,

    ReadKey,
    ReadSection,
    ReadValue,
    WhitespaceAfterBracket,
    WhitespaceAfterKey,
    WhitespaceAfterValue,
    WhitespaceBeforeValue,

    Pair,
    Section,
    Empty,
}

pub fn parse(s: &str) -> Data {
    let mut l = String::new();
    let mut r = String::new();
    let mut sec = String::new();
    let mut ws = String::new();
    let mut state = State::Init;

    for c in s.chars() {
        match state {
            State::Init => {
                if c.is_alphabetic() || "_.,:(){}-#@&*|".contains(c) {
                    l.push(c);
                    state = State::ReadKey;
                } else if c == '[' {
                    state = State::ReadSection;
                } else if c.is_whitespace() {} else if c == ';' {
                    state = State::Empty;
                } else {
                    panic!("something went wrong!");
                }
            }
            State::ReadKey => {
                if c.is_alphanumeric() || "_.,:(){}-#@&*|".contains(c) {
                    l.push(c);
                } else if c.is_whitespace() {
                    state = State::WhitespaceAfterKey;
                } else {
                    panic!("something went wrong!");
                }
            }
            State::WhitespaceAfterKey => {
                if c == '=' {
                    state = State::WhitespaceBeforeValue;
                } else if c.is_whitespace() {} else {
                    panic!("something went wrong!");
                }
            }
            State::WhitespaceBeforeValue => {
                if c.is_alphanumeric() {
                    r.push(c);
                    state = State::ReadValue;
                } else if c.is_whitespace() {} else {
                    panic!("something went wrong!");
                }
            }

            State::ReadValue => {
                if c.is_alphanumeric() || ".+-_/".contains(c) {
                    r.push(c);
                    state = State::ReadValue;
                } else if c.is_whitespace() || c == ',' {
                    state = State::WhitespaceAfterValue;
                } else {
                    panic!("incorrect char `{}`", c);
                }
            }

            State::WhitespaceAfterValue => {
                if c.is_alphanumeric() {
                    r.push_str(&ws);
                    ws.clear();
                    r.push(c);
                    state = State::ReadValue;
                } else if c == ';' {
                    state = State::Pair;
                    break;
                } else if c.is_whitespace() {
                    ws.push(c);
                } else {
                    panic!("something went wrong!");
                }
            }

            State::ReadSection => {
                if c.is_alphanumeric() || ".-_".contains(c) {
                    sec.push(c);
                } else if c == ']' {
                    state = State::WhitespaceAfterBracket;
                } else {
                    panic!("something went wrong!");
                }
            }

            State::WhitespaceAfterBracket => {
                if c == ';' {
                    state = State::Section;
                } else if c.is_whitespace() {
                } else {
                    panic!("something went wrong!");
                }
            }
            _ => {}
        }
    }

    match state {
        State::WhitespaceAfterValue | State::ReadValue => state = State::Pair,
        State::WhitespaceAfterBracket => state = State::Section,
        _ => {}
    }

    let result = match state {
        State::Pair => Data::Pair(l, r),
        State::Section => Data::Section(sec),
        State::Empty => Data::Empty,
        _ => Data::Error,
    };
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_comment() {
        match parse(";------") {
            Data::Empty => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_entry() {
        match parse("name1 = 100 ; comment") {
            Data::Pair(name, text) => {
                assert_eq!(name, String::from("name1"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_weird_name() {
        match parse("_.,:(){}-#@&*| = 100") {
            Data::Pair(name, text) => {
                assert_eq!(name, String::from("_.,:(){}-#@&*|"));
                assert_eq!(text, String::from("100"));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_text_entry() {
        match parse("text_name = hello world!") {
            Data::Pair(name, text) => {
                assert_eq!(name, String::from("text_name"));
                assert_eq!(text, String::from("hello world!"));
            },
            _ => assert!(false)
        }
    }

    #[test]
    fn test_incorrect_token() {
        match parse("[section = 1, 2 = value") {
            Data::Error => assert!(true),
            _ => assert!(false)
        }
    }
}