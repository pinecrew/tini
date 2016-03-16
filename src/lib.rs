mod parser;

use std::path::Path;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

use parser::{parse, Data};

type IniData = HashMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub struct Ini(IniData);

impl<'a> Ini {
    fn new() -> Ini {
        Ini(HashMap::new())
    }
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Ini {
        let file = File::open(path)
                       .ok()
                       .expect(&format!("Can't open `{}` file!", path.as_ref().display()));
        let reader = BufReader::new(file);
        let mut result = Ini::new();
        let mut section_name = String::new();
        let mut entry_list = HashMap::new();
        for (i, line) in reader.lines().filter_map(|l| l.ok()).enumerate() {
            println!("line = `{}`", line);
            match parse(&line) {
                Data::Section(name) => {
                    if section_name.len() != 0 {
                        result.0.insert(section_name, entry_list.clone());
                        entry_list.clear();
                    }
                    println!("section = `{}`", name);
                    section_name = name;
                }
                Data::Pair(name, value) => {
                    println!("`{}` = `{}`", name, value);
                    entry_list.insert(name, value);
                }
                Data::Error(msg) => println!("line {}: error: {}", i, msg),
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
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        unimplemented!()
    }
    pub fn section<S: Into<String>>(&'a self, name: S) -> Option<&'a HashMap<String, String>> {
        let name = name.into();
        self.0.get(&name)
    }
}
