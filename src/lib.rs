mod parser;

use std::path::Path;
use std::collections::HashMap;
use std::io::{BufReader, Read};
use std::fs::File;

use parser::{parse_line, Parsed};

type IniParsed = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub struct Ini(IniParsed);

impl<'a> Ini {
    fn new() -> Ini {
        Ini(HashMap::new())
    }
    fn from_string(string: &str) -> Ini {
        let mut result = Ini::new();
        let mut section_name = String::new();
        let mut entry_list = HashMap::new();
        for (i, line) in string.lines().enumerate() {
            match parse_line(&line) {
                Parsed::Section(name) => {
                    if section_name.len() != 0 {
                        result.0.insert(section_name, entry_list.clone());
                        entry_list.clear();
                    }
                    section_name = name;
                }
                Parsed::Key(name, value) => {
                    entry_list.insert(name, value);
                }
                Parsed::Error(msg) => println!("line {}: error: {}", i, msg),
                _ => (),
            };
        }
        // add last section
        if section_name.len() != 0 {
            result.0.insert(section_name, entry_list.clone());
            entry_list.clear();
        }
        result
    }

    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Ini {
        let file = File::open(path)
                       .ok()
                       .expect(&format!("Can't open `{}`!", path.as_ref().display()));
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        let _ = reader.read_to_string(&mut buffer)
                      .ok()
                      .expect(&format!("Can't read `{}`!", path.as_ref().display()));
        Ini::from_string(&buffer)
    }
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }
    pub fn section<S: Into<String>>(&'a self, name: S) -> Option<&'a HashMap<String, String>> {
        let name = name.into();
        self.0.get(&name)
    }
}
