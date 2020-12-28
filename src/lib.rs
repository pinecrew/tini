//! _**tini** is a **t**iny **ini**-file parsing library_
//!
//! This small library provides basic functions to operate with ini-files.
//!
//! Features:
//!
//! * no dependencies;
//! * parsing [from file](Ini::from_file), [from reader](Ini::from_reader) and [from buffer](Ini::from_buffer);
//! * [convert parsed value to given type](Ini::get);
//! * [parse comma-separated lists to vectors](Ini::get_vec);
//! * construct new ini-structure with [method chaining](Ini::item);
//! * writing [to file](Ini::to_file), [to writer](Ini::to_writer) and [to buffer](Ini::to_buffer).
//!
//! # Examples
//! ## Read from buffer and get string values
//! ````
//! # use tini::Ini;
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
//! # use tini::Ini;
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
mod ordered_hashmap;
mod parser;

use ordered_hashmap::OrderedHashMap;
use parser::{parse_line, Parsed};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::iter::Iterator;
use std::path::Path;
use std::str::FromStr;

type Section = OrderedHashMap<String, String>;
type IniParsed = OrderedHashMap<String, Section>;
type SectionIter<'a> = ordered_hashmap::Iter<'a, String, String>;
type SectionIterMut<'a> = ordered_hashmap::IterMut<'a, String, String>;

/// Structure for INI-file data
#[derive(Debug)]
pub struct Ini {
    #[doc(hidden)]
    data: IniParsed,
    last_section_name: String,
}

impl Ini {
    /// Create an empty Ini
    pub fn new() -> Ini {
        Ini { data: IniParsed::new(), last_section_name: String::new() }
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
    /// Errors returned by [File::open](File::open) and [BufReader::read_to_string](BufReader::read_to_string)
    ///
    ///
    /// # Examples
    /// You may use Path
    ///
    /// ```
    /// # use std::path::Path;
    /// # use tini::Ini;
    /// let path = Path::new("./examples/example.ini");
    /// let conf = Ini::from_file(path);
    /// assert!(conf.ok().is_some());
    /// ```
    ///
    /// or `&str`
    ///
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_file("./examples/example.ini");
    /// assert!(conf.ok().is_some());
    /// ```
    pub fn from_file<S: AsRef<Path> + ?Sized>(path: &S) -> Result<Ini, io::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Ini::from_reader(&mut reader)
    }

