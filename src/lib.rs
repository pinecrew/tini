//! _**tini** is a **t**iny **ini**-file parsing library_
//!
//! This small library provides basic functions to operate with ini-files.
//!
//! Features:
//!
//! * no dependencies;
//! * parsing [from file](Ini::from_file), [from reader](Ini::from_reader) and [from string](Ini::from_string);
//! * [convert parsed value to given type](Ini::get);
//! * [parse comma-separated lists to vectors](Ini::get_vec);
//! * construct new ini-structure with [method chaining](Ini::item);
//! * writing [to file](Ini::to_file), [to writer](Ini::to_writer) and [to string](Ini#impl-Display).
//!
//! # Examples
//! ## Read from buffer and get string values
//! ````
//! # use tini::Ini;
//! let conf = Ini::from_string(["[search]",
//!                              "g = google.com",
//!                              "dd = duckduckgo.com"].join("\n")).unwrap();
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
//!                      .item_vec("consts", &[3.1416, 2.7183])
//!                      .section("integers")
//!                      .item_vec("lost", &[4, 8, 15, 16, 23, 42]);
//!
//! let consts: Vec<f64> = conf.get_vec("floats", "consts").unwrap();
//! let lost: Vec<i32> = conf.get_vec("integers", "lost").unwrap();
//!
//! assert_eq!(consts, [3.1416, 2.7183]);
//! assert_eq!(lost, [4, 8, 15, 16, 23, 42]);
//! ````
mod error;
mod ordered_hashmap;
mod parser;

pub use error::{Error, ParseError};
use ordered_hashmap::OrderedHashMap;
use parser::{parse_line, Parsed};
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::iter::Iterator;
use std::path::Path;
use std::str::FromStr;

/// Structure for INI-file data
#[derive(Debug)]
pub struct Ini {
    #[doc(hidden)]
    document: OrderedHashMap<String, Section>,
    last_section_name: String,
    empty_section: Section,
}

impl Ini {
    /// Create an empty Ini (similar to [Ini::default])
    pub fn new() -> Ini {
        Ini { document: OrderedHashMap::new(), last_section_name: String::new(), empty_section: Section::new() }
    }

    /// Private construct method which creaate [Ini] struct from input string
    fn parse(string: &str) -> Result<Ini, Error> {
        let mut result = Ini::new();
        for (index, line) in string.lines().enumerate() {
            match parse_line(&line, index + 1)? {
                Parsed::Section(name) => result = result.section(name),
                Parsed::Value(name, value) => result = result.item(name, value),
                _ => (),
            };
        }
        Ok(result)
    }

    /// Construct Ini from file
    ///
    /// # Errors
    /// This function will return an [Error] if file cannot be opened or parsed
    ///
    /// # Examples
    /// You may use [Path]
    ///
    /// ```no_run
    /// # use std::path::Path;
    /// # use tini::Ini;
    /// let path = Path::new("example.ini");
    ///
    /// let conf = Ini::from_file(path);
    ///
    /// assert!(conf.ok().is_some());
    /// ```
    ///
    /// or `&str`
    ///
    /// ```no_run
    /// # use tini::Ini;
    /// let conf = Ini::from_file("example.ini");
    ///
    /// assert!(conf.ok().is_some());
    /// ```
    pub fn from_file<S>(path: &S) -> Result<Ini, Error>
    where
        S: AsRef<Path> + ?Sized,
    {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Ini::from_reader(&mut reader)
    }

    /// Construct Ini from any struct who implement [Read](std::io::Read) trait
    ///
    /// # Errors
    /// This function will return an [Error] if reader cannot be read or parsed
    ///
    /// # Example
    ///
    /// ```
    /// # use std::io::BufReader;
    /// # use tini::Ini;
    /// let f = "[section]\nitem=value".as_bytes();
    /// let mut reader = BufReader::new(f);
    ///
    /// let conf = Ini::from_reader(&mut reader);
    ///
    /// assert!(conf.ok().is_some());
    /// ```
    pub fn from_reader<R>(reader: &mut R) -> Result<Ini, Error>
    where
        R: Read,
    {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Ini::parse(&buffer)
    }

