//! _**tini** is a **t**iny **ini**-file parsing library_
//!
//! This small library provides basic functions to operate with ini-files.
//!
//! Features:
//!
//! * no dependencies;
//! * parsing [from file](struct.Ini.html#method.from_file) and [from buffer](struct.Ini.html#method.from_buffer);
//! * [convert parsed value to given type](struct.Ini.html#method.get);
//! * [parse comma-separated lists to vectors](struct.Ini.html#method.get_vec);
//! * construct new ini-structure with [method chaining](struct.Ini.html#method.item);
//! * writing [to file](struct.Ini.html#method.to_file) and [to buffer](struct.Ini.html#method.to_buffer).
//!
//! # Examples
//! ## Read from buffer and get string values
//! ````
//! use tini::Ini;
//!
//! let conf = Ini::from_buffer(["[search]",
//!                              "g = google.com",
//!                              "dd = duckduckgo.com"].join("\n"));
//!
//! let g: String = conf.get("search", "g").unwrap();
//! let dd: String = conf.get("search", "dd").unwrap();
//!
//! assert_eq!(g, "google.com");
//! assert_eq!(dd, "duckduckgo.com");
//! ````
//! ## Construct in program and get vectors
//! ````
//! use tini::Ini;
//!
//! let conf = Ini::new().section("floats")
//!                      .item("consts", "3.1416, 2.7183")
//!                      .section("integers")
//!                      .item("lost", "4,8,15,16,23,42");
//! let consts: Vec<f64> = conf.get_vec("floats", "consts").unwrap();
//! let lost: Vec<i32> = conf.get_vec("integers", "lost").unwrap();
//!
//! assert_eq!(consts, [3.1416, 2.7183]);
//! assert_eq!(lost, [4, 8, 15, 16, 23, 42]);
//! ````
use std::path::Path;
use std::collections::{btree_map, BTreeMap};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::iter::Iterator;
use std::fs::File;
use std::str::FromStr;
use parser::{parse_line, Parsed};
use std::fmt;

type Section = BTreeMap<String, String>;
type IniParsed = BTreeMap<String, Section>;
type SectionIter<'a> = btree_map::Iter<'a, String, String>;
type SectionIterMut<'a> = btree_map::IterMut<'a, String, String>;

/// Structure for INI-file data
#[derive(Debug)]
pub struct Ini {
    #[doc(hidden)] data: IniParsed,
    last_section_name: String,
}

