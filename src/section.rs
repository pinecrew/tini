use crate::ordered_hashmap::OrderedHashMap;
use std::fmt;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Section {
    #[doc(hidden)]
    base: OrderedHashMap<String, String>,
}

impl Section {
    pub fn new() -> Section {
        Section { base: OrderedHashMap::new() }
    }

    pub fn iter(&self) -> Iter {
        return Iter { iter: self.base.iter() };
    }

    pub fn iter_mut(&mut self) -> IterMut {
        return IterMut { iter: self.base.iter_mut() };
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.base.get(key)
    }

    pub fn keys(&self) -> Keys {
        Keys { iter: self.base.keys() }
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.base.insert(key, value)
    }

    pub fn remove(&mut self, key: &String) -> Option<String> {
        self.base.remove(key)
    }
}

impl IntoIterator for Section {
    type Item = (String, String);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { iter: self.base.into_iter() }
    }
}

impl<K, V> FromIterator<(K, V)> for Section
where
    K: fmt::Display + Eq + Hash,
    V: fmt::Display,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut s = Section::new();

        for (k, v) in iter {
            s.insert(k.to_string(), v.to_string());
        }

        s
    }
}

pub struct Keys<'a> {
    #[doc(hidden)]
    iter: std::slice::Iter<'a, String>,
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct Iter<'a> {
    #[doc(hidden)]
    iter: crate::ordered_hashmap::Iter<'a, String, String>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a String, &'a String);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct IterMut<'a> {
    #[doc(hidden)]
    iter: crate::ordered_hashmap::IterMut<'a, String, String>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a String, &'a mut String);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct IntoIter {
    #[doc(hidden)]
    iter: crate::ordered_hashmap::IntoIter<String, String>,
}

impl Iterator for IntoIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