    /// Construct Ini from any type of string which can be [Into]ed to String
    ///
    /// # Errors
    /// This function will return an [Error] if buffer cannot be parsed
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_string("[section]\none = 1").unwrap();
    ///
    /// let value: Option<u8> = conf.get("section", "one");
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn from_string<S>(buf: S) -> Result<Ini, Error>
    where
        S: Into<String>,
    {
        Ini::parse(&buf.into())
    }

    /// Write Ini to file. This function is similar to [from_file](Ini::from_file) in use.
    ///
    /// # Errors
    /// Errors returned by [File::create] and [Write::write_all]
    pub fn to_file<S>(&self, path: &S) -> Result<(), io::Error>
    where
        S: AsRef<Path> + ?Sized,
    {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        self.to_writer(&mut writer)
    }

    /// Write [Ini] to any struct who implement [Write] trait.
    ///
    /// # Errors
    /// Errors returned by [Write::write_all](Write::write_all)
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::default().section("a").item("a", 1);
    ///
    /// // create output Vec<u8> buffer
    /// let mut output = Vec::new();
    /// // let's write data to Vec<u8>
    /// conf.to_writer(&mut output);
    ///
    /// // cast Vec<u8> to utf-8 string
    /// let casted_result = String::from_utf8(output).unwrap();
    /// assert_eq!(casted_result, "[a]\na = 1\n")
    /// ```
    pub fn to_writer<W>(&self, writer: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        writer.write_all(self.to_string().as_bytes())?;
        Ok(())
    }