impl Ini {
    /// Create an empty Ini
    pub fn new() -> Ini {
        Ini {
            data: IniParsed::new(),
            last_section_name: String::new(),
        }
    }
    fn from_string(string: &str) -> Ini {
        let mut result = Ini::new();
        for (i, line) in string.lines().enumerate() {
            match parse_line(&line) {
                Parsed::Section(name) => result = result.section(name),
                Parsed::Value(name, value) => result = result.item(name, value),
                Parsed::Error(msg) => println!("line {}: error: {}", i, msg),
                _ => (),
            };
        }
        result
    }
    /// Construct Ini from file
    ///
    /// # Errors
    /// Errors returned by `File::open()` and `BufReader::read_to_string()`
    ///
    ///
    /// # Examples
    /// You may use Path
    ///
    /// ```
    /// use std::path::Path;
    /// use tini::Ini;
    ///
    /// let path = Path::new("./examples/example.ini");
    /// let conf = Ini::from_file(path);
    /// assert!(conf.ok().is_some());
    /// ```
    ///
    /// or `&str`
    ///
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_file("./examples/example.ini");
    /// assert!(conf.ok().is_some());
    /// ```
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Ini, io::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(Ini::from_string(&buffer))
    }
    /// Construct Ini from buffer
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }
    /// Set section name for following [`item()`](#method.item)s. This function doesn't create a
    /// section.
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::new().section("empty");
    /// assert_eq!(conf.to_buffer(), String::new());
    /// ```
    pub fn section<S: Into<String>>(mut self, name: S) -> Self {
        self.last_section_name = name.into();
        self
    }
    /// Add key-value pair to last section
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::new().section("test")
    ///                      .item("value", "10");
    ///
    /// let value: Option<u8> = conf.get("test", "value");
    /// assert_eq!(value, Some(10));
    /// ```
    pub fn item<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.data
            .entry(self.last_section_name.clone())
            .or_insert(Section::new())
            .insert(name.into(), value.into());
        self
    }
    /// Write Ini to file. This function is similar to `from_file` in use.
    /// # Errors
    /// Errors returned by `File::create()` and `BufWriter::write_all()`
    ///
    pub fn to_file<S: AsRef<Path> + ?Sized>(&self, path: &S) -> Result<(), io::Error> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(self.to_buffer().as_bytes())?;
        Ok(())
    }
    /// Write Ini to buffer
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// // you may use `conf.to_buffer()`
    /// let value: String = conf.to_buffer();
    /// // or format!("{}", conf);
    /// // let value: String = format!("{}", conf);
    /// // but the result will be the same
    /// assert_eq!(value, "[section]\none = 1".to_owned());
    /// ```
    pub fn to_buffer(&self) -> String {
        format!("{}", self)
    }

    fn get_raw(&self, section: &str, key: &str) -> Option<&String> {
        self.data.get(section).and_then(|x| x.get(key))
    }
    /// Get scalar value of key in section
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn get<T: FromStr>(&self, section: &str, key: &str) -> Option<T> {
        self.get_raw(section, key).and_then(|x| x.parse().ok())
    }
    /// Get vector value of key in section
    ///
    /// The function returns `None` if one of the elements can not be parsed.
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer("[section]\nlist = 1, 2, 3, 4");
    /// let value: Option<Vec<u8>> = conf.get_vec("section", "list");
    /// assert_eq!(value, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn get_vec<T>(&self, section: &str, key: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.get_raw(section, key).and_then(|x| {
            x.split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<T>, _>>()
                .ok()
        })
    }
    /// Iterate over a section by a name
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::from_buffer(["[search]",
    ///                         "g = google.com",
    ///                         "dd = duckduckgo.com"].join("\n"));
    /// let search = conf.iter_section("search").unwrap();
    /// for (k, v) in search {
    ///   println!("key: {} value: {}", k, v);
    /// }
    /// ```
    pub fn iter_section(&self, section: &str) -> Option<SectionIter> {
        self.data.get(section).map(|value| value.iter())
    }
    /// Iterate over all sections, yielding pairs of section name and iterator
    /// over the section elements. The concrete iterator element type is
    /// `(&'a String, std::collections::hash_map::Iter<'a, String, String>)`.
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let conf = Ini::new().section("foo")
    ///                      .item("item", "value")
    ///                      .item("other", "something")
    ///                      .section("bar")
    ///                      .item("one", "1");
    /// for (section, iter) in conf.iter() {
    ///   for (key, val) in iter {
    ///     println!("section: {} key: {} val: {}", section, key, val);
    ///   }
    /// }
    pub fn iter(&self) -> IniIter {
        IniIter {
            iter: self.data.iter(),
        }
    }

    /// Iterate over all sections, yielding pairs of section name and mutable
    /// iterator over the section elements. The concrete iterator element type is
    /// `(&'a String, std::collections::hash_map::IterMut<'a, String, String>)`.
    ///
    /// # Example
    /// ```
    /// use tini::Ini;
    ///
    /// let mut conf = Ini::new().section("foo")
    ///                          .item("item", "value")
    ///                          .item("other", "something")
    ///                          .section("bar")
    ///                          .item("one", "1");
    /// for (section, iter_mut) in conf.iter_mut() {
    ///   for (key, val) in iter_mut {
    ///     *val = String::from("replaced");
    ///   }
    /// }
    pub fn iter_mut(&mut self) -> IniIterMut {
        IniIterMut {
            iter: self.data.iter_mut(),
        }
    }
}

impl fmt::Display for Ini {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for (section, iter) in self.iter() {
            buffer.push_str(&format!("[{}]\n", section));
            for (key, value) in iter {
                buffer.push_str(&format!("{} = {}\n", key, value));
            }
        }
        // remove last '\n'
        buffer.pop();
        write!(f, "{}", buffer)
    }
}

