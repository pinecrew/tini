pub enum Parsed {
    Section(String),
    Key(String, String),
    Empty,
    Error(String),
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

    Key,
    Section,
    Empty,
}

static IDENT: &'static str = "_.,:(){}-#@&*|";

pub fn parse_line(s: &str) -> Parsed {
    let mut l = String::new();
    let mut r = String::new();
    let mut sec = String::new();
    let mut ws = String::new();
    let mut state = State::Init;

    for c in s.chars() {
        match state {
            State::Init => {
                if c.is_alphabetic() || IDENT.contains(c) {
                    l.push(c);
                    state = State::ReadKey;
                } else if c == '[' {
                    state = State::ReadSection;
                } else if c.is_whitespace() {} else if c == ';' {
                    state = State::Empty;
                } else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }
            State::ReadKey => {
                if c.is_alphanumeric() || IDENT.contains(c) {
                    l.push(c);
                } else if c.is_whitespace() {
                    state = State::WhitespaceAfterKey;
                } else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }
            State::WhitespaceAfterKey => {
                if c == '=' {
                    state = State::WhitespaceBeforeValue;
                } else if c.is_whitespace() {} else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }
            State::WhitespaceBeforeValue => {
                if c.is_alphanumeric() {
                    r.push(c);
                    state = State::ReadValue;
                } else if c.is_whitespace() {} else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }

            State::ReadValue => {
                if c.is_alphanumeric() || ".+-_/!".contains(c) {
                    r.push(c);
                    state = State::ReadValue;
                } else if c.is_whitespace() || c == ',' {
                    state = State::WhitespaceAfterValue;
                    ws.push(c);
                } else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }

            State::WhitespaceAfterValue => {
                if c.is_alphanumeric() {
                    r.push_str(&ws);
                    ws.clear();
                    r.push(c);
                    state = State::ReadValue;
                } else if c == ';' {
                    state = State::Key;
                    break;
                } else if c.is_whitespace() {
                    ws.push(c);
                } else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }

            State::ReadSection => {
                if c.is_alphanumeric() || IDENT.contains(c) {
                    sec.push(c);
                } else if c == ']' {
                    state = State::WhitespaceAfterBracket;
                } else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }

            State::WhitespaceAfterBracket => {
                if c == ';' {
                    state = State::Section;
                } else if c.is_whitespace() {
                } else {
                    return Parsed::Error(format!("incorrect char `{}` in {:?}", c, state));
                }
            }
            _ => {}
        }
    }

    match state {
        State::WhitespaceAfterValue | State::ReadValue => state = State::Key,
        State::WhitespaceAfterBracket => state = State::Section,
        State::Init => state = State::Empty,
        _ => {}
    }

    let result = match state {
        State::Key => Parsed::Key(l, r),
        State::Section => Parsed::Section(sec),
        State::Empty => Parsed::Empty,
        _ => Parsed::Error("Incorrect expression".to_owned()),
    };
    result
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
            Parsed::Key(name, text) => {
                assert_eq!(name, String::from("name1"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_weird_name() {
        match parse_line("_.,:(){}-#@&*| = 100") {
            Parsed::Key(name, text) => {
                assert_eq!(name, String::from("_.,:(){}-#@&*|"));
                assert_eq!(text, String::from("100"));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_text_entry() {
        match parse_line("text_name = hello world!") {
            Parsed::Key(name, text) => {
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
}