    /// Set section name for the following methods in chain ([`item()`](Ini::item), [`items()`](Ini::items), etc.)
    ///
    /// # Warning
    /// This function doesn't create a section.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut conf = Ini::new().section("empty");
    /// assert_eq!(conf.to_string(), "");
    ///
    /// // but section will be created on item() call
    /// conf = conf.section("one").item("a", 1);
    /// assert_eq!(conf.to_string(), "[one]\na = 1\n");
    /// ```
    pub fn section<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.last_section_name = name.into();
        self
    }

    /// Add key-value pair to the end of section, specified in last [`section()`](Ini::section) call,
    /// or replace value if key already in section
    ///
    /// - `name` must support [Into] to [String]
    /// - `value` must support [Display](fmt::Display) to support conversion to [String]
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut conf = Ini::new().section("test")
    ///                      .item("value", 10);
    ///
    /// assert_eq!(conf.to_string(), "[test]\nvalue = 10\n");
    ///
    /// // change existing value
    /// conf = conf.section("test").item("value", "updated");
    /// assert_eq!(conf.to_string(), "[test]\nvalue = updated\n");
    /// ```
    pub fn item<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<String>,
        V: fmt::Display,
    {
        self.document
            .entry(self.last_section_name.clone())
            .or_insert_with(Section::new)
            .insert(name.into(), value.to_string());
        self
    }

    /// Like [`item()`](Ini::item), but for vectors
    ///
    /// - `name` must support [Into] to [String]
    /// - `vector` elements must support [Display](fmt::Display) to support conversion to [String]
    /// - `sep` arbitrary string delimiter
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::new()
    ///     .section("default")
    /// // add a vector with `,` separator: 1,2,3,4
    ///     .item_vec_with_sep("a", &[1, 2, 3, 4], ",")
    /// // add a vector with `|` separator: a|b|c
    ///     .item_vec_with_sep("b", &vec!["a", "b", "c"], "|");
    ///
    /// let va: Option<Vec<u8>> = conf.get_vec("default", "a");
    /// let vb: Vec<String> = conf.get_vec_with_sep("default", "b", "|").unwrap();
    ///
    /// assert_eq!(va, Some(vec![1, 2, 3, 4]));
    /// assert_eq!(vb, ["a", "b", "c"]);
    /// ```
    pub fn item_vec_with_sep<S, V>(mut self, name: S, vector: &[V], sep: &str) -> Self
    where
        S: Into<String>,
        V: fmt::Display,
    {
        let vector_data = vector.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(sep);
        self.document
            .entry(self.last_section_name.clone())
            .or_insert_with(Section::new)
            .insert(name.into(), vector_data);
        self
    }

    /// Equivalent of [`item_vec_with_sep(name, vector, ", ")`](Ini::item_vec_with_sep)
    ///
    /// - `name` must support [Into] to [String]
    /// - `vector` elements must support [Display](fmt::Display) to support conversion to [String]
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::new()
    ///     .section("default")
    /// // add vector with default separator `, `
    ///     .item_vec("a", &[1, 2, 3, 4])
    /// // and another vector
    ///     .item_vec("b", &vec!["a", "b", "c"]);
    ///
    /// let va: Option<Vec<u8>> = conf.get_vec("default", "a");
    /// let vb: Vec<String> = conf.get_vec("default", "b").unwrap();
    ///
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

    /// Append pairs from any object supporting [IntoIterator] to the section, specified in last [`section()`](Ini::section) call.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// use std::collections::HashMap;
    ///
    /// let mut conf = Ini::new()
    ///                .section("colors")
    ///                .items(vec![("black", "#000000"),
    ///                            ("white", "#ffffff")]);
    ///
    /// // create custom section
    /// let mut numbers = HashMap::new();
    /// numbers.insert("round_pi", 3);
    /// // and add to `conf`
    /// conf = conf.section("numbers").items(numbers);
    ///
    /// assert_eq!(conf.to_string(), [
    ///                               "[colors]",
    ///                               "black = #000000",
    ///                               "white = #ffffff",
    ///                               "",
    ///                               "[numbers]",
    ///                               "round_pi = 3",
    ///                               ""
    ///                              ].join("\n"));
    /// ```
    pub fn items<K, V, I>(mut self, items: I) -> Self
    where
        K: Into<String>,
        V: fmt::Display,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in items {
            self = self.item(k.into(), v.to_string());
        }
        self
    }

    /// Remove section from [Ini].
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut config = Ini::from_string([
    ///                                    "[one]",
    ///                                    "a = 1",
    ///                                    "[two]",
    ///                                    "b = 2"
    ///                                   ].join("\n")).unwrap();
    /// // remove section
    /// config = config.section("one").clear();
    /// assert_eq!(config.to_string(), "[two]\nb = 2\n");
    ///
    /// // clear section from old data and add new
    /// config = config.section("two").clear().item("a", 1);
    /// assert_eq!(config.to_string(), "[two]\na = 1\n");
    /// ```
    pub fn clear(mut self) -> Self {
        self.document.remove(&self.last_section_name);
        self
    }

    /// Remove item from section.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut config = Ini::from_string([
    ///                                    "[one]",
    ///                                    "a = 1",
    ///                                    "b = 2"
    ///                                   ].join("\n")).unwrap();
    ///
    /// config = config.section("one").erase("b");
    ///
    /// assert_eq!(config.to_string(), "[one]\na = 1\n");
    /// ```
    pub fn erase(mut self, key: &str) -> Self {
        self.document.get_mut(&self.last_section_name).and_then(|s| s.remove(key));
        self
    }

    /// Private method which get value by `key` from `section`
    fn get_raw(&self, section: &str, key: &str) -> Option<&String> {
        self.document.get(section).and_then(|s| s.get(key))
    }

    /// Get scalar value of key in section.
    ///
    /// - output type `T` must implement [FromStr] trait for auto conversion
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_string("[section]\none = 1").unwrap();
    ///
    /// let value: Option<u8> = conf.get("section", "one");
    ///
    /// assert_eq!(value, Some(1));
    /// ```
    pub fn get<T>(&self, section: &str, key: &str) -> Option<T>
    where
        T: FromStr,
    {
        self.get_raw(section, key).and_then(|x| x.parse().ok())
    }

    /// Get vector value of `key` in `section`. Value should use `,` as separator.
    ///
    /// The function returns [None](Option::None) if one of the elements can not be parsed.
    ///
    /// - output type `T` must implement [FromStr] trait for auto conversion
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_string("[section]\nlist = 1, 2, 3, 4").unwrap();
    ///
    /// let value: Option<Vec<u8>> = conf.get_vec("section", "list");
    ///
    /// assert_eq!(value, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn get_vec<T>(&self, section: &str, key: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.get_vec_with_sep(section, key, ",")
    }

    /// Get vector value of `key` in `section` separated by `sep` string.
    ///
    /// The function returns [None](Option::None) if one of the elements can not be parsed or not found.
    ///
    /// - output type `T` must implement [FromStr] trait for auto conversion
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_string("[section]\nlist = 1|2|3|4").unwrap();
    ///
    /// let value: Option<Vec<u8>> = conf.get_vec_with_sep("section", "list", "|");
    ///
    /// assert_eq!(value, Some(vec![1, 2, 3, 4]));
    /// ```
    pub fn get_vec_with_sep<T>(&self, section: &str, key: &str, sep: &str) -> Option<Vec<T>>
    where
        T: FromStr,
    {
        self.get_raw(section, key)
            .and_then(|x| x.split(sep).map(|s| s.trim().parse()).collect::<Result<Vec<T>, _>>().ok())
    }

    /// An iterator visiting all key-value pairs of a section in order of appearance.
    ///
    /// If section with given name doesn't exist in document, method returns empty iterator
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::from_string(["[search]",
    ///                              "g = google.com",
    ///                              "dd = duckduckgo.com"].join("\n")).unwrap();
    ///
    /// let mut search = conf.section_iter("search");
    /// assert_eq!(search.next(), Some((&"g".to_string(), &"google.com".to_string())));
    /// assert_eq!(search.next(), Some((&"dd".to_string(), &"duckduckgo.com".to_string())));
    /// assert_eq!(search.next(), None);
    ///
    /// assert_eq!(conf.section_iter("absent").count(), 0);
    /// ```
    pub fn section_iter(&self, section: &str) -> ordered_hashmap::Iter<String, String> {
        self.document.get(section).unwrap_or(&self.empty_section).iter()
    }

    /// Iterate over all sections in order of appearance, yielding pairs of section name
    /// and section instance, which can be iterated over or queried similarly to Ini instances.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let conf = Ini::new().section("foo")
    ///                      .item("item", "value")
    ///                      .item("other", "something")
    ///                      .section("bar")
    ///                      .item("one", "1");
    ///
    /// for (name, section) in conf.iter() {
    ///     match name.as_str() {
    ///         "foo" => assert_eq!(section.iter().count(), 2),
    ///         "bar" => assert_eq!(section.iter().count(), 1),
    ///         _ => assert!(false),
    ///     }
    /// }
    pub fn iter(&self) -> IniIter {
        IniIter { iter: self.document.iter() }
    }

    /// Iterate over all sections in arbitrary order, yielding pairs of section name and mutable
    /// iterator over the section elements. The concrete iterator element type is
    /// `(&'a String, SectionIterMut<'a>)`.
    ///
    /// # Example
    /// ```
    /// # use tini::Ini;
    /// let mut conf = Ini::new().section("foo")
    ///                          .item("item", "value")
    ///                          .item("other", "something")
    ///                          .section("bar")
    ///                          .item("one", "1");
    ///
    /// for (name, section) in conf.iter_mut() {
    ///     for (key, val) in section.iter_mut() {
    ///         *val = String::from("replaced");
    ///     }
    /// }
    ///
    /// for (name, section) in conf.iter() {
    ///     for (key, val) in section.iter() {
    ///         assert_eq!(val.as_str(), "replaced");
    ///     }
    /// }
    pub fn iter_mut(&mut self) -> IniIterMut {
        IniIterMut { iter: self.document.iter_mut() }
    }
}

impl fmt::Display for Ini {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut items = Vec::new();
        for (name, section) in self.iter() {
            // insert section block
            items.push(format!("[{}]", name));
            // add items
            for (key, value) in section.iter() {
                items.push(format!("{} = {}", key, value));
            }
            // and blank line between sections
            items.push("".to_string());
        }
        write!(f, "{}", items.join("\n"))
    }
}

impl Default for Ini {
    fn default() -> Self {
        Self::new()
    }
}

/// An iterator over the sections of an ini document
pub struct IniIter<'a> {
    #[doc(hidden)]
    iter: ordered_hashmap::Iter<'a, String, Section>,
}

