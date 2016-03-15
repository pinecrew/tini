mod parser;

use std::path::Path;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

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
        for line in reader.lines().filter_map(|l| l.ok()) {
            println!("line = `{}`", line);
            if line.contains('[') && line.contains(']') {
                let left_pos = line.find('[').unwrap() + 1;
                let right_pos = line.find(']').unwrap();
                if section_name.len() != 0 {
                    result.0.insert(section_name, entry_list.clone());
                    entry_list.clear();
                }
                section_name = (&line[left_pos..right_pos]).to_owned();
            } else if !line.starts_with(';') {
                let vec: Vec<&str> = line.split('=').collect();
                if vec.len() < 2 { continue; }
                let token = vec[0].trim_right();
                let value = if vec[1].contains(';') {
                    vec[1].split(';').nth(0).unwrap().trim()
                } else {
                    vec[1].trim()
                };
                entry_list.insert(token.to_owned(), value.to_owned());
            }
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
