enum Data {
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

fn parse(s: &str) -> Data {
    let mut l = String::new();
    let mut r = String::new();
    let mut sec = String::new();
    let mut state = State::Init;

    for c in s.chars() {
        match state {
            State::Init => {
                if c.is_alphabetic() {
                    l.push(c);
                    state = State::ReadKey;
                } else if c == '[' {
                    state = State::ReadSection;
                } else if c.is_whitespace() {} else {
                    panic!("something went wrong!");
                }
            }
            State::ReadKey => {
                if c.is_alphabetic() {
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
                if c.is_alphanumeric() || ".+-_".contains(c) {
                    r.push(c);
                    state = State::ReadValue;
                } else if c.is_whitespace() {
                    state = State::WhitespaceAfterValue;
                } else {
                    panic!("something went wrong!");
                }
            }

            State::WhitespaceAfterValue => {
                if c == ';' {
                    state = State::Pair;
                } else if c.is_whitespace() {
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