#[doc(hidden)]
pub struct IniIter<'a> {
    iter: btree_map::Iter<'a, String, Section>,
}

impl<'a> Iterator for IniIter<'a> {
    type Item = (&'a String, SectionIter<'a>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(string, section)| (string, section.iter()))
    }
}

#[doc(hidden)]
pub struct IniIterMut<'a> {
    iter: btree_map::IterMut<'a, String, Section>,
}

impl<'a> Iterator for IniIterMut<'a> {
    type Item = (&'a String, SectionIterMut<'a>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(string, section)| (string, section.iter_mut()))
    }
}

#[cfg(test)]
mod library_test {
    use super::*;

    #[test]
    fn test_bool() {
        let ini = Ini::from_buffer("[string]\nabc = true");
        let abc: Option<bool> = ini.get("string", "abc");
        assert_eq!(abc, Some(true));
    }

    #[test]
    fn test_float() {
        let ini = Ini::from_string("[section]\nname=10.5");
        let name: Option<f64> = ini.get("section", "name");
        assert_eq!(name, Some(10.5));
    }

    #[test]
    fn test_float_vec() {
        let ini = Ini::from_string("[section]\nname=1.2, 3.4, 5.6");
        let name: Option<Vec<f64>> = ini.get_vec("section", "name");
        assert_eq!(name, Some(vec![1.2, 3.4, 5.6]));
    }

    #[test]
    fn test_string_vec() {
        let ini = Ini::from_string("[section]\nname=a, b, c");
        let name: Option<Vec<String>> = ini.get_vec("section", "name");
        assert_eq!(
            name,
            Some(vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
            ])
        );
    }

    #[test]
    fn test_parse_error() {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4");
        let name: Option<Vec<u8>> = ini.get_vec("section", "list");
        assert_eq!(name, None);
    }

    #[test]
    fn test_get_or_macro() {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4");
        let with_value: Vec<u8> = ini.get_vec("section", "list").unwrap_or(vec![1, 2, 3, 4]);
        assert_eq!(with_value, vec![1, 2, 3, 4]);
    }
}

mod parser {
    #[derive(Debug)]
    pub enum Parsed {
        Error(String),
        Empty,
        Section(String),
        Value(String, String), /* Vector(String, Vec<String>), impossible, because HashMap field has type String, not Vec */
    }

    pub fn parse_line(line: &str) -> Parsed {
        let content = match line.split(';').next() {
            Some(value) => value.trim(),
            None => return Parsed::Empty
        };
        if content.len() == 0 {
            return Parsed::Empty;
        }
        // add checks for content
        if content.starts_with('[') {
            if content.ends_with(']') {
                let section_name = content.trim_matches(|c| c == '[' || c == ']').to_owned();
                return Parsed::Section(section_name);
            } else {
                return Parsed::Error("incorrect section syntax".to_owned());
            }
        } else if content.contains('=') {
            let mut pair = content.splitn(2, '=').map(|s| s.trim());
            // if key is None => error
            let key = match pair.next() {
                Some(value) => value.to_owned(),
                None => return Parsed::Error("key is None".to_owned()),
            };
            // if value is None => empty string
            let value = match pair.next() {
                Some(value) => value.to_owned(),
                None => "".to_owned(),
            };
            if key.len() == 0 {
                return Parsed::Error("empty key".to_owned());
            }
            return Parsed::Value(key, value);
        }
        Parsed::Error("incorrect syntax".to_owned())
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
}