impl<'a> Iterator for IniIter<'a> {
    type Item = (&'a String, &'a Section);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

/// A mutable iterator over the sections of an ini document
pub struct IniIterMut<'a> {
    #[doc(hidden)]
    iter: ordered_hashmap::IterMut<'a, String, Section>,
}

impl<'a> Iterator for IniIterMut<'a> {
    type Item = (&'a String, &'a mut Section);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

type Section = OrderedHashMap<String, String>;

#[cfg(test)]
mod library_test {
    use super::*;

    #[test]
    fn bool() -> Result<(), Error> {
        let ini = Ini::from_string("[string]\nabc = true")?;
        let abc: Option<bool> = ini.get("string", "abc");
        assert_eq!(abc, Some(true));
        Ok(())
    }

    #[test]
    fn float() -> Result<(), Error> {
        let ini = Ini::from_string("[section]\nname=10.5")?;
        let name: Option<f64> = ini.get("section", "name");
        assert_eq!(name, Some(10.5));
        Ok(())
    }

    #[test]
    fn float_vec() -> Result<(), Error> {
        let ini = Ini::from_string("[section]\nname=1.2, 3.4, 5.6")?;
        let name: Option<Vec<f64>> = ini.get_vec("section", "name");
        assert_eq!(name, Some(vec![1.2, 3.4, 5.6]));
        Ok(())
    }