    /// Construct Ini from any struct who implement [Read](std::io::Read) trait
    ///
    /// # Errors
    /// Errors returned by [Read::read_to_string](Read::read_to_string)
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io::BufReader;
    /// # use std::fs::File;
    /// # use tini::Ini;
    /// let f = File::open("./examples/example.ini").unwrap();
    /// let mut reader = BufReader::new(f);
    /// let conf = Ini::from_reader(&mut reader);
    /// assert!(conf.ok().is_some());
    /// ```
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Ini, io::Error> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ok(Ini::from_string(&buffer))
    }

    /// Construct Ini from buffer
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn from_buffer<S: Into<String>>(buf: S) -> Ini {
        Ini::from_string(&buf.into())
    }

    /// Set section name for following [`item()`](Ini::item)s. This function doesn't create a
    /// section.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
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
    /// # use tini::Ini;
    /// let conf = Ini::new().section("test")
    ///                      .item("value", "10");
    ///
    /// let value: Option<u8> = conf.get("test", "value");
    /// assert_eq!(value, Some(10));
    /// ```
    pub fn item<S: Into<String>>(mut self, name: S, value: S) -> Self {
        self.data.entry(self.last_section_name.clone()).or_insert_with(Section::new).insert(name.into(), value.into());
        self
    }

    /// Add key-vector pair to last section separated by sep string
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::new()
    ///     .section("default")
    ///     .item_vec_with_sep("a", &[1, 2, 3, 4], ",")
    ///     .item_vec_with_sep("b", &vec!["a", "b", "c"], "|");
    /// let va: Option<Vec<u8>> = conf.get_vec("default", "a");
    /// let vb: Vec<String> = conf.get_vec_with_sep("default", "b", "|").unwrap();
    /// assert_eq!(va, Some(vec![1, 2, 3, 4]));
    /// assert_eq!(vb, ["a", "b", "c"]);
    /// ```
    pub fn item_vec_with_sep<S, V>(mut self, name: S, vector: &[V], sep: &str) -> Self
    where
        S: Into<String>,
        V: fmt::Display,
    {
        let vector_data = vector.iter().map(|v| format!("{}", v)).collect::<Vec<_>>().join(sep);
        self.data.entry(self.last_section_name.clone()).or_insert_with(Section::new).insert(name.into(), vector_data);
        self
    }

    /// Add key-vector pair to last section
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::new()
    ///     .section("default")
    ///     .item_vec("a", &[1, 2, 3, 4])
    ///     .item_vec("b", &vec!["a", "b", "c"]);
    /// let va: Option<Vec<u8>> = conf.get_vec("default", "a");
    /// let vb: Vec<String> = conf.get_vec("default", "b").unwrap();
    /// assert_eq!(va, Some(vec![1, 2, 3, 4]));
    /// assert_eq!(vb, ["a", "b", "c"]);
    /// ```
    pub fn item_vec<S, V>(self, name: S, vector: &[V]) -> Self
    where
        S: Into<String>,
        V: fmt::Display,
    {
        self.item_vec_with_sep(name, vector, ", ")
    }

    /// Write Ini to file. This function is similar to [from_file](Ini::from_file) in use.
    /// # Errors
    /// Errors returned by [File::create](File::create) and [Write::write_all](Write::write_all)
    ///
    pub fn to_file<S: AsRef<Path> + ?Sized>(&self, path: &S) -> Result<(), io::Error> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_writer(&mut writer)
    }

    /// Writer Ini to any struct who implement Write trait
    /// # Errors
    /// Errors returned by [Write::write_all](Write::write_all)
    ///
    pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write_all(self.to_buffer().as_bytes())?;
        Ok(())
    }

    /// Write Ini to buffer
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// // you may use `conf.to_buffer()`
    /// let value: String = conf.to_buffer();
    /// // or format!("{}", conf);
    /// // let value: String = format!("{}", conf);
    /// // but the result will be the same
    /// assert_eq!(value, "[section]\none = 1");
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
    /// # use tini::Ini;
    /// let conf = Ini::from_buffer("[section]\none = 1");
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn get<T: FromStr>(&self, section: &str, key: &str) -> Option<T> {
        self.get_raw(section, key).and_then(|x| x.parse().ok())
    }

    /// Get vector value of key in section
    ///
    /// The function returns [None](Option::None) if one of the elements can not be parsed.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_buffer("[section]\nlist = 1, 2, 3, 4");
    /// let value: Option<Vec<u8>> = conf.get_vec("section", "list");
    /// assert_eq!(value, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn get_vec<T>(&self, section: &str, key: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.get_vec_with_sep(section, key, ",")
    }

    /// Get vector value of key in section separeted by sep string
    ///
    /// The function returns [None](Option::None) if one of the elements can not be parsed.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_buffer("[section]\nlist = 1|2|3|4");
    /// let value: Option<Vec<u8>> = conf.get_vec_with_sep("section", "list", "|");
    /// assert_eq!(value, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn get_vec_with_sep<T>(&self, section: &str, key: &str, sep: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.get_raw(section, key)
            .and_then(|x| x.split(sep).map(|s| s.trim().parse()).collect::<Result<Vec<T>, _>>().ok())
    }

    /// Remove section from Ini
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut config = Ini::from_buffer("[one]\na = 1\n[two]\nb = 2");
    /// let section = config.remove_section("one").unwrap();
    /// assert_eq!(section.get("a"), Some(&"1".to_string()));
    /// assert_eq!(config.get::<u8>("one", "a"), None);
    /// assert_eq!(config.get::<u8>("two", "b"), Some(2));
    /// ```
    pub fn remove_section<S: Into<String>>(&mut self, section: S) -> Option<Section> {
        let section = section.into();
        self.data.remove(&section)
    }

    /// Remove item from section
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut config = Ini::from_buffer("[one]\na = 1\nb = 2");
    /// let item = config.remove_item("one", "b");
    /// assert_eq!(item, Some("2".to_string()));
    /// assert_eq!(config.get::<u8>("one", "a"), Some(1));
    /// assert_eq!(config.get::<u8>("one", "b"), None);
    /// ```
    pub fn remove_item<K: Into<String>>(&mut self, section: K, key: K) -> Option<String> {
        let section = section.into();
        let key = key.into();
        if let Some(sec) = self.data.get_mut(&section) {
            sec.remove(&key)
        } else {
            None
        }
    }

    /// Iterate over a section by a name
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
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
    /// `(&'a String, ordered_hashmap::Iter<'a, String, String>)`.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
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
        IniIter { iter: self.data.iter() }
    }

    /// Iterate over all sections, yielding pairs of section name and mutable
    /// iterator over the section elements. The concrete iterator element type is
    /// `(&'a String, ordered_hashmap::IterMut<'a, String, String>)`.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
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
        IniIterMut { iter: self.data.iter_mut() }
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
            // blank line between sections
            buffer.push_str("\n");
        }
        // remove last two '\n'
        buffer.pop();
        buffer.pop();
        write!(f, "{}", buffer)
    }
}

impl Default for Ini {
    fn default() -> Self {
        Self::new()
    }
}

#[doc(hidden)]
pub struct IniIter<'a> {
    iter: ordered_hashmap::Iter<'a, String, Section>,
}

impl<'a> Iterator for IniIter<'a> {
    type Item = (&'a String, SectionIter<'a>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(string, section)| (string, section.iter()))
    }
}

#[doc(hidden)]
pub struct IniIterMut<'a> {
    iter: ordered_hashmap::IterMut<'a, String, Section>,
}

impl<'a> Iterator for IniIterMut<'a> {
    type Item = (&'a String, SectionIterMut<'a>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(string, section)| (string, section.iter_mut()))
    }
}

#[cfg(test)]
mod library_test {
    use super::*;

    #[test]
    fn bool() {
        let ini = Ini::from_buffer("[string]\nabc = true");
        let abc: Option<bool> = ini.get("string", "abc");
        assert_eq!(abc, Some(true));
    }

    #[test]
    fn float() {
        let ini = Ini::from_string("[section]\nname=10.5");
        let name: Option<f64> = ini.get("section", "name");
        assert_eq!(name, Some(10.5));
    }

    #[test]
    fn float_vec() {
        let ini = Ini::from_string("[section]\nname=1.2, 3.4, 5.6");
        let name: Option<Vec<f64>> = ini.get_vec("section", "name");
        assert_eq!(name, Some(vec![1.2, 3.4, 5.6]));
    }

    #[test]
    fn bad_cast() {
        let ini = Ini::new().section("one").item("a", "3.14");
        let a: Option<u32> = ini.get("one", "a");
        assert_eq!(a, None);
    }

    #[test]
    fn string_vec() {
        let ini = Ini::from_string("[section]\nname=a, b, c");
        let name: Vec<String> = ini.get_vec("section", "name").unwrap_or(vec![]);
        assert_eq!(name, ["a", "b", "c"]);
    }

    #[test]
    fn parse_error() {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4");
        let name: Option<Vec<u8>> = ini.get_vec("section", "list");
        assert_eq!(name, None);
    }

    #[test]
    fn get_or_macro() {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4");
        let with_value: Vec<u8> = ini.get_vec("section", "list").unwrap_or(vec![1, 2, 3, 4]);
        assert_eq!(with_value, [1, 2, 3, 4]);
    }

    #[test]
    fn ordering_iter() {
        let ini = Ini::from_string("[a]\nc = 1\nb = 2\na = 3");
        let keys: Vec<&String> = ini.data.get("a").unwrap().iter().map(|(k, _)| k).collect();
        assert_eq!(["c", "b", "a"], keys[..]);
    }

    #[test]
    fn ordering_keys() {
        let ini = Ini::from_string("[a]\nc = 1\nb = 2\na = 3");
        let keys: Vec<&String> = ini.data.get("a").unwrap().keys().collect();
        assert_eq!(["c", "b", "a"], keys[..]);
    }

    #[test]
    fn mutating() {
        let mut config = Ini::new().section("items").item("a", "1").item("b", "2").item("c", "3");

        // mutate items
        for (_, item) in config.iter_mut() {
            for (_, value) in item {
                let v: i32 = value.parse().unwrap();
                *value = format!("{}", v + 1);
            }
        }

        let a_val: Option<u8> = config.get("items", "a");
        let b_val: Option<u8> = config.get("items", "b");
        let c_val: Option<u8> = config.get("items", "c");

        assert_eq!(a_val, Some(2));
        assert_eq!(b_val, Some(3));
        assert_eq!(c_val, Some(4));
    }

    #[test]
    fn redefine_item() {
        let config = Ini::new().section("items").item("one", "3").item("two", "2").item("one", "1");

        let one: Option<i32> = config.get("items", "one");
        assert_eq!(one, Some(1));
    }

    #[test]
    fn redefine_section() {
        let config =
            Ini::new().section("one").item("a", "1").section("two").item("b", "2").section("one").item("c", "3");

        let a_val: Option<i32> = config.get("one", "a");
        let c_val: Option<i32> = config.get("one", "c");

        assert_eq!(a_val, Some(1));
        assert_eq!(c_val, Some(3));
    }

    #[test]
    fn with_escaped_items() {
        let config = Ini::new().section("default").item("vector", r"1, 2, 3, 4, 5, 6, 7");

        let vector: Vec<String> = config.get_vec("default", "vector").unwrap();
        assert_eq!(vector, ["1", "2", "3", "4", "5", "6", "7"]);
    }

    #[test]
    fn use_item_vec() {
        let config = Ini::new().section("default").item_vec_with_sep("a", &["a,b", "c,d", "e"], "|");

        let v: Vec<String> = config.get_vec_with_sep("default", "a", "|").unwrap();
        assert_eq!(v, [r"a,b", "c,d", "e"]);
    }

    #[test]
    fn remove_section() {
        let mut config = Ini::new().section("one").item("a", "1").section("two").item("b", "2");
        let section = match config.remove_section("one") {
            Some(value) => value,
            None => panic!("section not found"),
        };

        assert_eq!(section.get("a"), Some(&"1".to_string()));
        assert_eq!(config.get::<u8>("one", "a"), None);
        assert_eq!(config.get::<u8>("two", "b"), Some(2));
    }

    #[test]
    fn remove_item() {
        let mut config = Ini::new().section("one").item("a", "1").item("b", "2");
        let item = config.remove_item("one", "a");

        assert_eq!(item, Some("1".to_string()));
        assert_eq!(config.get::<u8>("one", "a"), None);
        assert_eq!(config.get::<u8>("one", "b"), Some(2));
    }
}