    #[test]
    fn empty_key() {
        match Ini::from_string("[a]\nx = 1\n=2") {
            Err(Error::Parse(ParseError::EmptyKey(index))) => assert_eq!(index, 3),
            _ => assert!(false),
        }
    }

    #[test]
    fn invalid_section() {
        match Ini::from_string("[a]\nx = 1\ny = 2\n[b") {
            Err(Error::Parse(ParseError::IncorrectSection(index))) => assert_eq!(index, 4),
            _ => assert!(false),
        }
    }

    #[test]
    fn invalid_syntax() {
        match Ini::from_string("[a]\n\t- b") {
            Err(Error::Parse(ParseError::IncorrectSyntax(index))) => assert_eq!(index, 2),
            _ => assert!(false),
        }
    }

    #[test]
    fn bad_cast() {
        let ini = Ini::new().section("one").item("a", 3.14);
        let a: Option<u32> = ini.get("one", "a");
        assert_eq!(a, None);
    }

    #[test]
    fn string_vec() -> Result<(), Error> {
        let ini = Ini::from_string("[section]\nname=a, b, c")?;
        let name: Vec<String> = ini.get_vec("section", "name").unwrap_or(vec![]);
        assert_eq!(name, ["a", "b", "c"]);
        Ok(())
    }

    #[test]
    fn parse_error() -> Result<(), Error> {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4")?;
        let name: Option<Vec<u8>> = ini.get_vec("section", "list");
        assert_eq!(name, None);
        Ok(())
    }

    #[test]
    fn get_or_macro() -> Result<(), Error> {
        let ini = Ini::from_string("[section]\nlist = 1, 2, --, 4")?;
        let with_value: Vec<u8> = ini.get_vec("section", "list").unwrap_or(vec![1, 2, 3, 4]);
        assert_eq!(with_value, [1, 2, 3, 4]);
        Ok(())
    }

    #[test]
    fn ordering_iter() -> Result<(), Error> {
        let ini = Ini::from_string("[a]\nc = 1\nb = 2\na = 3")?;
        let keys: Vec<&String> = ini.document.get("a").unwrap().iter().map(|(k, _)| k).collect();
        assert_eq!(["c", "b", "a"], keys[..]);
        Ok(())
    }

    #[test]
    fn random_access_section() -> Result<(), Error> {
        let ini = Ini::from_string("[a]\nc = 1\nb = 2\na = 3\n\n[b]\na = 1\nb = 2\nc = 3\n\n[c]\na=0\nd=4")?;
        for (name, section) in ini.iter() {
            match name.as_str() {
                "a" => {
                    assert_eq!(section.get("a"), Some(&"3".to_owned()));
                    assert_eq!(section.get("d"), None);
                },
                "b" => {
                    assert_eq!(section.get("a"), Some(&"1".to_owned()));
                    assert_eq!(section.get("d"), None);
                },
                "c" => {
                    assert_eq!(section.get("a"), Some(&"0".to_owned()));
                    assert_eq!(section.get("b"), None);
                    assert_eq!(section.get("d"), Some(&"4".to_owned()));
                },
                _ => unreachable!()
            }
        }
        Ok(())
    }
}
